use crate::cli::display::DisplayService;
use crate::config::UserPreferences;
use crate::models::preset::TimerPreset;
use crate::models::session::SessionType;
use crate::services::database::DatabaseService;
use crate::services::timer::{get_duration_from_preset, TimerService};
use chrono::Utc;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

pub fn handle_start(
    db: Arc<DatabaseService>,
    preferences: &UserPreferences,
    type_str: Option<String>,
    preset_str: Option<String>,
) -> anyhow::Result<()> {
    let timer_service = TimerService::new(db, preferences);

    // Check for active timer
    if let Some((session, state)) = timer_service.check_active_timer()? {
        println!("⚠️  A timer is already running!");
        DisplayService::display_timer_info(
            session.session_type.as_str(),
            state.status.as_str(),
            timer_service.calculate_remaining_time(&state),
            session.duration_minutes,
        );
        anyhow::bail!("Use 'pomodoro status' to check it or 'pomodoro cancel' to stop it.");
    }

    // Determine session type
    let session_type = if let Some(ref t) = type_str {
        match t.as_str() {
            "work" => SessionType::Work,
            "break" => timer_service.determine_break_type()?,
            "short_break" => SessionType::ShortBreak,
            "long_break" => SessionType::LongBreak,
            _ => {
                anyhow::bail!("Invalid session type. Use: work, break, short_break, or long_break");
            }
        }
    } else {
        SessionType::Work
    };

    // Get preset
    let preset_name = preset_str
        .as_deref()
        .or(Some(preferences.default_preset.as_str()))
        .unwrap_or("standard");

    let preset = TimerPreset::parse(preset_name)
        .ok_or_else(|| anyhow::anyhow!("Invalid preset: {}", preset_name))?;

    // Get duration
    let duration_minutes = get_duration_from_preset(&preset, session_type, preferences);

    // Start the session
    let (session_id, _state) = timer_service.start_session(session_type, duration_minutes)?;

    println!(
        "✨ Starting {} session for {} minutes...\n",
        DisplayService::format_session_type(session_type.as_str()),
        duration_minutes
    );
    println!("Session ID: {}", session_id);
    println!("\nUse 'pomodoro status' to check progress");
    println!("Use 'pomodoro pause' to pause");
    println!("Use 'pomodoro cancel' to cancel\n");

    Ok(())
}

