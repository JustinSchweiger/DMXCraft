//! stdio bridge: `dmxcraft mcp` reads JSON-RPC messages line by line from
//! stdin, sends them to the HTTP endpoint of the running DMXCraft instance
//! and writes the answers to stdout. That way MCP clients that start
//! servers as a process (e.g. Claude Desktop) can control DMXCraft.
//!
//! Only `std` and `serde_json`: the same file is also part of the small
//! `dmxcraft-mcp` program (crates/app/src/bin) so the running bridge does
//! not lock `dmxcraft.exe` (rebuilding on Windows).

use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use serde_json::{json, Value};

/// Runs until stdin ends.
pub fn run(port: u16) {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            break;
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let answer = match forward(port, line) {
            Ok(Some(body)) => Some(body),
            Ok(None) => None,
            Err(err) => error_for(line, &err),
        };
        if let Some(a) = answer {
            // one message per line
            let a = a.replace(['\n', '\r'], " ");
            if writeln!(stdout, "{a}")
                .and_then(|_| stdout.flush())
                .is_err()
            {
                break;
            }
        }
    }
}

/// Error answer when DMXCraft is not reachable (only for requests with an
/// ID).
fn error_for(line: &str, err: &str) -> Option<String> {
    let msg: Value = serde_json::from_str(line).ok()?;
    let id = msg.get("id")?.clone();
    Some(
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32000, "message": err },
        })
        .to_string(),
    )
}

/// Forward a message via HTTP. `Ok(None)` for 202 (no answer).
pub(crate) fn forward(port: u16, body: &str) -> Result<Option<String>, String> {
    let not_running = || {
        format!(
            "DMXCraft is not reachable (127.0.0.1:{port}). Please start DMXCraft and enable \
             the server under Settings → MCP."
        )
    };
    let mut s = TcpStream::connect(("127.0.0.1", port)).map_err(|_| not_running())?;
    s.set_read_timeout(Some(Duration::from_secs(60))).ok();
    write!(
        s,
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .map_err(|_| not_running())?;
    let mut raw = Vec::new();
    s.read_to_end(&mut raw).map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&raw);
    let (head, rest) = text.split_once("\r\n\r\n").ok_or("invalid response")?;
    let status: u16 = head
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .ok_or("invalid response")?;
    let chunked = head.lines().any(|l| {
        l.to_ascii_lowercase()
            .starts_with("transfer-encoding: chunked")
    });
    let body = if chunked {
        dechunk(rest)
    } else {
        rest.to_string()
    };
    match status {
        202 => Ok(None),
        200 => Ok(Some(body)),
        _ => Err(format!(
            "DMXCraft answered with HTTP {status}: {}",
            body.trim()
        )),
    }
}

/// Resolve `Transfer-Encoding: chunked`.
fn dechunk(mut s: &str) -> String {
    let mut out = String::new();
    while let Some((len, rest)) = s.split_once("\r\n") {
        let Ok(n) = usize::from_str_radix(len.trim(), 16) else {
            break;
        };
        if n == 0 || rest.len() < n {
            break;
        }
        out.push_str(&rest[..n]);
        s = rest[n..].trim_start_matches("\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dechunks() {
        assert_eq!(dechunk("5\r\nhello\r\n3\r\n !!\r\n0\r\n\r\n"), "hello !!");
    }

    #[test]
    fn not_running_gives_error_with_id() {
        // Port 1 is practically never in use
        let e = forward(1, r#"{"jsonrpc":"2.0","id":7,"method":"ping"}"#).unwrap_err();
        let a = error_for(r#"{"jsonrpc":"2.0","id":7,"method":"ping"}"#, &e).unwrap();
        assert!(a.contains("\"id\":7"));
        assert!(error_for(r#"{"jsonrpc":"2.0","method":"x"}"#, &e).is_none());
    }
}
