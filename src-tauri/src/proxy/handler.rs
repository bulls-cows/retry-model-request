//! Request handler with retry logic

use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode, Uri},
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
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path();

    // Build target URL
    let target_url = format!("{}{}", state.profile.target_base_url, path);

    // Check if streaming request
    let is_streaming = is_streaming_request(&request);

    // Log incoming request
    let _ = state.log_sender.send(LogEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        level: "INFO".to_string(),
        message: format!("{} {}", method, path),
        details: Some(serde_json::json!({
            "target": target_url,
            "streaming": is_streaming,
        })),
    });

    // Create HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .unwrap();

    // Handle streaming vs regular requests
    if is_streaming {
        handle_streaming_request(state, client, request, target_url).await
    } else {
        handle_regular_request(state, client, request, target_url).await
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

    // Check body for stream field
    if let Ok(body_bytes) = axum::body::to_bytes(request.body(), 1024) {
        if let Ok(body_str) = std::str::from_utf8(&body_bytes) {
            if body_str.contains("\"stream\":true") {
                return true;
            }
        }
    }

    false
}

/// Handle regular (non-streaming) request with retry
async fn handle_regular_request(
    state: std::sync::Arc<ProxyState>,
    client: Client,
    request: Request,
    target_url: String,
) -> Response {
    let method = request.method().clone();
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(&body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
                .into_response();
        }
    };

    let mut attempt = 0;
    let max_retries = state.profile.max_retries;

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
                        details: None,
                    });

                    // Update stats
                    state.stats.record_retry();

                    tokio::time::sleep(Duration::from_millis(state.profile.retry_delay_ms)).await;
                    continue;
                }

                // Forward response
                let mut response_builder = Response::builder().status(status);
                for (name, value) in response.headers() {
                    response_builder = response_builder.header(name, value);
                }

                let body_bytes = response.bytes().await.unwrap_or_default();

                // Log success
                let _ = state.log_sender.send(LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    level: "INFO".to_string(),
                    message: format!("Response: {} {}", status, target_url),
                    details: Some(serde_json::json!({
                        "status": status.as_u16(),
                        "size": body_bytes.len(),
                        "attempt": attempt + 1,
                    })),
                });

                // Update stats
                state.stats.record_success();

                return response_builder
                    .body(Body::from(body_bytes))
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
                        details: None,
                    });

                    state.stats.record_retry();
                    tokio::time::sleep(Duration::from_millis(state.profile.retry_delay_ms)).await;
                    continue;
                }

                // Log error
                let _ = state.log_sender.send(LogEntry {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    level: "ERROR".to_string(),
                    message: format!("Request failed: {}", e),
                    details: None,
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
    client: Client,
    request: Request,
    target_url: String,
) -> Response {
    let (parts, body) = request.into_parts();
    let body_bytes = match axum::body::to_bytes(&body, 100 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to read body: {}", e),
            )
                .into_response();
        }
    };

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
            let mut response_builder = Response::builder()
                .status(status)
                .header("content-type", "text/event-stream")
                .header("cache-control", "no-cache")
                .header("connection", "keep-alive");

            // Create streaming body
            let stream = response.bytes_stream().map(|result| match result {
                Ok(bytes) => Ok(bytes),
                Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
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
                details: None,
            });

            state.stats.record_failure();

            (StatusCode::BAD_GATEWAY, format!("Proxy error: {}", e)).into_response()
        }
    }
}
