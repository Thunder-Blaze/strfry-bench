use serde::{Deserialize, Serialize};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub cpu_brand: String,
    pub physical_cores: usize,
    pub total_memory_mb: u64,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(sysinfo::CpuRefreshKind::everything())
                .with_memory(sysinfo::MemoryRefreshKind::everything()),
        );
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let os_name = System::name().unwrap_or_else(|| std::env::consts::OS.to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "unknown".to_string());

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let physical_cores = sys.physical_core_count().unwrap_or_else(num_cpus);
        let total_memory_mb = sys.total_memory() / (1024 * 1024);

        Self {
            os_name,
            kernel_version,
            cpu_brand,
            physical_cores,
            total_memory_mb,
        }
    }
}

pub fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub rss_mb: f64,
    pub peak_rss_mb: f64,
    pub cpu_pct: f32,
    pub read_bytes: u64,
    pub write_bytes: u64,
}

pub struct ProcessMonitor {
    running: Arc<Mutex<bool>>,
    stats: Arc<Mutex<ProcessSnapshot>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl ProcessMonitor {
    pub fn start(pid: u32) -> Self {
        let running = Arc::new(Mutex::new(true));
        let stats = Arc::new(Mutex::new(ProcessSnapshot {
            pid,
            ..Default::default()
        }));

        let running_clone = running.clone();
        let stats_clone = stats.clone();

        let handle = std::thread::spawn(move || {
            let mut sys = System::new();
            let sys_pid = Pid::from_u32(pid);
            let mut peak_rss: f64 = 0.0;

            while *running_clone.lock() {
                sys.refresh_processes_specifics(
                    sysinfo::ProcessesToUpdate::Some(&[sys_pid]),
                    true,
                    ProcessRefreshKind::nothing()
                        .with_memory()
                        .with_cpu()
                        .with_disk_usage(),
                );

                if let Some(proc_) = sys.process(sys_pid) {
                    let rss_mb = proc_.memory() as f64 / (1024.0 * 1024.0);
                    if rss_mb > peak_rss {
                        peak_rss = rss_mb;
                    }
                    let cpu_pct = proc_.cpu_usage();
                    let disk = proc_.disk_usage();

                    let mut s = stats_clone.lock();
                    s.rss_mb = rss_mb;
                    s.peak_rss_mb = peak_rss;
                    s.cpu_pct = cpu_pct;
                    s.read_bytes = disk.total_read_bytes;
                    s.write_bytes = disk.total_written_bytes;
                }

                std::thread::sleep(Duration::from_millis(100));
            }
        });

        Self {
            running,
            stats,
            handle: Some(handle),
        }
    }

    pub fn snapshot(&self) -> ProcessSnapshot {
        self.stats.lock().clone()
    }

    pub fn stop(mut self) -> ProcessSnapshot {
        *self.running.lock() = false;
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        self.stats.lock().clone()
    }
}

/// Query current RSS memory for a process in MB
pub fn get_process_rss(pid: u32) -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string(format!("/proc/{}/status", pid)) {
            for line in content.lines() {
                if line.starts_with("VmRSS:")
                    && let Some(val) = line.split_whitespace().nth(1)
                        && let Ok(kb) = val.parse::<f64>() {
                            return kb / 1024.0;
                        }
            }
        }
    }

    // Cross-platform fallback via sysinfo
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
        true,
        ProcessRefreshKind::nothing().with_memory(),
    );
    if let Some(proc_) = sys.process(Pid::from_u32(pid)) {
        return proc_.memory() as f64 / (1024.0 * 1024.0);
    }

    0.0
}

/// Count sockets in TIME_WAIT state connected to or listening on `port`
pub fn count_time_wait_sockets(port: u16) -> usize {
    #[cfg(target_os = "linux")]
    {
        let port_hex = format!("{:04X}", port);
        let mut count = 0;
        for path in &["/proc/net/tcp", "/proc/net/tcp6"] {
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let state = parts[3];
                        if state == "06" {
                            // 06 is TCP_TIME_WAIT
                            let local_port = parts[1].split(':').next_back().unwrap_or("");
                            let remote_port = parts[2].split(':').next_back().unwrap_or("");
                            if local_port.eq_ignore_ascii_case(&port_hex)
                                || remote_port.eq_ignore_ascii_case(&port_hex)
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count
    }

    #[cfg(not(target_os = "linux"))]
    {
        // Safe cross-platform fallback
        0
    }
}
