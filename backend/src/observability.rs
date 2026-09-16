use std::net::SocketAddr;
use std::time::Instant;

use axum::extract::{ConnectInfo, MatchedPath, Request};
use axum::http::{HeaderMap, Version, header};
use axum::middleware::Next;
use axum::response::Response;
use tracing::{Instrument, field, info, info_span};
use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .with_ansi(false)
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .init();
}

pub async fn access_log(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_owned();
    let query = request.uri().query().map(str::to_owned);
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .map(str::to_owned);
    let protocol_version = protocol_version(request.version());
    let (server_address, server_port) = server(request.headers());
    let peer = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|connect_info| connect_info.0);
    let peer_address = peer.map(|address| address.ip().to_string());
    let peer_port = peer.map(|address| address.port());
    let span_name = route
        .as_ref()
        .map_or_else(|| method.clone(), |route| format!("{method} {route}"));

    let span = info_span!(
        "http.server.request",
        "otel.name" = span_name,
        "otel.kind" = "server",
        "otel.status_code" = field::Empty,
        "http.request.method" = method,
        "url.path" = path,
        "url.query" = query.as_deref(),
        "url.scheme" = "http",
        "http.route" = route.as_deref(),
        "server.address" = server_address.as_deref(),
        "server.port" = server_port,
        "network.peer.address" = peer_address.as_deref(),
        "network.peer.port" = peer_port,
        "network.protocol.version" = protocol_version,
        "http.response.status_code" = field::Empty,
        "error.type" = field::Empty,
    );

    async move {
        let started_at = Instant::now();
        let response = next.run(request).await;
        let status = response.status().as_u16();
        let duration_ms = started_at.elapsed().as_secs_f64() * 1_000.0;
        let error_type = response
            .status()
            .is_server_error()
            .then(|| status.to_string());

        let current_span = tracing::Span::current();
        current_span.record("http.response.status_code", status);
        if let Some(error_type) = &error_type {
            current_span.record("otel.status_code", "ERROR");
            current_span.record("error.type", error_type.as_str());
        }

        info!(
            target: "access_log",
            {
                "event.name" = "http.server.request.completed",
                "http.request.method" = method,
                "url.path" = path,
                "url.query" = query.as_deref(),
                "url.scheme" = "http",
                "http.route" = route.as_deref(),
                "server.address" = server_address.as_deref(),
                "server.port" = server_port,
                "network.peer.address" = peer_address.as_deref(),
                "network.peer.port" = peer_port,
                "network.protocol.version" = protocol_version,
                "http.response.status_code" = status,
                "error.type" = error_type.as_deref(),
                duration_ms,
            },
            "HTTP request completed"
        );

        response
    }
    .instrument(span)
    .await
}

fn protocol_version(version: Version) -> &'static str {
    match version {
        Version::HTTP_09 => "0.9",
        Version::HTTP_10 => "1.0",
        Version::HTTP_11 => "1.1",
        Version::HTTP_2 => "2",
        Version::HTTP_3 => "3",
        _ => "unknown",
    }
}

fn server(headers: &HeaderMap) -> (Option<String>, Option<u16>) {
    let Some(host) = headers
        .get(header::HOST)
        .and_then(|host| host.to_str().ok())
    else {
        return (None, None);
    };

    match host.parse::<axum::http::uri::Authority>() {
        Ok(authority) => (Some(authority.host().to_owned()), authority.port_u16()),
        Err(_) => (None, None),
    }
}
