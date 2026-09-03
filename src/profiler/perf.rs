use std::path::Path;
use std::process::Command;

pub fn run_perf_stat(cmd_and_args: &[&str], output_csv: &Path) -> Result<u64, String> {
    #[cfg(target_os = "linux")]
    {
        if !is_perf_available() {
            return Ok(0);
        }

        let mut perf_cmd = Command::new("perf");
        perf_cmd
            .args(["stat", "-x,", "-o", output_csv.to_str().unwrap()])
            .args(["-e", "instructions:u,cpu_atom/instructions/u,cpu_core/instructions/u"]);

        for arg in cmd_and_args {
            perf_cmd.arg(arg);
        }

        let out = perf_cmd.output().map_err(|e| format!("Failed to run perf: {}", e))?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        let mut instructions = 0u64;
        if let Ok(content) = std::fs::read_to_string(output_csv) {
            for line in content.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 && parts[2].contains("instructions") {
                    let val_str = parts[0].trim();
                    if val_str != "<not counted>" && !val_str.is_empty()
                        && let Ok(val) = val_str.parse::<u64>() {
                            instructions += val;
                        }
                }
            }
        }

        Ok(instructions)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (cmd_and_args, output_csv);
        println!("[PROFILER] perf stat hardware counters only supported on Linux");
        Ok(0)
    }
}

pub fn is_perf_available() -> bool {
    Command::new("perf")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
