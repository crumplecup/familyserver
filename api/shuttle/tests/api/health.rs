use crate::helpers::{TestClient, HOST};

#[tokio::test]
async fn check() {
    let test = TestClient::new();

    let res = test
        .client()
        // Use the returned application address
        .get(&format!("{}/health", HOST))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(res.status().is_success());
    assert_eq!(api_lib::state::API_VERSION, res.headers()["family_server"]);
}
