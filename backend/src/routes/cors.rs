use axum::http::HeaderValue;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

pub fn build_cors_layer(origins: &[String]) -> CorsLayer {
    if origins.is_empty() {
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
    }

    let mut allowed = Vec::new();
    for origin in origins {
        match origin.parse::<HeaderValue>() {
            Ok(value) => allowed.push(value),
            Err(err) => tracing::warn!(%origin, %err, "invalid cors origin, skipping"),
        }
    }

    if allowed.is_empty() {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(allowed))
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
