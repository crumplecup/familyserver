pub mod configuration;
pub mod get_data;
pub mod health;
pub mod interface;
pub mod state;
pub mod utils;

pub mod prelude {
    pub use crate::configuration::DatabaseSettings;
    pub use crate::health::check;
    pub use crate::state::{AppState, API_VERSION};
    pub use crate::utils::prune_str;
}
