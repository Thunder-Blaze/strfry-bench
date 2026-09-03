use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tokio_tungstenite::connect_async;
use url::Url;

pub struct RelaySupervisor {
    binary_path: PathBuf,
    db_dir: PathBuf,
    config_path: Option<PathBuf>,
    child: Option<Child>,
}

impl RelaySupervisor {
    pub fn new(binary_path: impl Into<PathBuf>, db_dir: impl Into<PathBuf>) -> Self {
        Self {
            binary_path: binary_path.into(),
            db_dir: db_dir.into(),
            config_path: None,
            child: None,
        }
    }

    pub fn with_config(mut self, config_path: impl Into<PathBuf>) -> Self {
        self.config_path = Some(config_path.into());
        self
    }

    pub fn clean_db(&self) -> Result<(), String> {
        if self.db_dir.exists() {
            println!("[RELAY] Cleaning database at {}", self.db_dir.display());
            let _ = std::fs::remove_dir_all(&self.db_dir);
        }
        std::fs::create_dir_all(&self.db_dir)
            .map_err(|e| format!("Failed to create DB directory {}: {}", self.db_dir.display(), e))?;
        Ok(())
    }

    pub fn start(
        &mut self,
        config_overrides: Option<&HashMap<String, String>>,
        alloc_tracker: Option<&Path>,
        alloc_tracker_out: Option<&Path>,
        perf_record_out: Option<&Path>,
    ) -> Result<u32, String> {
        if self.child.is_some() {
            self.stop();
        }

        self.clean_db()?;

        let mut cmd = if let Some(perf_out) = perf_record_out {
            #[cfg(target_os = "linux")]
            {
                if crate::profiler::perf::is_perf_available() {
                    let mut c = Command::new("perf");
                    c.args([
                        "record",
                        "-F", "99",
                        "-g",
                        "-o", perf_out.to_str().unwrap(),
                        "--",
                    ]);
                    c.arg(&self.binary_path);
                    c
                } else {
                    Command::new(&self.binary_path)
                }
            }
            #[cfg(not(target_os = "linux"))]
            {
                let _ = perf_out;
                Command::new(&self.binary_path)
            }
        } else {
            Command::new(&self.binary_path)
        };
        if let Some(cfg) = &self.config_path
            && cfg.exists() {
                cmd.arg("--config").arg(cfg);
            }

        // Set DB path override
        let db_arg = format!("db={}/", self.db_dir.display());
        cmd.arg("--set").arg(db_arg);

        // Bind locally on 127.0.0.1 port 7777
        cmd.arg("--set").arg("relay.bind=127.0.0.1");
        cmd.arg("--set").arg("relay.port=7777");

        // Custom overrides
        if let Some(overrides) = config_overrides {
            for (k, v) in overrides {
                cmd.arg("--set").arg(format!("{}={}", k, v));
            }
        }

        // Environment variables (e.g. alloc_tracker)
        if let Some(tracker) = alloc_tracker
            && tracker.exists() {
                #[cfg(target_os = "macos")]
                {
                    cmd.env("DYLD_INSERT_LIBRARIES", tracker);
                }
                #[cfg(target_os = "linux")]
                {
                    cmd.env("LD_PRELOAD", tracker);
                }
            }

        if let Some(out_path) = alloc_tracker_out {
            cmd.env("ALLOC_TRACKER_OUT", out_path);
        }

        cmd.arg("relay");
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        println!("[RELAY] Launching relay: {:?}", cmd);
        let child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn relay at {}: {}", self.binary_path.display(), e))?;

        let pid = child.id();
        self.child = Some(child);

        println!("[RELAY] Spawned relay process with PID {}", pid);
        Ok(pid)
    }

    pub fn pid(&self) -> Option<u32> {
        self.child.as_ref().map(|c| c.id())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.child {
            matches!(child.try_wait(), Ok(None))
        } else {
            false
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            println!("[RELAY] Stopping relay process (PID {})...", child.id());

            #[cfg(unix)]
            {
                let pid = child.id() as libc::pid_t;
                unsafe {
                    libc::kill(pid, libc::SIGINT);
                }
            }

            #[cfg(not(unix))]
            {
                let _ = child.kill();
            }

            // Wait up to 5 seconds for clean exit
            let start = Instant::now();
            let mut exited = false;
            while start.elapsed() < Duration::from_secs(5) {
                if let Ok(Some(_)) = child.try_wait() {
                    exited = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }

            if !exited {
                println!("[RELAY] Relay did not exit in 5s, sending SIGKILL...");
                let _ = child.kill();
                let _ = child.wait();
            }

            println!("[RELAY] Relay stopped successfully.");
        }
    }
}

impl Drop for RelaySupervisor {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Polls WebSocket connection to ensure the relay is ready to accept queries.
pub async fn wait_for_relay_ready(ws_url: &str, timeout_secs: u64) -> Result<(), String> {
    let _ = Url::parse(ws_url).map_err(|e| format!("Invalid URL {}: {}", ws_url, e))?;
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    let mut backoff = Duration::from_millis(50);

    println!("[RELAY] Probing readiness at {} (timeout: {}s)...", ws_url, timeout_secs);

    while start.elapsed() < timeout {
        match connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                let _ = ws.close(None).await;
                println!("[RELAY] Connection verified in {:.2?}", start.elapsed());
                return Ok(());
            }
            Err(_) => {
                tokio::time::sleep(backoff).await;
                if backoff < Duration::from_millis(500) {
                    backoff *= 2;
                }
            }
        }
    }

    Err(format!(
        "Timed out after {}s waiting for relay at {} to become ready",
        timeout_secs, ws_url
    ))
}

/// Scrapes Prometheus metrics from HTTP endpoint
pub async fn scrape_prometheus(metrics_url: &str) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    let client = reqwest_or_hyper(metrics_url).await;
    if let Ok(body) = client {
        for line in body.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                if let Ok(val) = parts[1].parse::<f64>() {
                    map.insert(name, val);
                }
            }
        }
    }
    map
}

async fn reqwest_or_hyper(url_str: &str) -> Result<String, String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let parsed = Url::parse(url_str).map_err(|e| e.to_string())?;
    let host = parsed.host_str().unwrap_or("127.0.0.1");
    let port = parsed.port().unwrap_or(7777);
    let path = parsed.path();

    let mut stream = TcpStream::connect((host, port))
        .await
        .map_err(|e| format!("Failed to connect to {}:{}: {}", host, port, e))?;

    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
        path, host, port
    );
    stream
        .write_all(req.as_bytes())
        .await
        .map_err(|e| e.to_string())?;

    let mut resp = String::new();
    stream
        .read_to_string(&mut resp)
        .await
        .map_err(|e| e.to_string())?;

    // Extract body after \r\n\r\n
    if let Some(idx) = resp.find("\r\n\r\n") {
        Ok(resp[idx + 4..].to_string())
    } else {
        Ok(resp)
    }
}
