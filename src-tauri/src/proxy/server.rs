//! HTTP proxy server implementation

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::config::ProxyProfile;
use crate::stats::StatsCollector;

/// Proxy server state shared across handlers
#[derive(Clone)]
pub struct ProxyState {
    pub profile: ProxyProfile,
    pub stats: Arc<StatsCollector>,
    pub log_sender: broadcast::Sender<LogEntry>,
}

/// Log entry for real-time logging
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Running proxy server handle
pub struct ProxyServer {
    addr: SocketAddr,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    state: Arc<ProxyState>,
}

impl ProxyServer {
    /// Create a new proxy server
    pub fn new(profile: ProxyProfile) -> (Self, broadcast::Receiver<LogEntry>) {
        let (log_tx, log_rx) = broadcast::channel(1000);
        let stats = Arc::new(StatsCollector::new());

        let state = Arc::new(ProxyState {
            profile,
            stats,
            log_sender: log_tx,
        });

        (
            Self {
                addr: "127.0.0.1:0".parse().unwrap(),
                shutdown_tx: None,
                state,
            },
            log_rx,
        )
    }

    /// Start the proxy server on the configured port
    pub async fn start(&mut self) -> anyhow::Result<u16> {
        let addr: SocketAddr = format!("127.0.0.1:{}", self.state.profile.local_port)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;

        let app = Router::new()
            .fallback(crate::proxy::handler::proxy_handler)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            )
            .with_state(self.state.clone());

        let listener = tokio::net::TcpListener::bind(addr).await?;
        self.addr = listener.local_addr()?;

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        let state = self.state.clone();
        let _log_tx = state.log_sender.clone();

        // Log server start
        let _ = state.log_sender.send(LogEntry {
            timestamp: chrono::Local::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: format!(
                "Proxy server started on port {}",
                state.profile.local_port
            ),
            details: Some(serde_json::json!({
                "target": state.profile.target_base_url,
                "max_retries": state.profile.max_retries,
            })),
        });

        tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .ok();
        });

        Ok(self.addr.port())
    }

    /// Stop the proxy server
    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }

    /// Get the server state
    pub fn state(&self) -> Arc<ProxyState> {
        self.state.clone()
    }
}
