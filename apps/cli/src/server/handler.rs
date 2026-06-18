use nexgit_protocol::{ClientMessage, ServerMessage};
use serde_json::Value;

use crate::commands;

use super::ServerState;

pub async fn handle_client_message(state: &ServerState, message: ClientMessage) -> ServerMessage {
    match message {
        ClientMessage::Request { id, method, params } => {
            handle_request(state, id, method.as_str(), params).await
        }
    }
}

async fn handle_request(
    state: &ServerState,
    id: u64,
    method: &str,
    _params: Value,
) -> ServerMessage {
    match method {
        "system.version" => ServerMessage::success(id, commands::version_payload()),
        "repo.status" => match state.core().repo_status() {
            Ok(status) => match serde_json::to_value(status) {
                Ok(value) => ServerMessage::success(id, value),
                Err(error) => ServerMessage::failure(id, "serialization_error", error.to_string()),
            },
            Err(error) => ServerMessage::failure(id, "repo_status_failed", error.to_string()),
        },
        "stack.list" | "stacks.list" => match state.core().stacks() {
            Ok(stacks) => match serde_json::to_value(stacks) {
                Ok(value) => ServerMessage::success(id, value),
                Err(error) => ServerMessage::failure(id, "serialization_error", error.to_string()),
            },
            Err(error) => ServerMessage::failure(id, "stack_list_failed", error.to_string()),
        },
        "github.auth.status" => match state.core().github_auth_status() {
            Ok(auth_status) => match serde_json::to_value(auth_status) {
                Ok(value) => ServerMessage::success(id, value),
                Err(error) => ServerMessage::failure(id, "serialization_error", error.to_string()),
            },
            Err(error) => ServerMessage::failure(id, "auth_status_failed", error.to_string()),
        },
        "github.repo.current" => match state.core().github_repo_current() {
            Ok(repo) => {
                let repo_json = serde_json::json!({
                    "owner": repo.owner,
                    "repo": repo.repo,
                });
                ServerMessage::success(id, repo_json)
            }
            Err(error) => ServerMessage::failure(id, "repo_current_failed", error.to_string()),
        },
        _ => ServerMessage::failure(
            id,
            "method_not_found",
            format!("unknown app-server method `{method}`"),
        ),
    }
}
