use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoStatus {
    pub root: Option<String>,
    pub current_branch: Option<String>,
    pub is_clean: bool,
    pub pending_changes: u32,
}

impl RepoStatus {
    pub fn placeholder() -> Self {
        Self {
            root: None,
            current_branch: None,
            is_clean: true,
            pending_changes: 0,
        }
    }
}
