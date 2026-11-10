//! `dmxcraft-mcp`: stdio bridge for MCP clients that start servers as a
//! process (e.g. Claude Desktop). A separate small program so the
//! permanently running bridge does not lock `dmxcraft(.exe)` – otherwise
//! DMXCraft cannot be rebuilt or updated on Windows while the client is
//! running.
//!
//! Usage: `dmxcraft-mcp [--port 7373]`

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
