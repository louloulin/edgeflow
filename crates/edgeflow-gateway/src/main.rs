use std::sync::Arc;
use anyhow::{Result};
use pingora::server::configuration::ServerConf as PingoraServerConf;
use pingora::server::Server as PingoraServer;
use pingora::proxy::http_proxy_service;
use tokio::sync::broadcast;
use tracing::info;

use edgeflow::config::load as load_config;
use edgeflow::http_server::create_server;
use edgeflow::monitor::init_prometheus;
use edgeflow::plugins::manager::PluginManager;
use edgeflow::proxy_server::https_proxy::Router;
use edgeflow::proxy_server::http_proxy::HttpLB;
use edgeflow::services::BackgroundFunctionService;
use edgeflow::MsgProxy;

pub const SERVER_NAME: &str = "edgeflow";

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Starting EdgeFlow - Edge AI Data Flow Platform (Hybrid Architecture)");

    // Load configuration (this will also parse CLI arguments)
    let config = Arc::new(load_config("./")?);

    println!("✅ Configuration loaded successfully");
    println!("   Service name: {}", config.service_name);
    println!("   Management API: {}", config.server.https_address.as_ref().unwrap_or(&"0.0.0.0:8999".into()));
    println!("   HTTP Proxy: {}", config.server.http_address.as_ref().unwrap_or(&"0.0.0.0:8080".into()));
    println!("   HTTPS Proxy: {}", config.server.https_proxy_address.as_ref().unwrap_or(&"0.0.0.0:8443".into()));

    // Initialize Prometheus metrics
    init_prometheus();

    // Create broadcast channel for inter-service communication
    let (broadcast_tx, _) = broadcast::channel::<MsgProxy>(1000);

    // Configure Pingora server
    let mut pingora_server = PingoraServer::new(None)?;
    let mut server_conf = PingoraServerConf::default();

    // Configure server with proper fields from config
    if let Some(threads) = config.worker_threads {
        server_conf.threads = threads;
    }
    server_conf.daemon = config.daemon;
    server_conf.upgrade_sock = "/tmp/edgeflow_upgrade.sock".to_string();
    server_conf.error_log = None; // Using our own logging system

    // Wrap server_conf in Arc for Pingora
    let server_conf = std::sync::Arc::new(server_conf);

    // Create plugin manager
    let plugin_manager = Arc::new(PluginManager::new());

    // Create services
    let mut services: Vec<Box<dyn pingora::services::Service>> = Vec::new();

    // 1. Management HTTP Server (Enhanced version on port 8999)
    let management_server = create_server(config.clone(), plugin_manager.clone())?;
    services.push(management_server);

    // 2. HTTP Proxy Service (Let's Encrypt challenges and HTTP redirect, port 8080)
    {
        let http_proxy = HttpLB {};
        let mut http_service = http_proxy_service(
            &server_conf,
            http_proxy,
        );
        let http_address = config.server.http_address.as_ref()
            .unwrap_or(&"0.0.0.0:8080".into())
            .to_string();
        http_service.add_tcp(&http_address);
        services.push(Box::new(http_service));
    }

    // 3. HTTPS Proxy Service (Main proxy functionality, port 8443)
    {
        let https_proxy = Router {};
        let mut https_service = http_proxy_service(
            &server_conf,
            https_proxy,
        );
        let https_proxy_address = config.server.https_proxy_address.as_ref()
            .unwrap_or(&"0.0.0.0:8443".into())
            .to_string();

        // For now, use HTTP on port 8443 (TLS will be added later)
        https_service.add_tcp(&https_proxy_address);

        // Set worker threads for proxy services
        if let Some(threads) = config.worker_threads {
            https_service.threads = Some(threads);
        }
        services.push(Box::new(https_service));
    }

    // 4. Background Services (routing, health check, etc.)
    let background_service = BackgroundFunctionService::new(config.clone(), broadcast_tx.clone());
    services.push(Box::new(background_service));

    // Add all services to Pingora server
    pingora_server.add_services(services);

    // Run server
    info!("🚀 Starting {} server with Hybrid Architecture...", SERVER_NAME);
    info!("📊 Management API: {}", config.server.https_address.as_ref().unwrap_or(&"0.0.0.0:8999".into()));
    info!("🌐 HTTP Proxy: {}", config.server.http_address.as_ref().unwrap_or(&"0.0.0.0:8080".into()));
    info!("🔒 HTTPS Proxy: {}", config.server.https_proxy_address.as_ref().unwrap_or(&"0.0.0.0:8443".into()));
    info!("🔧 Background services: routing, health check, certificate management");
    info!("🎯 EdgeFlow Hybrid Architecture is ready!");

    pingora_server.run_forever();
}
