use crate::configuration::Configuration;
use std::ops::Deref;
use tokio::sync::RwLockReadGuard;

pub struct RefConfiguration(RwLockReadGuard<'static, Configuration>);

impl RefConfiguration {
    pub fn new(guard: RwLockReadGuard<'static, Configuration>) -> RefConfiguration {
        RefConfiguration(guard)
    }

    pub fn get_domain(&self) -> Option<String> {
        self.0.domain.clone()
    }
}

impl Deref for RefConfiguration {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
