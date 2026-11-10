//! JSON-RPC 2.0 / MCP: evaluate and answer messages.

use serde_json::{json, Value};

use crate::{tools, AppEvent, Host};

/// Supported protocol versions (newest first).
pub const VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

const INSTRUCTIONS: &str = "DMXCraft is a DMX lighting control in the style of Daslight. \
Terms: fixtures are patched (universe, start address) and have channels \
(index from 0, kind such as Dimmer/Red/Green/Blue/White/Pan/Tilt …). Scenes live in \
banks; scenes of the same bank exclude each other. A scene has steps with channel \
values 0–255, hold and fade time; FX scenes additionally have effects (ColorFx, \
ChaserFx, MoveFx, ValueFx, CurveFx, Mappings, ColorMappings). A SUPER SCENE is a \
timeline with tracks that hold scene and audio blocks. \
Tip: call get_overview or list_fixtures first to learn IDs and channels. \
visualizer_screenshot shows the result in the 3D visualizer; custom fixture \
profiles (Profile Builder) go through list_profiles, get_profile and save_profile. \
Changes are visible in the UI immediately and can be undone there with Ctrl+Z; \
nothing is saved until save_show.";

/// Answers a JSON-RPC message (or a batch). `None` if no answer is needed
/// (notifications only).
pub fn handle(host: &Host, msg: Value) -> Option<Value> {
    match msg {
        Value::Array(batch) => {
            let out: Vec<Value> = batch
                .into_iter()
                .filter_map(|m| handle_one(host, m))
                .collect();
            (!out.is_empty()).then_some(Value::Array(out))
        }
        m => handle_one(host, m),
    }
}

fn handle_one(host: &Host, msg: Value) -> Option<Value> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    // Client answers or notifications: return nothing
    let id = id?;
    if method.is_empty() {
        return Some(error(id, -32600, "Invalid request"));
    }
    let result = match method {
        "initialize" => Ok(initialize(&params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools::list() })),
        "tools/call" => Ok(call_tool(host, &params)),
        "resources/list" => Ok(json!({ "resources": [] })),
        "prompts/list" => Ok(json!({ "prompts": [] })),
        _ => Err((-32601, format!("Unknown method: {method}"))),
    };
    Some(match result {
        Ok(r) => json!({ "jsonrpc": "2.0", "id": id, "result": r }),
        Err((code, m)) => error(id, code, &m),
    })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn initialize(params: &Value) -> Value {
    let asked = params
        .get("protocolVersion")
        .and_then(Value::as_str)
        .unwrap_or(VERSIONS[0]);
    let version = VERSIONS
        .iter()
        .find(|v| **v == asked)
        .copied()
        .unwrap_or(VERSIONS[0]);
    json!({
        "protocolVersion": version,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": "dmxcraft", "title": "DMXCraft", "version": env!("CARGO_PKG_VERSION") },
        "instructions": INSTRUCTIONS,
    })
}

/// `tools/call`: errors of a tool are not protocol errors but results with
/// `isError` so the model can read them and react.
fn call_tool(host: &Host, params: &Value) -> Value {
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut out = tools::call(host, name, &args);
    // Image (e.g. visualizer_screenshot) as its own content
    let image = match &mut out {
        Ok(Value::Object(m)) => m.remove("__image_png"),
        _ => None,
    };
    let (text, is_error) = match &out {
        Ok(v) => (
            serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string()),
            false,
        ),
        Err(e) => (e.clone(), true),
    };
    if !is_error {
        (host.notify)(AppEvent::ToolCalled(name.to_string()));
    }
    let mut content = vec![json!({ "type": "text", "text": text })];
    if let Some(Value::String(data)) = image {
        content.push(json!({ "type": "image", "data": data, "mimeType": "image/png" }));
    }
    let mut res = json!({
        "content": content,
        "isError": is_error,
    });
    if let Ok(v @ Value::Object(_)) = out {
        res["structuredContent"] = v;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> Host {
        let mut show = dmxcraft_core::Show::new("Test", 1);
        for t in dmxcraft_core::library::builtin_fixture_types() {
            show.upsert_fixture_type(t);
        }
        Host::headless(dmxcraft_core::Engine::new(show))
    }

    #[test]
    fn initialize_and_list() {
        let h = host();
        let r = handle(
            &h,
            json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}),
        )
        .unwrap();
        assert_eq!(r["result"]["protocolVersion"], "2025-03-26");
        assert!(handle(
            &h,
            json!({"jsonrpc":"2.0","method":"notifications/initialized"})
        )
        .is_none());
        let r = handle(&h, json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})).unwrap();
        let tools = r["result"]["tools"].as_array().unwrap();
        assert!(tools.len() > 20);
        for t in tools {
            assert_eq!(t["inputSchema"]["type"], "object", "{}", t["name"]);
        }
    }

    #[test]
    fn unknown_tool_is_tool_error() {
        let h = host();
        let r = handle(
            &h,
            json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"nope","arguments":{}}}),
        )
        .unwrap();
        assert_eq!(r["result"]["isError"], true);
    }
}
