use api_lib::prelude::*;
use axum::routing::{get, post, Router};
// use once_cell::sync::Lazy;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
// static TRACING: Lazy<()> = Lazy::new(|| {
//     let default_filter_level = "info".to_string();
//     let subscriber_name = "test".to_string();
//     if std::env::var("TEST_LOG").is_ok() {
//         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
//         init_subscriber(subscriber);
//     } else {
//         let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
//         init_subscriber(subscriber);
//     };
// });

pub const HOST: &str = "https://familyserver.shuttleapp.rs";
pub const LOCAL: &str = "http://localhost:8000";

pub struct TestClient {
    client: reqwest::Client,
}

impl TestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }
}

pub struct TestApp {
    router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.create_db().await;
        // settings.configure_database().await;
        let db_pool = settings.get_connection_pool();
        let router = AppState::new(db_pool).app();
        TestApp { router }
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub async fn create_db(&self) {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.create_db().await;
    }

    pub async fn delete_db(&self) {
        let settings = DatabaseSettings::from_env().unwrap();
        settings.delete_db().await;
    }
}
