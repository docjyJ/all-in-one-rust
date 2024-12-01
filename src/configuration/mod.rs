pub use crate::configuration::models::{BackupMode, Configuration};
pub use crate::configuration::read::RefConfiguration;
pub use crate::configuration::write::MutConfiguration;
use std::fs;
use tokio::sync::{OnceCell, RwLock};
use tracing::error;

mod models;
mod read;
mod write;

pub struct StateConfiguration(RwLock<Configuration>);

static STATE: OnceCell<StateConfiguration> = OnceCell::const_new();
const FILE: &str = "config.json";

fn read_file() -> serde_json::Result<Configuration> {
    match fs::read(FILE) {
        Ok(data) => serde_json::from_slice(&data),
        Err(data) => Err(serde_json::error::Error::io(data)),
    }
}

impl StateConfiguration {
    fn new() -> Self {
        let config = read_file().unwrap_or_else(|e| {
            error!("Failed to read configuration file: {}", e);
            Configuration::default()
        });
        Self(RwLock::new(config))
    }

    async fn instance() -> &'static Self {
        STATE
            .get_or_init(|| async { StateConfiguration::new() })
            .await
    }

    pub async fn instance_ref() -> RefConfiguration {
        RefConfiguration::new(Self::instance().await.0.read().await)
    }

    pub async fn instance_mut() -> MutConfiguration {
        MutConfiguration::new(Self::instance().await.0.write().await, FILE)
    }

    pub async fn set_password(password: String) {
        Self::instance_mut().await.password = password;
    }

    pub async fn set_aio_token(token: String) {
        Self::instance_mut().await.aio_token = Some(token);
    }

    pub async fn clear_aio_token() {
        Self::instance_mut().await.aio_token = None;
    }

    pub async fn test_password(password: &str) -> bool {
        Self::instance_ref().await.password == password
    }

    pub async fn test_token(token: &str) -> bool {
        if let Some(aio_token) = Self::instance_ref().await.aio_token.as_deref() {
            aio_token == token
        } else {
            false
        }
    }

    pub async fn set_backup_mode(backup_mode: BackupMode) {
        Self::instance_mut().await.backup_mode = backup_mode;
    }
}
