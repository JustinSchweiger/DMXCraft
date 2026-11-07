//! JSON-RPC 2.0 / MCP: Nachrichten auswerten und beantworten.

use serde_json::{json, Value};

use crate::{tools, AppEvent, Host};

/// Unterstützte Protokollversionen (neueste zuerst).
pub const VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

const INSTRUCTIONS: &str = "DMXCraft ist eine DMX-Lichtsteuerung im Stil von Daslight. \
Begriffe: Geräte (fixtures) sind gepatcht (Universum, Startadresse) und haben Kanäle \
(Index ab 0, Art wie Dimmer/Red/Green/Blue/White/Pan/Tilt …). Szenen liegen in Bänken \
(banks); Szenen derselben Bank schließen sich gegenseitig aus. Eine Szene hat Schritte \
(steps) mit Kanalwerten 0–255, Halte- und Überblendzeit; FX-Szenen haben zusätzlich \
Effekte (ColorFx, ChaserFx, MoveFx, ValueFx, CurveFx, Mappings, ColorMappings). \
Eine SUPER SCENE ist eine Zeitleiste mit Spuren, auf denen Szenen- und Audioblöcke liegen. \
Tipp: erst get_overview bzw. list_fixtures aufrufen, um IDs und Kanäle zu kennen. \
Änderungen sind sofort in der Oberfläche sichtbar und lassen sich dort mit Strg+Z \
rückgängig machen; gespeichert wird erst mit save_show.";

/// Beantwortet eine JSON-RPC-Nachricht (oder ein Bündel). `None`, wenn keine
/// Antwort nötig ist (nur Benachrichtigungen).
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
    // Antworten des Clients oder Benachrichtigungen: nichts zurückgeben
    let id = id?;
    if method.is_empty() {
        return Some(error(id, -32600, "Ungültige Anfrage"));
    }
    let result = match method {
        "initialize" => Ok(initialize(&params)),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({ "tools": tools::list() })),
        "tools/call" => Ok(call_tool(host, &params)),
        "resources/list" => Ok(json!({ "resources": [] })),
        "prompts/list" => Ok(json!({ "prompts": [] })),
        _ => Err((-32601, format!("Unbekannte Methode: {method}"))),
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

/// `tools/call`: Fehler eines Werkzeugs sind keine Protokollfehler, sondern
/// Ergebnisse mit `isError`, damit das Modell sie lesen und reagieren kann.
fn call_tool(host: &Host, params: &Value) -> Value {
    let name = params.get("name").and_then(Value::as_str).unwrap_or("");
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let out = tools::call(host, name, &args);
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
    let mut res = json!({
        "content": [{ "type": "text", "text": text }],
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
