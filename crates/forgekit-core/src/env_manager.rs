//! Environment variable management module
//!
//! This module provides functionality to manage environment variables
//! for different build configurations (dev, staging, production).

use crate::error::ForgeKitError;
use std::collections::HashMap;
use std::path::Path;

/// Environment variable manager
#[derive(Debug, Clone)]
pub struct EnvManager {
    env_vars: HashMap<String, String>,
}

impl EnvManager {
    /// Create a new empty environment manager
    pub fn new() -> Self {
        Self {
            env_vars: HashMap::new(),
        }
    }

    /// Load environment variables from a .env file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the .env file
    ///
    /// # Returns
    ///
    /// A new `EnvManager` with loaded variables
    pub fn load_from_file(path: &Path) -> Result<Self, ForgeKitError> {
        let mut manager = Self::new();

        if !path.exists() {
            return Ok(manager);
        }

        let content = std::fs::read_to_string(path)?;
        manager.parse_env_content(&content)?;

        Ok(manager)
    }

    /// Load environment variables for a specific environment
    ///
    /// # Arguments
    ///
    /// * `env` - Environment name (e.g., "dev", "staging", "prod")
    /// * `base_path` - Base path to look for .env files
    ///
    /// # Returns
    ///
    /// A new `EnvManager` with loaded variables
    pub fn load_for_environment(env: &str, base_path: &Path) -> Result<Self, ForgeKitError> {
        let mut manager = Self::new();

        // Load base .env file first
        let base_env_path = base_path.join(".env");
        if base_env_path.exists() {
            let content = std::fs::read_to_string(&base_env_path)?;
            manager.parse_env_content(&content)?;
        }

        // Load environment-specific .env file
        let env_specific_path = base_path.join(format!(".env.{}", env));
        if env_specific_path.exists() {
            let content = std::fs::read_to_string(&env_specific_path)?;
            manager.parse_env_content(&content)?;
        }

        Ok(manager)
    }

    /// Parse environment file content
    fn parse_env_content(&mut self, content: &str) -> Result<(), ForgeKitError> {
        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse KEY=VALUE format
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                // Remove quotes if present
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value[1..value.len() - 1].to_string()
                } else {
                    value
                };

