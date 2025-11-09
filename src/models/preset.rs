use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresetType {
    Standard,
    Short,
    Long,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerPreset {
    pub preset_type: PresetType,
    pub work_minutes: i32,
    pub short_break_minutes: i32,
    pub long_break_minutes: i32,
}

impl TimerPreset {
    pub fn standard() -> Self {
        Self {
            preset_type: PresetType::Standard,
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
        }
    }

    pub fn short() -> Self {
        Self {
            preset_type: PresetType::Short,
            work_minutes: 15,
            short_break_minutes: 3,
            long_break_minutes: 10,
        }
    }

    pub fn long() -> Self {
        Self {
            preset_type: PresetType::Long,
            work_minutes: 50,
            short_break_minutes: 10,
            long_break_minutes: 30,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "standard" => Some(Self::standard()),
            "short" => Some(Self::short()),
            "long" => Some(Self::long()),
            _ => None,
        }
    }
}

impl Default for TimerPreset {
    fn default() -> Self {
        Self::standard()
    }
}
