use secrecy::ExposeSecret;
use secrecy::Secret;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use sqlx::{postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool};
use tracing::info;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok();
        let username = std::env::var("DB_USERNAME")?;
        let password = std::env::var("DB_PASSWORD")?.into();
        let host = std::env::var("DB_HOST")?;
        let port = 5432;
        let database_name = std::env::var("DB_NAME")?;
        let require_ssl = false;
        Ok(Self {
            username,
            password,
            host,
            port,
            database_name,
            require_ssl,
        })
    }
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
    // Renamed from `connection_string_without_db`
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            // Try an encrypted connection, fallback to unencrypted if it fails
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    // Renamed from `connection_string`
    // switched from mut options to clone... but why?
    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.database_name);
        options
            .clone()
            .log_statements(tracing::log::LevelFilter::Trace);
        options
    }

    pub async fn create_db(&self) {
        info!("Creating database {}.", &self.database_name);
        let mut connection = PgConnection::connect_with(&self.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(&*format!(r#"CREATE DATABASE "{}";"#, self.database_name))
            .await
            .expect("Failed to create database.");
    }

    pub async fn migrate_db(&self) {
        let connection_pool = PgPool::connect_with(self.with_db())
            .await
            .expect("Failed to connect to Postgres.");
        info!("Migrating database.");
        sqlx::migrate!("../shuttle/migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
    }

    pub async fn delete_db(&self) {
        info!("Deleting database {}.", &self.database_name);
        let mut connection = PgConnection::connect_with(&self.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(&*format!(r#"DROP DATABASE "{}";"#, self.database_name))
            .await
            .expect("Failed to delete database.");
    }

    pub async fn configure_database(&self) -> PgPool {
        info!("Configuring database {}.", &self.database_name);
        let mut connection = PgConnection::connect_with(&self.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(&*format!(r#"CREATE DATABASE "{}";"#, self.database_name))
            .await
            .expect("Failed to create database.");

        let connection_pool = PgPool::connect_with(self.with_db())
            .await
            .expect("Failed to connect to Postgres.");
        info!("Migrating database.");
        sqlx::migrate!("../shuttle/migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
        connection_pool
    }

    pub fn get_connection_pool(&self) -> PgPool {
        info!("Creating connection pool.");
        PgPoolOptions::new().connect_lazy_with(self.with_db())
    }
}

// pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
//     info!("Configuring database {}.", &config.database_name);
//     let mut connection = PgConnection::connect_with(&config.without_db())
//         .await
//         .expect("Failed to connect to Postgres");
//     connection
//         .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
//         .await
//         .expect("Failed to create database.");
//
//     let connection_pool = PgPool::connect_with(config.with_db())
//         .await
//         .expect("Failed to connect to Postgres.");
//     info!("Migrating database.");
//     sqlx::migrate!("../shuttle/migrations")
//         .run(&connection_pool)
//         .await
//         .expect("Failed to migrate the database");
//     connection_pool
// }
//
// pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
//     info!("Creating connection pool.");
//     PgPoolOptions::new().connect_lazy_with(configuration.with_db())
// }
