use crate::configuration::Configuration;
use std::ops::{Deref, DerefMut};
use tokio::sync::RwLockWriteGuard;
use tracing::error;

pub struct MutConfiguration(RwLockWriteGuard<'static, Configuration>, &'static str);

impl MutConfiguration {
    pub fn new(inner: RwLockWriteGuard<'static, Configuration>, file: &'static str) -> Self {
        Self(inner, file)
    }

    pub fn commit(self) {
        if let Err(e) = self.0.write(self.1) {
            error!("Error writing config file: {}", e)
        }
    }
}

impl Deref for MutConfiguration {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MutConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
