use std::collections::HashMap;
use std::env;

/// Trait for providing configuration values
/// Internal implementation detail - not exposed in public API
pub trait ConfigProvider: Send + Sync {
    fn get_var(&self, key: &str) -> Option<String>;
}

/// Production implementation using real environment variables
pub struct EnvConfigProvider;

impl ConfigProvider for EnvConfigProvider {
    fn get_var(&self, key: &str) -> Option<String> {
        env::var(key).ok()
    }
}

/// Test implementation using in-memory HashMap
/// Only available when building tests or with test-utils feature
pub struct TestConfigProvider {
    pub values: HashMap<String, String>,
}

impl ConfigProvider for TestConfigProvider {
    fn get_var(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}
