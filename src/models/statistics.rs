use crate::models::session::{SessionStatus, SessionType, TimerSession};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub date: DateTime<Utc>,
    pub completed_work_sessions: i32,
    pub completed_break_sessions: i32,
    pub total_work_minutes: i32,
    pub total_break_minutes: i32,
    pub cancelled_sessions: i32,
    pub current_streak: i32,
}

impl UserStatistics {
    pub fn calculate_from_sessions(sessions: &[TimerSession]) -> Self {
        let mut stats = Self {
            date: Utc::now(),
            completed_work_sessions: 0,
            completed_break_sessions: 0,
            total_work_minutes: 0,
            total_break_minutes: 0,
            cancelled_sessions: 0,
            current_streak: 0,
        };

        let mut consecutive_completed = 0;
        for session in sessions {
            match session.status {
                SessionStatus::Completed => {
                    consecutive_completed += 1;
                    match session.session_type {
                        SessionType::Work => {
                            stats.completed_work_sessions += 1;
                            stats.total_work_minutes += session.duration_minutes;
                        }
                        SessionType::ShortBreak | SessionType::LongBreak => {
                            stats.completed_break_sessions += 1;
                            stats.total_break_minutes += session.duration_minutes;
                            // Break sessions don't break the streak, only count work sessions
                        }
                    }
                }
                SessionStatus::Cancelled => {
                    stats.cancelled_sessions += 1;
                    consecutive_completed = 0;
                }
                SessionStatus::Active | SessionStatus::Paused => {
                    // Active or paused break sessions don't break the streak
                    // Only work sessions that are incomplete reset the streak
                    if session.session_type == SessionType::Work {
                        consecutive_completed = 0;
                    }
                }
            }
        }

        stats.current_streak = consecutive_completed;
        stats
    }
}
