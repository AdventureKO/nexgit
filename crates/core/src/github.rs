use crate::{CoreError, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Represents a GitHub repository as owner/repo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
}

impl GitHubRepo {
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

/// Resolve the current GitHub repository from Git remotes in the given directory.
///
/// This function:
/// 1. Reads Git remotes from the current working directory
/// 2. Looks for GitHub remotes (github.com URLs)
/// 3. Prefers "origin" if available
/// 4. Errors on missing, non-GitHub, or ambiguous remotes
pub fn resolve_github_repo(cwd: Option<&Path>) -> Result<GitHubRepo> {
    let remotes = read_git_remotes(cwd)?;

    if remotes.is_empty() {
        return Err(CoreError::Message(
            "No Git remotes found. Please check that this is a Git repository.".to_string(),
        ));
    }

    // Filter for GitHub remotes
    let github_remotes: HashMap<String, String> = remotes
        .into_iter()
        .filter_map(|(name, url)| parse_github_remote(&url).map(|repo| (name, repo)))
        .collect();

    if github_remotes.is_empty() {
        return Err(CoreError::Message(
            "No GitHub remotes found. Nexgit currently only supports GitHub repositories."
                .to_string(),
        ));
    }

    // Prefer "origin" if available
    if let Some(origin_url) = github_remotes.get("origin") {
        return parse_owner_repo(origin_url).ok_or_else(|| {
            CoreError::Message("Failed to parse GitHub remote URL.".to_string())
        });
    }

    // If only one remote, use it
    if github_remotes.len() == 1 {
        let url = github_remotes.values().next().unwrap();
        return parse_owner_repo(url).ok_or_else(|| {
            CoreError::Message("Failed to parse GitHub remote URL.".to_string())
        });
    }

    // Multiple non-origin GitHub remotes
    Err(CoreError::Message(
        "Multiple GitHub remotes found and no 'origin'. Please specify which one to use."
            .to_string(),
    ))
}

/// Read all Git remotes from the repository.
fn read_git_remotes(cwd: Option<&Path>) -> Result<Vec<(String, String)>> {
    let mut cmd = Command::new("git");
    cmd.args(["config", "--get-regexp", "^remote\\..*\\.url"]);

    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }

    let output = cmd
        .output()
        .map_err(|e| CoreError::Message(format!("Failed to run git command: {}", e)))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut remotes = Vec::new();

    for line in stdout.lines() {
        // Format: remote.NAME.url VALUE
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            // remote.origin.url -> origin
            let name_part = parts[0]; // remote.origin.url
            if let Some(name) = name_part.strip_prefix("remote.").and_then(|s| s.strip_suffix(".url")) {
                let url = parts[1..].join(" ");
                remotes.push((name.to_string(), url));
            }
        }
    }

    Ok(remotes)
}

/// Check if a URL is a GitHub remote and return the URL portion.
fn parse_github_remote(url: &str) -> Option<String> {
    if url.contains("github.com") {
        Some(url.to_string())
    } else {
        None
    }
}

/// Parse OWNER/REPO from a GitHub remote URL.
///
/// Supports:
/// - https://github.com/OWNER/REPO
/// - https://github.com/OWNER/REPO.git
/// - git@github.com:OWNER/REPO
/// - git@github.com:OWNER/REPO.git
fn parse_owner_repo(url: &str) -> Option<GitHubRepo> {
    let url = url.trim();

    // Handle git@github.com:OWNER/REPO format
    if let Some(rest) = url.strip_prefix("git@github.com:") {
        let path = rest.strip_suffix(".git").unwrap_or(rest);
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Some(GitHubRepo::new(parts[0], parts[1]));
        }
    }

    // Handle https://github.com/OWNER/REPO format
    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let path = rest.strip_suffix(".git").unwrap_or(rest);
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Some(GitHubRepo::new(parts[0], parts[1]));
        }
    }

    // Handle http://github.com/OWNER/REPO format
    if let Some(rest) = url.strip_prefix("http://github.com/") {
        let path = rest.strip_suffix(".git").unwrap_or(rest);
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() == 2 {
            return Some(GitHubRepo::new(parts[0], parts[1]));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owner_repo_https() {
        let repo = parse_owner_repo("https://github.com/Codelab-Davis/nexgit").unwrap();
        assert_eq!(repo.owner, "Codelab-Davis");
        assert_eq!(repo.repo, "nexgit");
    }

    #[test]
    fn test_parse_owner_repo_https_git() {
        let repo = parse_owner_repo("https://github.com/Codelab-Davis/nexgit.git").unwrap();
        assert_eq!(repo.owner, "Codelab-Davis");
        assert_eq!(repo.repo, "nexgit");
    }

    #[test]
    fn test_parse_owner_repo_ssh() {
        let repo = parse_owner_repo("git@github.com:Codelab-Davis/nexgit").unwrap();
        assert_eq!(repo.owner, "Codelab-Davis");
        assert_eq!(repo.repo, "nexgit");
    }

    #[test]
    fn test_parse_owner_repo_ssh_git() {
        let repo = parse_owner_repo("git@github.com:Codelab-Davis/nexgit.git").unwrap();
        assert_eq!(repo.owner, "Codelab-Davis");
        assert_eq!(repo.repo, "nexgit");
    }

    #[test]
    fn test_parse_github_remote() {
        assert!(parse_github_remote("https://github.com/Codelab-Davis/nexgit").is_some());
        assert!(parse_github_remote("git@github.com:Codelab-Davis/nexgit").is_some());
        assert!(parse_github_remote("https://gitlab.com/user/repo").is_none());
    }

    #[test]
    fn test_github_repo_to_string() {
        let repo = GitHubRepo::new("Codelab-Davis", "nexgit");
        assert_eq!(repo.to_string(), "Codelab-Davis/nexgit");
    }
}
