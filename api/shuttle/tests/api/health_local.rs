use crate::prelude::*;
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[tokio::test]
async fn check_local() {
    let app = TestApp::new().await;
    let response = app
        .router()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        api_lib::state::API_VERSION,
        response.headers()["family_server"]
    );
}
