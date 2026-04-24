//! Request handler with retry logic

use axum::{
    body::Body,
    extract::{Request, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
};
use reqwest::Client;
use std::time::Duration;
use tokio_stream::StreamExt;

use super::server::{LogEntry, ProxyState};

/// Main proxy handler
pub async fn proxy_handler(
    State(state): State<std::sync::Arc<ProxyState>>,
    request: Request,
) -> Response {
    let uri = request.uri().clone();
    let path = uri.path();

    // Build target URL
    let target_url = format!("{}{}", state.profile.target_base_url, path);

    // Check if streaming request
    let is_streaming = is_streaming_request(&request);

    // Handle streaming vs regular requests
    if is_streaming {
        handle_streaming_request(state, request, target_url).await
    } else {
        handle_regular_request(state, request, target_url).await
    }
}

/// Check if request is a streaming request
fn is_streaming_request(request: &Request) -> bool {
    // Check Accept header
    if let Some(accept) = request.headers().get("accept") {
        if let Ok(accept_str) = accept.to_str() {
            if accept_str.contains("text/event-stream") {
                return true;
            }
        }
    }

    false
}

/// Truncate string to max length
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}... (truncated, {} bytes total)", &s[..max_len], s.len())
    } else {
        s.to_string()
    }
}

/// Format bytes as string, handling JSON specially
fn format_body(bytes: &[u8], max_len: usize) -> String {
    let s = String::from_utf8_lossy(bytes);
    // Try to parse as JSON and pretty print
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
        let pretty = serde_json::to_string_pretty(&json).unwrap_or_else(|_| s.to_string());
        truncate_string(&pretty, max_len)
    } else {
        truncate_string(&s, max_len)
    }
}

/// Handle regular (non-streaming) request with retry
async fn handle_regular_request(
    state: std::sync::Arc<ProxyState>,
    request: Request,
    target_url: String,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let (parts, body) = request.into_parts();

    let body_bytes = match axum::body::to_bytes(body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            let _ = state.log_sender.send(LogEntry {
                timestamp: chrono::Local::now().to_rfc3339(),
                level: "ERROR".to_string(),
                message: format!("Failed to read request body: {}", e),
                details: None,
            });
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
                .into_response();
        }
    };

    // Extract request headers
    let req_headers: serde_json::Map<String, serde_json::Value> = parts
        .headers
        .iter()
        .filter_map(|(k, v)| {
            v.to_str()
                .ok()
                .map(|s| (k.to_string(), serde_json::Value::String(s.to_string())))
        })
        .collect();

    // Log incoming request with details
    let _ = state.log_sender.send(LogEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!(">>> {} {}", method, path),
        details: Some(serde_json::json!({
            "type": "request",
            "method": method.to_string(),
            "path": path,
            "target": target_url,
            "headers": req_headers,
            "body": format_body(&body_bytes, 2000),
        })),
    });

    let mut attempt = 0;
    let max_retries = state.profile.max_retries;

    // Create HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .unwrap();

    loop {
        // Build request
        let mut req_builder = client.request(
            Method::from_bytes(parts.method.as_str().as_bytes()).unwrap(),
            &target_url,
        );

        // Copy headers
        for (name, value) in &parts.headers {
            if name != "host" && name != "content-length" {
                req_builder = req_builder.header(name, value);
            }
        }

        // Set host header
        if let Ok(url) = url::Url::parse(&state.profile.target_base_url) {
            if let Some(host) = url.host_str() {
                req_builder = req_builder.header("host", host);
            }
        }

        req_builder = req_builder.body(body_bytes.clone());

        // Send request
        match req_builder.send().await {
            Ok(response) => {
                let status = response.status();
                let should_retry = state.profile.retry_status_codes.contains(&status.as_u16());

                if should_retry && attempt < max_retries {
                    attempt += 1;
                    let _ = state.log_sender.send(LogEntry {
                        timestamp: chrono::Local::now().to_rfc3339(),
                        level: "WARN".to_string(),
                        message: format!(
                            "Retry {}/{}: {} {}",
                            attempt, max_retries, status, target_url
                        ),
                        details: Some(serde_json::json!({
                            "type": "retry",
                            "attempt": attempt,
                            "max_retries": max_retries,
                            "status": status.as_u16(),
                        })),
                    });

                    // Update stats
                    state.stats.record_retry();

                    tokio::time::sleep(Duration::from_millis(state.profile.retry_delay_ms)).await;
                    continue;
                }

                // Extract response headers
                let resp_headers: serde_json::Map<String, serde_json::Value> = response
                    .headers()
                    .iter()
                    .filter_map(|(k, v)| {
                        v.to_str()
                            .ok()
                            .map(|s| (k.to_string(), serde_json::Value::String(s.to_string())))
                    })
                    .collect();

                // Forward response
                let mut response_builder = Response::builder().status(status);
                for (name, value) in response.headers() {
                    response_builder = response_builder.header(name, value);
                }

                let resp_body = response.bytes().await.unwrap_or_default();

                // Log response with details
                let _ = state.log_sender.send(LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    level: if status.is_success() { "INFO" } else { "WARN" }.to_string(),
                    message: format!("<<< {} {} (attempt {})", status, path, attempt + 1),
                    details: Some(serde_json::json!({
                        "type": "response",
                        "status": status.as_u16(),
                        "headers": resp_headers,
                        "body": format_body(&resp_body, 2000),
                        "size": resp_body.len(),
                        "attempt": attempt + 1,
                    })),
                });

                // Update stats
                state.stats.record_success();

                return response_builder
                    .body(Body::from(resp_body))
                    .unwrap()
                    .into_response();
            }
            Err(e) => {
                let should_retry = e.is_timeout() || e.is_connect();

                if should_retry && attempt < max_retries {
                    attempt += 1;
                    let _ = state.log_sender.send(LogEntry {
                        timestamp: chrono::Local::now().to_rfc3339(),
                        level: "WARN".to_string(),
                        message: format!("Retry {}/{}: {}", attempt, max_retries, e),
                        details: Some(serde_json::json!({
                            "type": "retry_error",
                            "attempt": attempt,
                            "max_retries": max_retries,
                            "error": e.to_string(),
                        })),
                    });

                    state.stats.record_retry();
                    tokio::time::sleep(Duration::from_millis(state.profile.retry_delay_ms)).await;
                    continue;
                }

                // Log error
                let _ = state.log_sender.send(LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    level: "ERROR".to_string(),
                    message: format!("<<< Request failed: {}", e),
                    details: Some(serde_json::json!({
                        "type": "error",
                        "error": e.to_string(),
                        "attempt": attempt + 1,
                    })),
                });

                state.stats.record_failure();

                return (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)).into_response();
            }
        }
    }
}

