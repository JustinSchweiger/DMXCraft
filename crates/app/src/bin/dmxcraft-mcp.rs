//! `dmxcraft-mcp`: stdio-Brücke für MCP-Clients, die Server als Prozess
//! starten (z. B. Claude Desktop). Ein eigenes, kleines Programm, damit die
//! dauerhaft laufende Brücke nicht `dmxcraft(.exe)` sperrt – sonst lässt sich
//! DMXCraft unter Windows nicht neu bauen oder aktualisieren, solange der
//! Client läuft.
//!
//! Aufruf: `dmxcraft-mcp [--port 7373]`

#[path = "../../../mcp/src/bridge.rs"]
mod bridge;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|p| p.parse().ok())
        .unwrap_or(7373);
    bridge::run(port);
}
