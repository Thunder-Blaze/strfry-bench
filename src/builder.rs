use std::path::{Path, PathBuf};
use std::process::Command;
use crate::system::num_cpus;

pub const ALLOC_TRACKER_C: &str = r#"#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <stdatomic.h>

static void* (*real_malloc)(size_t) = NULL;
static void* (*real_calloc)(size_t, size_t) = NULL;
static void* (*real_realloc)(void*, size_t) = NULL;
static void (*real_free)(void*) = NULL;

static atomic_size_t total_allocations = 0;
static atomic_size_t total_bytes = 0;

static __thread int in_hook = 0;
static char bootstrap_buf[4096];
static size_t bootstrap_offset = 0;

static void init() {
    if (real_malloc) return;
    in_hook = 1;
    real_malloc = (void* (*)(size_t))dlsym(RTLD_NEXT, "malloc");
    real_calloc = (void* (*)(size_t, size_t))dlsym(RTLD_NEXT, "calloc");
    real_realloc = (void* (*)(void*, size_t))dlsym(RTLD_NEXT, "realloc");
    real_free = (void (*)(void*))dlsym(RTLD_NEXT, "free");
    in_hook = 0;
}

void* malloc(size_t size) {
    if (!real_malloc) init();
    if (in_hook) return real_malloc ? real_malloc(size) : NULL;
    
    in_hook = 1;
    void* ptr = real_malloc(size);
    in_hook = 0;
    
    if (ptr) {
        atomic_fetch_add(&total_allocations, 1);
        atomic_fetch_add(&total_bytes, size);
    }
    return ptr;
}

void* calloc(size_t nmemb, size_t size) {
    if (!real_calloc) {
        if (in_hook) {
            size_t total = nmemb * size;
            if (bootstrap_offset + total < sizeof(bootstrap_buf)) {
                void* ptr = &bootstrap_buf[bootstrap_offset];
                bootstrap_offset += total;
                return ptr;
            }
            return NULL;
        }
        init();
    }
    if (in_hook) return real_calloc ? real_calloc(nmemb, size) : NULL;
    
    in_hook = 1;
    void* ptr = real_calloc(nmemb, size);
    in_hook = 0;
    
    if (ptr) {
        atomic_fetch_add(&total_allocations, 1);
        atomic_fetch_add(&total_bytes, nmemb * size);
    }
    return ptr;
}

void* realloc(void* ptr, size_t size) {
    if (!real_realloc) init();
    if (in_hook) return real_realloc ? real_realloc(ptr, size) : NULL;
    
    in_hook = 1;
    void* new_ptr = real_realloc(ptr, size);
    in_hook = 0;
    
    if (new_ptr) {
        atomic_fetch_add(&total_allocations, 1);
        atomic_fetch_add(&total_bytes, size);
    }
    return new_ptr;
}

void free(void* ptr) {
    if (!real_free) init();
    if (real_free) {
        if (ptr >= (void*)bootstrap_buf && ptr < (void*)(bootstrap_buf + sizeof(bootstrap_buf))) {
            return;
        }
        real_free(ptr);
    }
}

__attribute__((destructor)) void report() {
    const char* out_path = getenv("ALLOC_TRACKER_OUT");
    FILE* f = stderr;
    if (out_path) {
        f = fopen(out_path, "a");
        if (!f) f = stderr;
    }
    fprintf(f, "ALLOCS: %zu\nBYTES: %zu\n", atomic_load(&total_allocations), atomic_load(&total_bytes));
    if (f != stderr) fclose(f);
}
"#;

pub struct Builder {
    pub dir: PathBuf,
    pub high_performance: bool,
}

impl Builder {
    pub fn new(dir: impl Into<PathBuf>, high_performance: bool) -> Self {
        Self {
            dir: dir.into(),
            high_performance,
        }
    }

    /// Build strfry binary. Default uses -j4; uses -j$(nproc) only when high_performance is true.
    pub fn build_strfry(&self) -> Result<PathBuf, String> {
        let jobs = if self.high_performance {
            num_cpus()
        } else {
            4
        };

        let jobs_arg = format!("-j{}", jobs);

        // Determine make command (cross-platform check)
        let make_cmd = if cfg!(windows) {
            if which_exists("mingw32-make") {
                "mingw32-make"
            } else if which_exists("make") {
                "make"
            } else if which_exists("nmake") {
                "nmake"
            } else {
                "make"
            }
        } else {
            "make"
        };

        println!(
            "[BUILDER] Building strfry in {} using {} {}",
            self.dir.display(),
            make_cmd,
            jobs_arg
        );

        let out = Command::new(make_cmd)
            .current_dir(&self.dir)
            .arg(&jobs_arg)
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", make_cmd, e))?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stdout = String::from_utf8_lossy(&out.stdout);
            return Err(format!(
                "Build failed with exit code {:?}:\nStdout:\n{}\nStderr:\n{}",
                out.status.code(),
                stdout,
                stderr
            ));
        }

        let bin_name = if cfg!(windows) { "strfry.exe" } else { "strfry" };
        let binary_path = self.dir.join(bin_name);

        if !binary_path.exists() {
            return Err(format!(
                "Build succeeded but binary not found at {}",
                binary_path.display()
            ));
        }

        println!("[BUILDER] strfry built successfully: {}", binary_path.display());
        Ok(binary_path)
    }

    /// Compile embedded alloc_tracker shared library
    pub fn build_alloc_tracker(&self, output_dir: &Path) -> Result<Option<PathBuf>, String> {
        #[cfg(target_os = "windows")]
        {
            let _ = output_dir;
            println!("[BUILDER] alloc_tracker is skipped on Windows (LD_PRELOAD not supported)");
            return Ok(None);
        }

        #[cfg(not(target_os = "windows"))]
        {
            let is_macos = cfg!(target_os = "macos");
            let so_name = if is_macos {
                "alloc_tracker.dylib"
            } else {
                "alloc_tracker.so"
            };

            let c_path = output_dir.join("alloc_tracker.c");
            let so_path = output_dir.join(so_name);

            std::fs::write(&c_path, ALLOC_TRACKER_C)
                .map_err(|e| format!("Failed to write alloc_tracker.c: {}", e))?;

            let mut cmd = Command::new("gcc");
            cmd.args(["-shared", "-fPIC", "-o"])
                .arg(&so_path)
                .arg(&c_path);

            if !is_macos {
                cmd.arg("-ldl");
            }

            let out = cmd.output().map_err(|e| format!("Failed to invoke gcc: {}", e))?;

            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr);
                return Err(format!("Failed to compile alloc_tracker: {}", stderr));
            }

            println!("[BUILDER] Compiled allocation tracker to {}", so_path.display());
            Ok(Some(so_path))
        }
    }
}

fn which_exists(cmd: &str) -> bool {
    Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
