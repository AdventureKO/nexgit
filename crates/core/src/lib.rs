pub mod auth;
pub mod error;
pub mod github;
pub mod repo;
pub mod stack;

pub use auth::AuthStatus;
pub use error::{CoreError, Result};
pub use github::{resolve_github_repo, GitHubRepo};
pub use repo::RepoStatus;
pub use stack::StackSummary;

#[derive(Debug, Clone, Default)]
pub struct Core;

impl Core {
    pub fn new() -> Self {
        Self
    }

    pub fn repo_status(&self) -> Result<RepoStatus> {
        Ok(RepoStatus::placeholder())
    }

    pub fn stacks(&self) -> Result<Vec<stack::StackSummary>> {
        Ok(Vec::new())
    }

    pub fn github_auth_status(&self) -> Result<AuthStatus> {
        auth::check_auth_status()
    }

    pub fn github_repo_current(&self) -> Result<GitHubRepo> {
        github::resolve_github_repo(None)
    }
}
