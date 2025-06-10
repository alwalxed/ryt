use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub download_dir: PathBuf,
    pub default_quality: String,
    pub default_format: String,
    pub ytdlp_path: Option<String>,
    pub max_concurrent_downloads: usize,
}

impl Default for Config {
    fn default() -> Self {
        let download_dir = dirs::document_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("ryt");

        Self {
            download_dir,
            default_quality: "1080p".to_string(),
            default_format: "video".to_string(),
            ytdlp_path: None,
            max_concurrent_downloads: 3,
        }
    }
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content).unwrap_or_else(|_| Config::default());
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("ryt").join("config.toml"))
    }

    pub fn ensure_download_dirs(&self) -> Result<()> {
        fs::create_dir_all(&self.download_dir)?;
        fs::create_dir_all(self.download_dir.join("single-videos"))?;
        fs::create_dir_all(self.download_dir.join("playlists"))?;
        Ok(())
    }
}
