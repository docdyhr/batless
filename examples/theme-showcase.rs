// batless Theme Showcase Example
// This file demonstrates syntax highlighting across various themes

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

/// Configuration struct for the application
#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub port: u16,
    pub enabled: bool,
}

impl Config {
    /// Creates a new configuration with default values
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            port: 8080,
            enabled: true,
        }
    }
}

/// Main application logic
fn main() -> io::Result<()> {
    // Initialize configuration
    let config = Config::new("batless");
    let mut cache: HashMap<String, Vec<u8>> = HashMap::new();

    // Process data
    for i in 0..10 {
        let key = format!("item_{}", i);
        let value = vec![i as u8; i];
        cache.insert(key, value);
    }

    // Print results
    println!("Configuration: {:?}", config);
    println!("Cache size: {}", cache.len());

    // Conditional logic
    if config.enabled {
        process_data(&cache)?;
    } else {
        eprintln!("Processing disabled!");
    }

    Ok(())
}

/// Processes cached data
fn process_data(cache: &HashMap<String, Vec<u8>>) -> io::Result<()> {
    for (key, value) in cache.iter() {
        println!("Key: {}, Value size: {}", key, value.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new("test");
        assert_eq!(config.name, "test");
        assert_eq!(config.port, 8080);
        assert!(config.enabled);
    }
}
