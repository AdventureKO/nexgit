use serde_json::{json, Value};

pub fn version_payload() -> Value {
    json!({
        "name": "nexgit",
        "version": env!("CARGO_PKG_VERSION"),
        "protocolVersion": nexgit_protocol::PROTOCOL_VERSION,
    })
}

pub fn protocol_schema_placeholder() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Nexgit app-server protocol",
        "description": "Placeholder schema. Use `nexgit app-server generate-ts` for the current TypeScript wire types.",
        "type": "object",
    })
}
