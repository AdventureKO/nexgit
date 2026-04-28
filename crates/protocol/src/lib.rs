use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION: u32 = 1;

pub type RequestId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Request {
        id: RequestId,
        method: String,
        #[serde(default)]
        params: Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Ready {
        #[serde(rename = "serverName")]
        server_name: String,
        version: String,
        #[serde(rename = "protocolVersion")]
        protocol_version: u32,
        transport: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        endpoint: Option<String>,
    },
    Response {
        id: RequestId,
        ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<ProtocolError>,
    },
    Event {
        event: String,
        #[serde(default)]
        payload: Value,
    },
}

impl ServerMessage {
    pub fn ready(transport: impl Into<String>, endpoint: Option<String>) -> Self {
        Self::Ready {
            server_name: "nexgit".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: PROTOCOL_VERSION,
            transport: transport.into(),
            endpoint,
        }
    }

    pub fn success(id: RequestId, result: Value) -> Self {
        Self::Response {
            id,
            ok: true,
            result: Some(result),
            error: None,
        }
    }

    pub fn failure(id: RequestId, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Response {
            id,
            ok: false,
            result: None,
            error: Some(ProtocolError {
                code: code.into(),
                message: message.into(),
            }),
        }
    }

    pub fn event(event: impl Into<String>, payload: Value) -> Self {
        Self::Event {
            event: event.into(),
            payload,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolError {
    pub code: String,
    pub message: String,
}

pub const TYPESCRIPT_DEFINITIONS: &str = r#"export type RequestId = number;

export type ClientMessage =
  | {
      type: 'request';
      id: RequestId;
      method: string;
      params?: unknown;
    };

export type ServerMessage =
  | {
      type: 'ready';
      serverName: string;
      version: string;
      protocolVersion: number;
      transport: string;
      endpoint?: string;
    }
  | {
      type: 'response';
      id: RequestId;
      ok: true;
      result?: unknown;
    }
  | {
      type: 'response';
      id: RequestId;
      ok: false;
      error: ProtocolError;
    }
  | {
      type: 'event';
      event: string;
      payload?: unknown;
    };

export type ProtocolError = {
  code: string;
  message: string;
};
"#;
