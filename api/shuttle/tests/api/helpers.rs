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
