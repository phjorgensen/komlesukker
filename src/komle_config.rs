use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct KomlesukkerConfig {
    url: String,
    secret: String,
    threshold: Threshold,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Threshold {
    pub very_high: f64,
    pub high: f64,
    pub low: f64,
    pub very_low: f64,
}

impl KomlesukkerConfig {
    pub fn new() -> KomlesukkerConfig {
        let os_config_dir = dirs::config_dir().expect("Could not get the OS config directory.");
        let app_config = os_config_dir.join("komlesukker/config.json");
        return KomlesukkerConfig::from(app_config);
    }

    pub fn from<T: AsRef<Path>>(config_path: T) -> KomlesukkerConfig {
        let config_content =
            fs::read(config_path).expect("Could not find the specified config file.");
        return serde_json::from_slice(&config_content)
            .expect("Could not parse the config content as JSON.");
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_secret(&self) -> String {
        self.secret.clone()
    }

    pub fn get_thresholds(&self) -> Threshold {
        self.threshold.clone()
    }
}
