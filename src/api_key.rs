#![cfg(not(feature = "readonly"))]

use std::env::VarError;

/// API Key for the remote service.
/// This is required for PUT/POST requests.
///
/// If you believe you need an API Key,
/// contact the maintainer of the remote service you are using.
pub struct ApiKey(String);

impl ApiKey {
    /// Attempts to load an API Key from the environment.
    /// If the environment variable is not set, this will return an error.
    pub fn from_env(env_name: &'static str) -> Result<ApiKey, VarError> {
        Ok(Self(std::env::var(env_name)?))
    }

    pub fn new(key: String) -> Self {
        Self(key)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_api_key() {
        let key = ApiKey::new("test".to_string());
        assert_eq!(key.get(), "test");
    }

    #[test]
    fn test_from_env_ok() {
        std::env::set_var("CODES__TEST_API_KEY", "test");

        assert!(ApiKey::from_env("CODES__TEST_API_KEY").is_ok())
    }

    #[test]
    fn test_from_env_err() {
        assert!(ApiKey::from_env("CODES__TEST_API_KEY_NOT_SET").is_err())
    }
}