/// Handle streaming (SSE) request
async fn handle_streaming_request(
    state: std::sync::Arc<ProxyState>,
    request: Request,
    target_url: String,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let (parts, body) = request.into_parts();

    let body_bytes = match axum::body::to_bytes(body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            let _ = state.log_sender.send(LogEntry {
                timestamp: chrono::Local::now().to_rfc3339(),
                level: "ERROR".to_string(),
                message: format!("Failed to read request body: {}", e),
                details: None,
            });
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
                .into_response();
        }
    };

    // Extract request headers
    let req_headers: serde_json::Map<String, serde_json::Value> = parts
        .headers
        .iter()
        .filter_map(|(k, v)| {
            v.to_str()
                .ok()
                .map(|s| (k.to_string(), serde_json::Value::String(s.to_string())))
        })
        .collect();

    // Log incoming streaming request
    let _ = state.log_sender.send(LogEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!(">>> {} {} [STREAMING]", method, path),
        details: Some(serde_json::json!({
            "type": "request",
            "method": method.to_string(),
            "path": path,
            "target": target_url,
            "headers": req_headers,
            "body": format_body(&body_bytes, 2000),
        })),
    });

    // Create HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .unwrap();

    // Build request
    let mut req_builder = client.request(
        Method::from_bytes(parts.method.as_str().as_bytes()).unwrap(),
        &target_url,
    );

    // Copy headers
    for (name, value) in &parts.headers {
        if name != "host" && name != "content-length" {
            req_builder = req_builder.header(name, value);
        }
    }

    if let Ok(url) = url::Url::parse(&state.profile.target_base_url) {
        if let Some(host) = url.host_str() {
            req_builder = req_builder.header("host", host);
        }
    }

    req_builder = req_builder.body(body_bytes);

    match req_builder.send().await {
        Ok(response) => {
            let status = response.status();

            // Log streaming response start
            let _ = state.log_sender.send(LogEntry {
                timestamp: chrono::Local::now().to_rfc3339(),
                level: "INFO".to_string(),
                message: format!("<<< {} {} [STREAMING STARTED]", status, path),
                details: Some(serde_json::json!({
                    "type": "streaming_start",
                    "status": status.as_u16(),
                })),
            });

            let response_builder = Response::builder()
                .status(status)
                .header("content-type", "text/event-stream")
                .header("cache-control", "no-cache")
                .header("connection", "keep-alive");

            // Create streaming body
            let stream = response.bytes_stream().map(|result| match result {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(std::io::Error::other(e)),
            });

            state.stats.record_success();

            response_builder
                .body(Body::from_stream(stream))
                .unwrap()
                .into_response()
        }
        Err(e) => {
            let _ = state.log_sender.send(LogEntry {
                timestamp: chrono::Local::now().to_rfc3339(),
                level: "ERROR".to_string(),
                message: format!("Streaming request failed: {}", e),
                details: Some(serde_json::json!({
                    "type": "error",
                    "error": e.to_string(),
                })),
            });

            state.stats.record_failure();

            (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)).into_response()
        }
    }
}
