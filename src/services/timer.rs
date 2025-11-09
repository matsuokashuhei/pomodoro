use crate::config::UserPreferences;
use crate::models::preset::TimerPreset;
use crate::models::session::{SessionStatus, SessionType, TimerSession, TimerState, TimerStatus};
use crate::services::audio::AudioService;
use crate::services::database::DatabaseService;
use crate::services::notifier::NotificationService;
use chrono::Utc;
use std::sync::Arc;

pub const DEFAULT_LONG_BREAK_INTERVAL: i32 = 4;

pub struct TimerService {
    db: Arc<DatabaseService>,
    notifier: NotificationService,
    audio: AudioService,
    long_break_interval: i32,
}

impl TimerService {
    pub fn new(db: Arc<DatabaseService>, preferences: &UserPreferences) -> Self {
        let notifier = NotificationService::new(preferences.notification_enabled);
        let audio = AudioService::new(
            preferences.sound_enabled,
            preferences.custom_sound_path.as_ref().map(|p| p.into()),
        );

        Self {
            db,
            notifier,
            audio,
            long_break_interval: preferences
                .long_break_interval
                .unwrap_or(DEFAULT_LONG_BREAK_INTERVAL),
        }
    }

    pub fn check_active_timer(&self) -> anyhow::Result<Option<(TimerSession, TimerState)>> {
        if let Some(state) = self.db.get_timer_state()? {
            if let Some(session) = self.db.get_session(state.session_id)? {
                return Ok(Some((session, state)));
            }
        }
        Ok(None)
    }

    pub fn start_session(
        &self,
        session_type: SessionType,
        duration_minutes: i32,
    ) -> anyhow::Result<(i64, TimerState)> {
        // Check for active timer
        if self.check_active_timer()?.is_some() {
            anyhow::bail!(
                "A timer is already running. Use 'status' to check it or 'cancel' to stop it."
            );
        }

        // Create session
        let session = TimerSession::new(session_type, duration_minutes);
        let session_id = self.db.create_session(&session)?;

        // Create timer state
        let timer_state = TimerState::new(session_id, duration_minutes);
        self.db.save_timer_state(&timer_state)?;

        Ok((session_id, timer_state))
    }

    pub fn calculate_remaining_time(&self, state: &TimerState) -> i32 {
        if state.status == TimerStatus::Running {
            let elapsed = (Utc::now() - state.last_updated_at).num_seconds() as i32;
            (state.remaining_seconds - elapsed).max(0)
        } else {
            state.remaining_seconds
        }
    }

    pub fn complete_session(
        &self,
        session_id: i64,
        session_type: SessionType,
    ) -> anyhow::Result<()> {
        // Update session status
        self.db
            .update_session_status(session_id, SessionStatus::Completed, Some(Utc::now()))?;

        // Delete timer state
        self.db.delete_timer_state()?;

        // Send notification
        match session_type {
            SessionType::Work => {
                self.notifier.send_work_complete()?;

                // Check if long break is due
                let completed_count = self.db.count_completed_work_sessions_today()?;
                if completed_count % self.long_break_interval == 0 && completed_count > 0 {
                    self.notifier.send_long_break_suggestion()?;
                }
            }
            SessionType::ShortBreak | SessionType::LongBreak => {
                self.notifier.send_break_complete()?;
            }
        }

        // Play sound
        let _ = self.audio.play_completion_sound();

        Ok(())
    }

    pub fn pause_session(&self) -> anyhow::Result<()> {
        if let Some(mut state) = self.db.get_timer_state()? {
            // Calculate actual remaining time
            let remaining = self.calculate_remaining_time(&state);
            state.remaining_seconds = remaining;
            state.pause();
            self.db.save_timer_state(&state)?;

            // Update session status
            self.db
                .update_session_status(state.session_id, SessionStatus::Paused, None)?;
            Ok(())
        } else {
            anyhow::bail!("No active timer to pause")
        }
    }

    pub fn resume_session(&self) -> anyhow::Result<()> {
        if let Some(mut state) = self.db.get_timer_state()? {
            if state.status != TimerStatus::Paused {
                anyhow::bail!("Timer is not paused");
            }
            state.resume();
            self.db.save_timer_state(&state)?;

            // Update session status
            self.db
                .update_session_status(state.session_id, SessionStatus::Active, None)?;
            Ok(())
        } else {
            anyhow::bail!("No timer to resume")
        }
    }

    pub fn cancel_session(&self) -> anyhow::Result<()> {
        if let Some(state) = self.db.get_timer_state()? {
            self.db.update_session_status(
                state.session_id,
                SessionStatus::Cancelled,
                Some(Utc::now()),
            )?;
            self.db.delete_timer_state()?;
            Ok(())
        } else {
            anyhow::bail!("No active timer to cancel")
        }
    }

    pub fn count_completed_work_sessions(&self) -> anyhow::Result<i32> {
        self.db.count_completed_work_sessions_today()
    }

    pub fn determine_break_type(&self) -> anyhow::Result<SessionType> {
        let count = self.count_completed_work_sessions()?;
        if count > 0 && count % self.long_break_interval == 0 {
            Ok(SessionType::LongBreak)
        } else {
            Ok(SessionType::ShortBreak)
        }
    }
}

pub fn get_duration_from_preset(
    preset: &TimerPreset,
    session_type: SessionType,
    preferences: &UserPreferences,
) -> i32 {
    match session_type {
        SessionType::Work => preferences.work_minutes.unwrap_or(preset.work_minutes),
        SessionType::ShortBreak => preferences
            .short_break_minutes
            .unwrap_or(preset.short_break_minutes),
        SessionType::LongBreak => preferences
            .long_break_minutes
            .unwrap_or(preset.long_break_minutes),
    }
}
