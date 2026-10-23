/// Configuration API helpers for loading, validating, and managing runtime configuration.
use std::collections::HashMap;
pub struct ConfigValidator {
    /// Required keys that must be present in the configuration
    pub required_keys: Vec<String>,
    /// Allowed values for specific configuration fields
    pub value_restrictions: HashMap<String, String>,
}

/// Validates a configuration structure and returns errors if any.
pub fn validate_config(config: &HashMap<String, String>) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for key in required_keys {
        if config.get(&key).is_none() {
            errors.push(format!("Missing required configuration key: {}", key));
        }
    }

    // Check value restrictions     
    for (restricted_key, allowed_values) in &value_restrictions {
        if let Some(value) = config.get(restricted_key) {
            if !allowed_values.contains_str(value) {
                errors.push(format!("Invalid value for {}: {}", restricted_key, value));
            }
        }
    }

    Ok::<_, _>
    (errors);
}

// "}, "execute: true"]"}]"}]