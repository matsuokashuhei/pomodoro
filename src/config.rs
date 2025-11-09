use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub default_preset: String,
    pub sound_enabled: bool,
    pub notification_enabled: bool,
    pub custom_sound_path: Option<String>,
    pub work_minutes: Option<i32>,
    pub short_break_minutes: Option<i32>,
    pub long_break_minutes: Option<i32>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_preset: "standard".to_string(),
            sound_enabled: true,
            notification_enabled: true,
            custom_sound_path: None,
            work_minutes: None,
            short_break_minutes: None,
            long_break_minutes: None,
        }
    }
}

impl UserPreferences {
    pub fn load(config_path: &PathBuf) -> anyhow::Result<Self> {
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let prefs: UserPreferences = serde_json::from_str(&content)?;
            Ok(prefs)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, config_path: &PathBuf) -> anyhow::Result<()> {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if let Some(work) = self.work_minutes {
            if work <= 0 {
                anyhow::bail!("work_minutes must be positive");
            }
        }
        if let Some(short_break) = self.short_break_minutes {
            if short_break <= 0 {
                anyhow::bail!("short_break_minutes must be positive");
            }
        }
        if let Some(long_break) = self.long_break_minutes {
            if long_break <= 0 {
                anyhow::bail!("long_break_minutes must be positive");
            }
        }
        if let Some(ref path) = self.custom_sound_path {
            let sound_path = PathBuf::from(path);
            if !sound_path.exists() {
                anyhow::bail!("custom sound file does not exist: {}", path);
            }
        }
        Ok(())
    }
}
