pub mod error;
pub mod repo;
pub mod stack;

pub use error::{CoreError, Result};
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

    pub fn stacks(&self) -> Result<Vec<StackSummary>> {
        Ok(Vec::new())
    }
}
