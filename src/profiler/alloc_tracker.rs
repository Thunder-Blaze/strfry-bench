use std::path::Path;

pub fn parse_alloc_tracker_output(path: &Path) -> (u64, u64) {
    let mut allocs = 0;
    let mut bytes = 0;

    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            if line.starts_with("ALLOCS:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    allocs += val.parse::<u64>().unwrap_or(0);
                }
            } else if line.starts_with("BYTES:")
                && let Some(val) = line.split_whitespace().nth(1) {
                    bytes += val.parse::<u64>().unwrap_or(0);
                }
        }
    }

    (allocs, bytes)
}
