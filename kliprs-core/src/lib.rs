use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Mutex;
use std::sync::Once;

lazy_static! {
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub buffer_size: u8,
    pub retention_time: u8, // in days
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_size: 100,
            retention_time: 30,
        }
    }
}

impl Config {
    pub fn new(buffer_size: u8, retention_time: u8) -> Self {
        Self {
            buffer_size,
            retention_time,
        }
    }

    pub fn update_from_env(&mut self) {
        if let Ok(env_buffer_size) = env::var("CLIPBOARD_BUFFER_SIZE") {
            if let Ok(env_buffer_size) = env_buffer_size.parse::<u8>() {
                self.buffer_size = env_buffer_size;
            }
        }
        if let Ok(env_retention_time) = env::var("CLIPBOARD_RETENTION_TIME") {
            if let Ok(env_retention_time) = env_retention_time.parse::<u8>() {
                self.retention_time = env_retention_time;
            }
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.exists() {
            let mut config = Self::default();
            config.update_from_env();
            return Ok(config);
        }

        let file = std::fs::File::open(path).unwrap();
        let file = std::io::BufReader::new(file);
        let mut config: Config = serde_json::from_reader(file).unwrap();

        config.update_from_env();

        Ok(config)
    }
}

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        let path = Path::new("config.json");
        match Config::from_file(&path) {
            Ok(config) => {
                let mut global_config = CONFIG.lock().unwrap();
                *global_config = Some(config);
            }
            Err(e) => eprintln!("Failed to read item from file: {}", e),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, r#"{{"buffer_size": 10, "retention_time": 7}}"#).unwrap();

        let config = Config::from_file(&file_path).unwrap();

        assert_eq!(config.buffer_size, 10);
        assert_eq!(config.retention_time, 7);
    }

    #[test]
    fn test_from_file_with_env() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, r#"{{"buffer_size": 10, "retention_time": 7}}"#).unwrap();

        env::set_var("CLIPBOARD_BUFFER_SIZE", "20");
        env::set_var("CLIPBOARD_RETENTION_TIME", "30");

        let config = Config::from_file(&file_path).unwrap();

        assert_eq!(config.buffer_size, 20);
        assert_eq!(config.retention_time, 30);
    }
}
