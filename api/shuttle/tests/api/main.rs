mod health;
mod health_local;
mod helpers;
mod user;

pub mod prelude {
    pub use crate::helpers::{TestApp, TestClient, HOST, LOCAL};
}
