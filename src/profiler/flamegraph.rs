use std::io::BufReader;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

/// Spawns `perf record` attached to the relay process PID
pub fn start_perf_record(pid: u32, output_path: &Path) -> Option<Child> {
    #[cfg(target_os = "linux")]
    {
        if !crate::profiler::perf::is_perf_available() {
            println!("[PROFILER] perf CLI is not available, skipping flamegraph recording");
            return None;
        }

        println!("[PROFILER] Attaching perf record to PID {} -> {}", pid, output_path.display());
        let child = Command::new("perf")
            .args([
                "record",
                "-F", "99",
                "-g",
                "-p", &pid.to_string(),
                "-o", output_path.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        match child {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("[PROFILER] Failed to spawn perf record: {}", e);
                None
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (pid, output_path);
        println!("[PROFILER] CPU flamegraph recording is only supported on Linux");
        None
    }
}

/// Stops `perf record` and waits for it to write `perf.data` cleanly
pub fn stop_perf_record(mut child: Child) {
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

    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(Some(_)) = child.try_wait() {
            println!("[PROFILER] perf record completed.");
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    let _ = child.kill();
    let _ = child.wait();
}

pub fn generate_flamegraph_from_perf(perf_data_path: &Path, output_svg_path: &Path) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if !perf_data_path.exists() {
            return Err(format!("perf data not found at {}", perf_data_path.display()));
        }

        println!("[PROFILER] Converting {} to flamegraph SVG via inferno...", perf_data_path.display());
        let mut perf_cmd = Command::new("perf");
        perf_cmd.env("DEBUGINFOD_URLS", "");
        perf_cmd.args(["script", "--no-inline", "-i", perf_data_path.to_str().unwrap()]);
        let script_out = perf_cmd.output().map_err(|e| format!("Failed to run perf script: {}", e))?;
        if !script_out.status.success() {
            return Err(format!("perf script failed: {}", String::from_utf8_lossy(&script_out.stderr)));
        }

        let script_reader = BufReader::new(&script_out.stdout[..]);
        let mut collapsed = Vec::new();

        let mut folder = inferno::collapse::perf::Folder::default();
        inferno::collapse::Collapse::collapse(&mut folder, script_reader, &mut collapsed)
            .map_err(|e| format!("Failed to collapse stack traces: {}", e))?;

        let mut flamegraph_options = inferno::flamegraph::Options::default();
        flamegraph_options.title = "strfry Relay CPU Flamegraph".to_string();

        let collapsed_reader = BufReader::new(&collapsed[..]);
        let mut svg_out = std::fs::File::create(output_svg_path)
            .map_err(|e| format!("Failed to create SVG output file: {}", e))?;

        inferno::flamegraph::from_reader(&mut flamegraph_options, collapsed_reader, &mut svg_out)
            .map_err(|e| format!("Failed to render flamegraph SVG: {}", e))?;

        println!("[PROFILER] Flamegraph rendered successfully to {}", output_svg_path.display());
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (perf_data_path, output_svg_path);
        println!("[PROFILER] perf record/script CPU flamegraphs are only supported on Linux");
        Ok(())
    }
}
