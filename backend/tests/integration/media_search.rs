use axum::{body::Body, http::{Method, Request, StatusCode}};
use http_body_util::BodyExt;
use serde_json::Value;

use super::support::IntegrationTestApp;

#[tokio::test]
async fn search_filters_by_tags_and_attributes() {
    let app = IntegrationTestApp::new().await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?tags=sunset,coast&attributes[rating]=5")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("json");
    let items = json["items"].as_array().expect("items array");
    assert_eq!(items.len(), 1, "AND filters should narrow to one match");
    assert_eq!(json["total"], 1);

    let item = &items[0];
    let tags = item["tags"].as_array().expect("tags array");
    assert!(tags.iter().any(|t| t["name"] == "sunset"));
    assert!(tags.iter().any(|t| t["name"] == "coast"));
    assert!(tags.iter().any(|t| t["name"] == "location" && t["value"] == "okinawa"));
}

#[tokio::test]
async fn search_accepts_keyvalue_tag_name_filters() {
    let app = IntegrationTestApp::new().await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?tags=rating")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("json");
    assert!(json["total"].as_u64().unwrap_or(0) >= 1);
}

#[tokio::test]
async fn search_validation_errors_surface_contract_envelope() {
    let app = IntegrationTestApp::new().await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?page=0")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["error"]["code"], "VALIDATION_FAILED");
}

#[tokio::test]
async fn search_paginates_results() {
    let app = IntegrationTestApp::new().await;

    let first = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?page=1&pageSize=1")
        .body(Body::empty())
        .expect("request");
    let first_response = app.request(first).await;
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_json: Value = serde_json::from_slice(
        &first_response.into_body().collect().await.expect("body").to_bytes(),
    )
    .expect("json");
    assert_eq!(first_json["page"], 1);

    let second = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?page=2&pageSize=1")
        .body(Body::empty())
        .expect("request");
    let second_response = app.request(second).await;
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_json: Value = serde_json::from_slice(
        &second_response.into_body().collect().await.expect("body").to_bytes(),
    )
    .expect("json");
    assert_eq!(second_json["page"], 2);
    assert_ne!(
        first_json["items"][0]["id"],
        second_json["items"][0]["id"],
        "pagination should move to next item"
    );
}
