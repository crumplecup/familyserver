use crate::prelude::*;
use shared::prelude::*;
use tracing::info;

#[tokio::test]
async fn local_create_user() {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    let test = TestClient::new();
    // let pass = std::env::var("ADMIN_PASSWORD").unwrap();
    let pass = "password";
    let name = "admin";
    let user = User::new(name, &pass);
    let usr = serde_json::json!({ "username": "admin", "password_hash": "password" });
    info!(
        "Sending local request to create user {}.",
        &user.username_ref()
    );

    let res = test
        .client()
        // Use the returned application address
        .post(&format!("{}/users", LOCAL))
        .json(&usr)
        .send()
        .await
        .expect("Failed to execute request.");

    info!("Response {:#?}.", res.text().await);
    // let res = test
    //     .client()
    //     // Use the returned application address
    //     .post(&format!("{}/users", LOCAL))
    //     .json(&user)
    //     .send()
    //     .await
    //     .expect("Failed to execute request.");
    // assert_eq!(&res.status(), &reqwest::StatusCode::CREATED);
    // let usr = res.json::<User>().await.unwrap();
    // assert_eq!(&user, &usr);
}

#[tokio::test]
async fn remote_create_user() {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    let test = TestClient::new();
    // let pass = std::env::var("ADMIN_PASSWORD").unwrap();
    let pass = "password";
    let name = "admin";
    let user = User::new(name, &pass);
    info!(
        "Sending remote request to create user {}.",
        &user.username_ref()
    );

    let res = test
        .client()
        // Use the returned application address
        .post(&format!("{}/users", HOST))
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");

    // info!("Response {:#?}.", res.text().await);
    // let res = test
    //     .client()
    //     // Use the returned application address
    //     .post(&format!("{}/users", LOCAL))
    //     .json(&user)
    //     .send()
    //     .await
    //     .expect("Failed to execute request.");
    assert_eq!(&res.status(), &reqwest::StatusCode::CREATED);
    let usr = res.json::<User>().await.unwrap();
    assert_eq!(&user, &usr);
}
