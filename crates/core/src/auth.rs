use crate::Result;
use std::path::Path;
use std::process::Command;

/// GitHub authentication status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub message: String,
}

/// Check if GitHub CLI authentication is available.
///
/// This runs `gh auth status` to verify that the user has authenticated with GitHub.
/// Returns a clear message about the authentication state.
pub fn check_auth_status() -> Result<AuthStatus> {
    let output = Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map_err(|e| {
            crate::CoreError::Message(format!(
                "GitHub CLI not found or not accessible: {}",
                e
            ))
        })?;

    let authenticated = output.status.success();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let message = if authenticated {
        "GitHub CLI authenticated".to_string()
    } else if stderr.contains("not authenticated") || stdout.contains("not authenticated") {
        "GitHub CLI not authenticated. Run 'gh auth login' to authenticate.".to_string()
    } else {
        format!(
            "GitHub CLI authentication check failed: {}",
            stderr.trim().to_string()
        )
    };

    Ok(AuthStatus {
        authenticated,
        message,
    })
}
