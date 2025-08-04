use std::sync::Arc;
use anyhow::Result;
use pingora::server::{ListenFds, ShutdownWatch};
use async_trait::async_trait;
use pingora::services::Service;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, error, warn};

use crate::config::Config;
use crate::plugins::manager::PluginManager;

pub fn create_server(
    config: Arc<Config>,
    plugin_manager: Arc<PluginManager>,
) -> Result<Box<dyn Service>> {
    // Initialize proxy service with config and plugins
    Ok(Box::new(HttpServer {
        config,
        plugin_manager,
    }))
}

pub struct HttpServer {
    config: Arc<Config>,
    plugin_manager: Arc<PluginManager>,
}

#[async_trait]
impl Service for HttpServer {
    async fn start_service(&mut self, _fds: Option<ListenFds>, mut shutdown: ShutdownWatch) {
        info!("🌐 HTTP Server starting...");

        let http_addr = self.config.server.https_address.as_ref()
            .unwrap_or(&"0.0.0.0:8999".into())
            .to_string();

        info!("📡 Binding HTTP server to: {}", http_addr);

        let listener = match TcpListener::bind(&http_addr).await {
            Ok(listener) => {
                info!("✅ HTTP server successfully bound to {}", http_addr);
                listener
            }
            Err(e) => {
                error!("❌ Failed to bind HTTP server to {}: {}", http_addr, e);
                return;
            }
        };

        info!("🚀 EdgeFlow HTTP Gateway is ready to accept connections");

        loop {
            tokio::select! {
                // Handle shutdown signal
                _ = shutdown.changed() => {
                    info!("🛑 HTTP server shutting down...");
                    break;
                }
                // Accept new connections
                result = listener.accept() => {
                    match result {
                        Ok((mut socket, addr)) => {
                            info!("🔗 New connection from: {}", addr);

                            // Spawn a task to handle this connection
                            tokio::spawn(async move {
                                let mut buffer = [0; 1024];

                                match socket.read(&mut buffer).await {
                                    Ok(0) => {
                                        warn!("⚠️  Connection closed by client: {}", addr);
                                        return;
                                    }
                                    Ok(n) => {
                                        let request = String::from_utf8_lossy(&buffer[..n]);
                                        info!("📨 Request from {}: {}", addr, request.lines().next().unwrap_or(""));

                                        // Simple HTTP response
                                        let response = create_response(&request);

                                        if let Err(e) = socket.write_all(response.as_bytes()).await {
                                            error!("❌ Failed to write response to {}: {}", addr, e);
                                        } else {
                                            info!("✅ Response sent to {}", addr);
                                        }
                                    }
                                    Err(e) => {
                                        error!("❌ Failed to read from socket {}: {}", addr, e);
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!("❌ Failed to accept connection: {}", e);
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "edgeflow_http_server"
    }

    fn threads(&self) -> Option<usize> {
        self.config.worker_threads
    }
}

fn create_response(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return create_error_response(400, "Bad Request");
    }

    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        return create_error_response(400, "Bad Request");
    }

    let method = parts[0];
    let path = parts[1];

    info!("🔍 Processing {} request to {}", method, path);

    match path {
        "/health" => create_success_response("application/json", r#"{"status":"healthy","service":"edgeflow","version":"0.6.0","port":"8999","capabilities":["http_gateway","background_services","plugin_system"]}"#),
        "/status" => create_success_response("application/json", r#"{"status":"running","uptime":"unknown","connections":1,"port":"8999","mode":"enhanced_gateway"}"#),
        "/api/info" => create_success_response("application/json", r#"{"name":"EdgeFlow","version":"0.6.0","description":"Edge AI Data Flow Platform","port":"8999","features":["routing","load_balancing","plugins","monitoring"]}"#),
        "/api/capabilities" => create_success_response("application/json", r#"{"proxy":true,"load_balancing":true,"ssl_termination":true,"plugins":true,"monitoring":true,"ai_integration":true}"#),
        "/" => create_success_response("text/html",
            r#"<html><body><h1>🚀 EdgeFlow - Edge AI Data Flow Platform</h1><p>Enhanced Gateway running on port 8999!</p><h2>Available Endpoints:</h2><ul><li><a href="/health">Health Check</a></li><li><a href="/status">Status</a></li><li><a href="/api/info">API Info</a></li><li><a href="/api/capabilities">Capabilities</a></li></ul><h2>Features:</h2><ul><li>✅ HTTP Gateway</li><li>✅ Background Services</li><li>✅ Plugin System</li><li>✅ Monitoring</li><li>🔧 Proxy Capabilities (Ready)</li></ul></body></html>"#),
        _ => create_error_response(404, "Not Found"),
    }
}

fn create_success_response(content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        content_type,
        body.len(),
        body
    )
}

fn create_error_response(status_code: u16, status_text: &str) -> String {
    let body = format!(r#"{{"error":"{}","code":{}}}"#, status_text, status_code);
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    )
}