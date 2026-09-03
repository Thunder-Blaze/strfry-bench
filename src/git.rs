use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub timestamp: i64,
    pub date: String,
    pub message: String,
    pub refs: String,
}

pub struct GitManager {
    pub repo_path: PathBuf,
}

impl GitManager {
    pub fn new(repo_path: impl Into<PathBuf>) -> Self {
        Self {
            repo_path: repo_path.into(),
        }
    }

    pub fn is_git_repo(&self) -> bool {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["rev-parse", "--is-inside-work-tree"])
            .output();

        matches!(output, Ok(out) if out.status.success() && String::from_utf8_lossy(&out.stdout).trim() == "true")
    }

    pub fn get_current_commit(&self) -> Result<String, String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["rev-parse", "HEAD"])
            .output()
            .map_err(|e| format!("Failed to run git rev-parse: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    pub fn get_current_branch(&self) -> Result<String, String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .map_err(|e| format!("Failed to run git: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    pub fn has_uncommitted_changes(&self) -> bool {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["status", "--porcelain"])
            .output();

        match out {
            Ok(o) if o.status.success() => !String::from_utf8_lossy(&o.stdout).trim().is_empty(),
            _ => false,
        }
    }
    pub fn get_all_branches(&self) -> Result<Vec<String>, String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["branch", "-a", "--format=%(refname:short)"])
            .output()
            .map_err(|e| format!("Failed to list branches: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        let raw = String::from_utf8_lossy(&out.stdout);
        let mut branches = Vec::new();
        for line in raw.lines() {
            let b = line.trim();
            if b.is_empty() || b.contains("HEAD") {
                continue;
            }
            let clean_b = if let Some(stripped) = b.strip_prefix("origin/") {
                stripped
            } else {
                b
            };
            if !branches.contains(&clean_b.to_string()) {
                branches.push(clean_b.to_string());
            }
        }
        branches.sort();
        Ok(branches)
    }

    pub fn get_commits_for_ref(&self, git_ref: Option<&str>, limit: usize) -> Result<Vec<CommitInfo>, String> {
        let limit_str = format!("-n{}", limit);
        let mut cmd = Command::new("git");
        cmd.arg("-C").arg(&self.repo_path).arg("log").arg(&limit_str);

        if let Some(r) = git_ref {
            if !r.is_empty() {
                cmd.arg(r);
            } else {
                cmd.arg("--all");
            }
        } else {
            cmd.arg("--all");
        }

        cmd.arg("--format=%H%x00%an%x00%at%x00%s%x00%D");
        let out = cmd.output().map_err(|e| format!("Failed to run git log: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        let raw = String::from_utf8_lossy(&out.stdout);
        let mut commits = Vec::new();

        for line in raw.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split('\0').collect();
            if parts.len() >= 5 {
                let hash = parts[0].to_string();
                let short_hash = if hash.len() >= 7 {
                    hash[0..7].to_string()
                } else {
                    hash.clone()
                };
                let author = parts[1].to_string();
                let timestamp = parts[2].parse::<i64>().unwrap_or(0);
                let message = parts[3].to_string();
                let refs = parts[4].to_string();

                let date = chrono::DateTime::from_timestamp(timestamp, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                commits.push(CommitInfo {
                    hash,
                    short_hash,
                    author,
                    timestamp,
                    date,
                    message,
                    refs,
                });
            }
        }

        Ok(commits)
    }

    pub fn get_commit_log(&self, limit: usize) -> Result<Vec<CommitInfo>, String> {
        self.get_commits_for_ref(None, limit)
    }

    pub fn checkout(&self, git_ref: &str) -> Result<(), String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["checkout", git_ref])
            .output()
            .map_err(|e| format!("Failed to run git checkout: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(())
    }

    pub fn stash_push(&self, message: &str) -> Result<bool, String> {
        if !self.has_uncommitted_changes() {
            return Ok(false);
        }

        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["stash", "push", "-u", "-m", message])
            .output()
            .map_err(|e| format!("Failed to run git stash push: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(true)
    }

    pub fn stash_pop(&self) -> Result<(), String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["stash", "pop"])
            .output()
            .map_err(|e| format!("Failed to run git stash pop: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(())
    }

    pub fn create_worktree(&self, target_dir: &Path, git_ref: &str) -> Result<(), String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["worktree", "add", "--detach", target_dir.to_str().unwrap(), git_ref])
            .output()
            .map_err(|e| format!("Failed to create worktree: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(())
    }

    pub fn remove_worktree(&self, target_dir: &Path) -> Result<(), String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.repo_path)
            .args(["worktree", "remove", "--force", target_dir.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to remove worktree: {}", e))?;

        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }

        Ok(())
    }
}

/// RAII Guard ensuring that original git branch and stashed changes are always restored.
pub struct StashGuard<'a> {
    manager: &'a GitManager,
    original_ref: String,
    stashed: bool,
    active: bool,
}

impl<'a> StashGuard<'a> {
    pub fn enter(manager: &'a GitManager, stash_label: &str) -> Result<Self, String> {
        let branch = manager.get_current_branch().unwrap_or_else(|_| "HEAD".to_string());
        let original_ref = if branch == "HEAD" {
            manager.get_current_commit()?
        } else {
            branch
        };

        let stashed = manager.stash_push(stash_label)?;
        if stashed {
            println!("[GIT] Automatically stashed uncommitted working tree changes ({})", stash_label);
        }

        Ok(Self {
            manager,
            original_ref,
            stashed,
            active: true,
        })
    }

    pub fn restore(&mut self) -> Result<(), String> {
        if !self.active {
            return Ok(());
        }
        self.active = false;

        let mut res = Ok(());
        println!("[GIT] Restoring repository to original reference: {}", self.original_ref);
        if let Err(e) = self.manager.checkout(&self.original_ref) {
            eprintln!("[ERROR] Failed to restore git reference {}: {}", self.original_ref, e);
            res = Err(e);
        }

        if self.stashed {
            println!("[GIT] Automatically popping stashed changes onto {}", self.original_ref);
            if let Err(e) = self.manager.stash_pop() {
                eprintln!("[ERROR] Failed to pop stash onto {}: {}", self.original_ref, e);
                res = Err(e);
            }
        }

        res
    }
}

impl<'a> Drop for StashGuard<'a> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
