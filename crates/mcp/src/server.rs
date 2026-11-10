//! HTTP endpoint (“Streamable HTTP” of MCP): `POST /mcp` with JSON-RPC.
//! Listens on 127.0.0.1 only; requests from web pages (foreign `Origin`)
//! are rejected.

use std::io::Read;
use std::sync::Arc;
use std::thread::JoinHandle;

use serde_json::Value;
use tiny_http::{Header, Method, Response, Server};

use crate::{protocol, Host};

/// Running server; it is shut down when dropped.
pub struct McpServer {
    server: Arc<Server>,
    thread: Option<JoinHandle<()>>,
    pub port: u16,
}

impl McpServer {
    /// Starts the server on `127.0.0.1:port`.
    pub fn start(host: Host, port: u16) -> Result<Self, String> {
        let server =
            Arc::new(Server::http(("127.0.0.1", port)).map_err(|e| format!("Port {port}: {e}"))?);
        let s = server.clone();
        let thread = std::thread::Builder::new()
            .name("mcp".into())
            .spawn(move || {
                for req in s.incoming_requests() {
                    serve(&host, req);
                }
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            server,
            thread: Some(thread),
            port,
        })
    }
}

impl Drop for McpServer {
    fn drop(&mut self) {
        // Stop accepting, but do not wait for the thread: a hanging connection
        // must not hold up closing DMXCraft
        self.server.unblock();
        drop(self.thread.take());
    }
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("valid header")
}

/// Only allow calls without `Origin` (programs) or from localhost.
fn origin_ok(req: &tiny_http::Request) -> bool {
    req.headers()
        .iter()
        .find(|h| h.field.equiv("Origin"))
        .is_none_or(|h| {
            let o = h.value.as_str();
            ["http://localhost", "http://127.0.0.1", "https://localhost"]
                .iter()
                .any(|p| o == *p || o.starts_with(&format!("{p}:")))
        })
}

fn serve(host: &Host, mut req: tiny_http::Request) {
    let path = req.url().split('?').next().unwrap_or("").to_string();
    if path != "/mcp" && path != "/" {
        let _ = req.respond(Response::from_string("Not found").with_status_code(404));
        return;
    }
    if !origin_ok(&req) {
        let _ = req.respond(Response::from_string("Forbidden origin").with_status_code(403));
        return;
    }
    match req.method() {
        Method::Post => {}
        Method::Delete => {
            // End session: there are no sessions
            let _ = req.respond(Response::empty(200));
            return;
        }
        _ => {
            // no server-sent events stream
            let _ = req.respond(
                Response::from_string("Method not allowed")
                    .with_status_code(405)
                    .with_header(header("Allow", "POST")),
            );
            return;
        }
    }
    let mut body = String::new();
    if req
        .as_reader()
        .take(16 << 20)
        .read_to_string(&mut body)
        .is_err()
    {
        let _ = req.respond(Response::from_string("Bad request").with_status_code(400));
        return;
    }
    let msg: Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => {
            let err = serde_json::json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":format!("Parse error: {e}")}});
            let _ = req.respond(
                Response::from_string(err.to_string())
                    .with_status_code(400)
                    .with_header(header("Content-Type", "application/json")),
            );
            return;
        }
    };
    match protocol::handle(host, msg) {
        Some(answer) => {
            let _ = req.respond(
                Response::from_string(answer.to_string())
                    .with_header(header("Content-Type", "application/json")),
            );
        }
        None => {
            let _ = req.respond(Response::empty(202));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bridge_forwards_to_server() {
        let host =
            crate::Host::headless(dmxcraft_core::Engine::new(dmxcraft_core::Show::new("T", 1)));
        let port = (40100..40200)
            .find(|p| std::net::TcpListener::bind(("127.0.0.1", *p)).is_ok())
            .unwrap();
        let _srv = McpServer::start(host, port).unwrap();
        let r = crate::bridge::forward(port, r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#)
            .unwrap()
            .unwrap();
        assert!(r.contains("create_scene"));
    }

    use super::*;
    use std::io::Write;
    use std::net::TcpStream;

    fn post(port: u16, body: &str, origin: Option<&str>) -> (u16, String) {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let o = origin.map_or(String::new(), |o| format!("Origin: {o}\r\n"));
        write!(
            s,
            "POST /mcp HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\n{o}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        let status = out[9..12].parse().unwrap();
        let body = out.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
        (status, body)
    }

    #[test]
    fn http_roundtrip() {
        let host = Host::headless(dmxcraft_core::Engine::new(dmxcraft_core::Show::new("T", 1)));
        // find a free port
        let port = (40000..40100)
            .find(|p| std::net::TcpListener::bind(("127.0.0.1", *p)).is_ok())
            .unwrap();
        let srv = McpServer::start(host, port).unwrap();
        let (st, body) = post(port, r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#, None);
        assert_eq!(st, 200);
        assert!(body.contains("\"result\""));
        let (st, _) = post(
            port,
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            None,
        );
        assert_eq!(st, 202);
        let (st, _) = post(
            port,
            r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
            Some("https://evil.example"),
        );
        assert_eq!(st, 403);
        drop(srv);
    }
}