                self.env_vars.insert(key, value);
            }
        }

        Ok(())
    }

    /// Get an environment variable
    ///
    /// # Arguments
    ///
    /// * `key` - The variable name
    ///
    /// # Returns
    ///
    /// The variable value if it exists
    pub fn get(&self, key: &str) -> Option<&str> {
        self.env_vars.get(key).map(|v| v.as_str())
    }

    /// Get an environment variable or a default value
    pub fn get_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.env_vars
            .get(key)
            .map(|v| v.as_str())
            .unwrap_or(default)
    }

    /// Set an environment variable
    pub fn set(&mut self, key: String, value: String) {
        self.env_vars.insert(key, value);
    }

    /// Check if a variable exists
    pub fn contains(&self, key: &str) -> bool {
        self.env_vars.contains_key(key)
    }

    /// Get all environment variables
    pub fn all(&self) -> &HashMap<String, String> {
        &self.env_vars
    }

    /// Interpolate variables in a string
    ///
    /// Replaces ${VAR_NAME} or $VAR_NAME with the corresponding value
    ///
    /// # Arguments
    ///
    /// * `value` - The string to interpolate
    ///
    /// # Returns
    ///
    /// The interpolated string
    pub fn interpolate(&self, value: &str) -> Result<String, ForgeKitError> {
        let mut result = value.to_string();

        // Replace ${VAR_NAME} patterns
        for (key, val) in &self.env_vars {
            let pattern = format!("${{{}}}", key);
            result = result.replace(&pattern, val);
        }

        // Replace $VAR_NAME patterns (word boundaries)
        for (key, val) in &self.env_vars {
            let pattern = format!("${}", key);
            // Only replace if followed by non-word character or end of string
            let mut new_result = String::new();
            let mut chars = result.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == '$' {
                    let mut var_name = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if var_name == *key {
                        new_result.push_str(val);
                    } else if !var_name.is_empty() {
                        new_result.push('$');
                        new_result.push_str(&var_name);
                    } else {
                        new_result.push('$');
                    }
                } else {
                    new_result.push(ch);
                }
            }
            result = new_result;
        }

        Ok(result)
    }

    /// Validate that required variables are set
    ///
    /// # Arguments
    ///
    /// * `required_vars` - List of required variable names
    ///
    /// # Returns
    ///
    /// Error if any required variable is missing
    pub fn validate_required(&self, required_vars: &[&str]) -> Result<(), ForgeKitError> {
        let missing: Vec<&str> = required_vars
            .iter()
            .filter(|var| !self.contains(var))
            .copied()
            .collect();

        if !missing.is_empty() {
            return Err(ForgeKitError::InvalidConfig(format!(
                "Missing required environment variables: {}",
                missing.join(", ")
            )));
        }

        Ok(())
    }

    /// Save environment variables to a file
    pub fn save_to_file(&self, path: &Path) -> Result<(), ForgeKitError> {
        let mut content = String::new();
        for (key, value) in &self.env_vars {
            content.push_str(&format!("{}={}\n", key, value));
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl Default for EnvManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_env_manager_creation() {
        let manager = EnvManager::new();
        assert!(manager.all().is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut manager = EnvManager::new();
        manager.set("KEY".to_string(), "value".to_string());
        assert_eq!(manager.get("KEY"), Some("value"));
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let env_file = temp_dir.path().join(".env");
        std::fs::write(
            &env_file,
            "KEY1=value1\nKEY2=value2\n# Comment\nKEY3=\"quoted value\"",
        )
        .unwrap();

        let manager = EnvManager::load_from_file(&env_file).unwrap();
        assert_eq!(manager.get("KEY1"), Some("value1"));
        assert_eq!(manager.get("KEY2"), Some("value2"));
        assert_eq!(manager.get("KEY3"), Some("quoted value"));
    }

    #[test]
    fn test_load_for_environment() {
        let temp_dir = TempDir::new().unwrap();
        let base_env = temp_dir.path().join(".env");
        let dev_env = temp_dir.path().join(".env.dev");

        std::fs::write(&base_env, "BASE_VAR=base_value").unwrap();
        std::fs::write(&dev_env, "DEV_VAR=dev_value").unwrap();

        let manager = EnvManager::load_for_environment("dev", temp_dir.path()).unwrap();
        assert_eq!(manager.get("BASE_VAR"), Some("base_value"));
        assert_eq!(manager.get("DEV_VAR"), Some("dev_value"));
    }

    #[test]
    fn test_interpolate_braces() {
        let mut manager = EnvManager::new();
        manager.set("NAME".to_string(), "World".to_string());
        let result = manager.interpolate("Hello ${NAME}").unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_interpolate_dollar() {
        let mut manager = EnvManager::new();
        manager.set("NAME".to_string(), "World".to_string());
        let result = manager.interpolate("Hello $NAME").unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_validate_required() {
        let mut manager = EnvManager::new();
        manager.set("KEY1".to_string(), "value1".to_string());

        assert!(manager.validate_required(&["KEY1"]).is_ok());
        assert!(manager.validate_required(&["KEY1", "KEY2"]).is_err());
    }

    #[test]
    fn test_save_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let env_file = temp_dir.path().join(".env");

        let mut manager = EnvManager::new();
        manager.set("KEY1".to_string(), "value1".to_string());
        manager.set("KEY2".to_string(), "value2".to_string());

        manager.save_to_file(&env_file).unwrap();

        let loaded = EnvManager::load_from_file(&env_file).unwrap();
        assert_eq!(loaded.get("KEY1"), Some("value1"));
        assert_eq!(loaded.get("KEY2"), Some("value2"));
    }

    #[test]
    fn test_contains() {
        let mut manager = EnvManager::new();
        manager.set("KEY".to_string(), "value".to_string());
        assert!(manager.contains("KEY"));
        assert!(!manager.contains("MISSING"));
    }

    #[test]
    fn test_get_or() {
        let manager = EnvManager::new();
        assert_eq!(manager.get_or("MISSING", "default"), "default");
    }
}
