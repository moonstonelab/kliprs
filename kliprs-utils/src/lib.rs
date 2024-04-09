use std::env;
use std::path::PathBuf;

pub fn get_config_file_path(file_path: &str) -> PathBuf {
    if let Ok(env_config_file) = env::var("CLIPBOARD_CONFIG_FILE") {
        PathBuf::from(env_config_file)
    } else {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir,
            None => {
                eprintln!("Failed to determine platform's config directory");
                return PathBuf::from(file_path);
            }
        };
        config_dir.join(file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_config_file_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.json");
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, r#"{{"buffer_size": 10, "retention_time": 7}}"#).unwrap();

        env::set_var("CLIPBOARD_CONFIG_FILE", file_path.to_str().unwrap());

        let path = get_config_file_path("config.json");

        assert_eq!(path, file_path);
    }
}