pub fn handle_status(
    db: Arc<DatabaseService>,
    preferences: &UserPreferences,
    json: bool,
) -> anyhow::Result<()> {
    let timer_service = TimerService::new(db, preferences);

    if let Some((session, state)) = timer_service.check_active_timer()? {
        let remaining = timer_service.calculate_remaining_time(&state);

        // Check if timer has expired
        if remaining == 0 {
            println!("⏰ Timer completed!");
            timer_service.complete_session(session.id.unwrap(), session.session_type)?;
            println!("✅ Session marked as complete\n");
            return Ok(());
        }

        if json {
            let output = serde_json::json!({
                "session_id": session.id,
                "session_type": session.session_type.as_str(),
                "status": state.status.as_str(),
                "duration_minutes": session.duration_minutes,
                "remaining_seconds": remaining,
                "started_at": session.started_at.to_rfc3339(),
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            DisplayService::display_timer_info(
                session.session_type.as_str(),
                state.status.as_str(),
                remaining,
                session.duration_minutes,
            );

            // Show progress bar
            let elapsed_seconds = (session.duration_minutes * 60) - remaining;
            let total_seconds = session.duration_minutes * 60;
            let progress_pct = (elapsed_seconds as f64 / total_seconds as f64 * 100.0) as i32;

            println!(
                "Progress: [{}{}] {}%",
                "=".repeat((progress_pct / 2) as usize),
                " ".repeat((50 - progress_pct / 2) as usize),
                progress_pct
            );
            println!(
                "\nElapsed: {} / {}\n",
                DisplayService::format_time(elapsed_seconds),
                DisplayService::format_time(total_seconds)
            );
        }
    } else if json {
        println!("{{\"status\": \"no_active_timer\"}}");
    } else {
        println!("📭 No active timer");
        println!("\nStart a timer with: pomodoro start\n");
    }

    Ok(())
}

pub fn handle_pause(db: Arc<DatabaseService>, preferences: &UserPreferences) -> anyhow::Result<()> {
    let timer_service = TimerService::new(db, preferences);
    timer_service.pause_session()?;
    println!("⏸️  Timer paused");
    println!("\nUse 'pomodoro resume' to continue\n");
    Ok(())
}

pub fn handle_resume(
    db: Arc<DatabaseService>,
    preferences: &UserPreferences,
) -> anyhow::Result<()> {
    let timer_service = TimerService::new(db, preferences);
    timer_service.resume_session()?;
    println!("▶️  Timer resumed");
    println!("\nUse 'pomodoro status' to check progress\n");
    Ok(())
}

pub fn handle_cancel(
    db: Arc<DatabaseService>,
    preferences: &UserPreferences,
    force: bool,
) -> anyhow::Result<()> {
    let timer_service = TimerService::new(db, preferences);

    // Check if there's an active timer
    if timer_service.check_active_timer()?.is_none() {
        anyhow::bail!("No active timer to cancel");
    }

    if !force {
        print!("Are you sure you want to cancel the current session? (y/N): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    timer_service.cancel_session()?;
    println!("❌ Timer cancelled");
    println!("\nThis session will not be counted in statistics.\n");
    Ok(())
}

pub fn handle_stats(
    db: Arc<DatabaseService>,
    date_str: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    use crate::models::statistics::UserStatistics;

    let date = if let Some(d) = date_str {
        chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")?
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .and_utc()
    } else {
        Utc::now()
    };

    let sessions = db.get_sessions_by_date(date)?;
    let stats = UserStatistics::calculate_from_sessions(&sessions);

    if json {
        println!("{}", serde_json::to_string_pretty(&stats)?);
    } else {
        println!("\n📊 Statistics for {}", date.format("%Y-%m-%d"));
        println!("┌─────────────────────────────────────┐");
        println!(
            "│  Completed work sessions: {:>9} │",
            stats.completed_work_sessions
        );
        println!(
            "│  Completed break sessions: {:>8} │",
            stats.completed_break_sessions
        );
        println!("│  Total work time: {:>14} min │", stats.total_work_minutes);
        println!(
            "│  Total break time: {:>13} min │",
            stats.total_break_minutes
        );
        println!("│  Cancelled sessions: {:>12} │", stats.cancelled_sessions);
        println!("│  Current streak: {:>16} │", stats.current_streak);
        println!("└─────────────────────────────────────┘\n");

        if !sessions.is_empty() {
            println!("Recent sessions:");
            for session in sessions.iter().rev().take(10) {
                let time = session.started_at.format("%H:%M:%S");
                let type_icon = match session.session_type {
                    SessionType::Work => "🍅",
                    SessionType::ShortBreak => "☕",
                    SessionType::LongBreak => "🌟",
                };
                let status_icon = match session.status {
                    crate::models::session::SessionStatus::Completed => "✅",
                    crate::models::session::SessionStatus::Cancelled => "❌",
                    _ => "⏸️",
                };
                println!(
                    "  {} {} {} - {} min ({})",
                    time,
                    type_icon,
                    session.session_type.as_str(),
                    session.duration_minutes,
                    status_icon
                );
            }
            println!();
        }
    }

    Ok(())
}

pub fn handle_config(
    config_path: PathBuf,
    list: bool,
    get: Option<String>,
    set: Option<(String, String)>,
) -> anyhow::Result<()> {
    let mut preferences = UserPreferences::load(&config_path)?;

    if list {
        println!("\n⚙️  Current Configuration:");
        println!("┌─────────────────────────────────────┐");
        println!("│  default_preset: {:>18} │", preferences.default_preset);
        println!("│  sound_enabled: {:>19} │", preferences.sound_enabled);
        println!(
            "│  notification_enabled: {:>12} │",
            preferences.notification_enabled
        );
        if let Some(ref path) = preferences.custom_sound_path {
            println!("│  custom_sound_path: {:>15} │", path);
        }
        if let Some(work) = preferences.work_minutes {
            println!("│  work_minutes: {:>20} │", work);
        }
        if let Some(short) = preferences.short_break_minutes {
            println!("│  short_break_minutes: {:>13} │", short);
        }
        if let Some(long) = preferences.long_break_minutes {
            println!("│  long_break_minutes: {:>14} │", long);
        }
        println!("└─────────────────────────────────────┘\n");
        return Ok(());
    }

    if let Some(key) = get {
        let value = match key.as_str() {
            "default_preset" => preferences.default_preset.clone(),
            "sound_enabled" => preferences.sound_enabled.to_string(),
            "notification_enabled" => preferences.notification_enabled.to_string(),
            "custom_sound_path" => preferences.custom_sound_path.clone().unwrap_or_default(),
            "work_minutes" => preferences
                .work_minutes
                .map(|v| v.to_string())
                .unwrap_or_default(),
            "short_break_minutes" => preferences
                .short_break_minutes
                .map(|v| v.to_string())
                .unwrap_or_default(),
            "long_break_minutes" => preferences
                .long_break_minutes
                .map(|v| v.to_string())
                .unwrap_or_default(),
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        };
        println!("{}", value);
        return Ok(());
    }

    if let Some((key, value)) = set {
        let value_display = value.clone();
        match key.as_str() {
            "default_preset" => {
                if !["standard", "short", "long"].contains(&value.as_str()) {
                    anyhow::bail!("Invalid preset. Use: standard, short, or long");
                }
                preferences.default_preset = value;
            }
            "sound_enabled" => {
                preferences.sound_enabled = value.parse()?;
            }
            "notification_enabled" => {
                preferences.notification_enabled = value.parse()?;
            }
            "custom_sound_path" => {
                preferences.custom_sound_path = Some(value);
            }
            "work_minutes" => {
                preferences.work_minutes = Some(value.parse()?);
            }
            "short_break_minutes" => {
                preferences.short_break_minutes = Some(value.parse()?);
            }
            "long_break_minutes" => {
                preferences.long_break_minutes = Some(value.parse()?);
            }
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }

        preferences.validate()?;
        preferences.save(&config_path)?;
        println!("✅ Configuration updated: {} = {}", key, value_display);
        return Ok(());
    }

    println!("Use --list to show all config, --get KEY to get a value, or --set KEY=VALUE to set a value");
    Ok(())
}
