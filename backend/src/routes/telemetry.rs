use std::time::Duration;

use axum::{
    extract::MatchedPath,
    http::Request,
};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse};
use tracing::{field, Span};

#[derive(Clone)]
pub struct HttpMakeSpan;

impl<B> MakeSpan<B> for HttpMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let method = request.method().clone();
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(|path| path.as_str())
            .unwrap_or_else(|| request.uri().path());

        let span = tracing::info_span!(
            "http_request",
            http.request.method = %method,
            http.route = %matched_path,
            url.path = request.uri().path(),
            url.query = field::Empty,
            http.response.status_code = field::Empty,
            http.latency_ms = field::Empty
        );

        if let Some(query) = request.uri().query() {
            span.record("url.query", &field::display(query));
        }

        span
    }
}

#[derive(Clone)]
pub struct LogOnRequest;

impl<B> OnRequest<B> for LogOnRequest {
    fn on_request(&mut self, request: &Request<B>, span: &Span) {
        tracing::info!(
            parent: span,
            http.request.method = %request.method(),
            http.route = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|path| path.as_str())
                        .unwrap_or_else(|| request.uri().path()),
            url.path = %request.uri().path(),
            "HTTP request received: {} {}",
            request.method(),
            request.uri().path()
        );
    }
}

#[derive(Clone)]
pub struct LogOnResponse;

impl<B> OnResponse<B> for LogOnResponse {
    fn on_response(self, response: &axum::http::Response<B>, latency: Duration, span: &Span) {
        let status_code = response.status().as_u16();

        span.record("http.response.status_code", &field::display(status_code));
        span.record("http.latency_ms", &field::display(latency.as_millis()));

        tracing::info!(
            parent: span,
            http.latency_ms = %latency.as_millis(),
            http.response.status_code = %status_code,
            "HTTP request completed with status {} in {} ms",
            status_code,
            latency.as_millis()
        );
    }
}
