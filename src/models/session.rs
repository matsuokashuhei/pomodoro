use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub fn as_str(&self) -> &str {
        match self {
            SessionType::Work => "work",
            SessionType::ShortBreak => "short_break",
            SessionType::LongBreak => "long_break",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "work" => Some(SessionType::Work),
            "short_break" => Some(SessionType::ShortBreak),
            "long_break" => Some(SessionType::LongBreak),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl SessionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SessionStatus::Active => "active",
            SessionStatus::Paused => "paused",
            SessionStatus::Completed => "completed",
            SessionStatus::Cancelled => "cancelled",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "active" => Some(SessionStatus::Active),
            "paused" => Some(SessionStatus::Paused),
            "completed" => Some(SessionStatus::Completed),
            "cancelled" => Some(SessionStatus::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerSession {
    pub id: Option<i64>,
    pub session_type: SessionType,
    pub status: SessionStatus,
    pub duration_minutes: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl TimerSession {
    pub fn new(session_type: SessionType, duration_minutes: i32) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            session_type,
            status: SessionStatus::Active,
            duration_minutes,
            started_at: now,
            completed_at: None,
            cancelled_at: None,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimerStatus {
    Running,
    Paused,
    Completed,
}

impl TimerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TimerStatus::Running => "running",
            TimerStatus::Paused => "paused",
            TimerStatus::Completed => "completed",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "running" => Some(TimerStatus::Running),
            "paused" => Some(TimerStatus::Paused),
            "completed" => Some(TimerStatus::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub session_id: i64,
    pub status: TimerStatus,
    pub remaining_seconds: i32,
    pub last_updated_at: DateTime<Utc>,
}

impl TimerState {
    pub fn new(session_id: i64, duration_minutes: i32) -> Self {
        Self {
            session_id,
            status: TimerStatus::Running,
            remaining_seconds: duration_minutes * 60,
            last_updated_at: Utc::now(),
        }
    }

    pub fn pause(&mut self) {
        if self.status == TimerStatus::Running {
            self.status = TimerStatus::Paused;
            self.last_updated_at = Utc::now();
        }
    }

    pub fn resume(&mut self) {
        if self.status == TimerStatus::Paused {
            self.status = TimerStatus::Running;
            self.last_updated_at = Utc::now();
        }
    }

    pub fn tick(&mut self, seconds: i32) {
        if self.status == TimerStatus::Running {
            self.remaining_seconds = (self.remaining_seconds - seconds).max(0);
            self.last_updated_at = Utc::now();
            if self.remaining_seconds == 0 {
                self.status = TimerStatus::Completed;
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        self.remaining_seconds == 0 || self.status == TimerStatus::Completed
    }
}
