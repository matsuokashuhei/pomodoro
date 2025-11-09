use pomodoro::config::UserPreferences;
use pomodoro::models::preset::TimerPreset;
use pomodoro::models::session::SessionType;
use pomodoro::services::database::DatabaseService;
use pomodoro::services::timer::{get_duration_from_preset, TimerService};
use std::sync::Arc;
use tempfile::TempDir;

fn setup_test_db() -> (TempDir, Arc<DatabaseService>) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(DatabaseService::new(db_path).unwrap());
    (temp_dir, db)
}

#[test]
fn test_complete_timer_flow() {
    let (_temp_dir, db) = setup_test_db();
    let prefs = UserPreferences::default();
    let timer_service = TimerService::new(db.clone(), &prefs);

    // Start a work session
    let (session_id, state) = timer_service
        .start_session(SessionType::Work, 25)
        .expect("Failed to start session");

    assert_eq!(session_id, 1);
    assert_eq!(state.remaining_seconds, 25 * 60);

    // Check active timer
    let active = timer_service
        .check_active_timer()
        .expect("Failed to check active timer");
    assert!(active.is_some());

    let (session, _) = active.unwrap();
    assert_eq!(session.session_type, SessionType::Work);
    assert_eq!(session.duration_minutes, 25);

    // Complete the session
    timer_service
        .complete_session(session_id, SessionType::Work)
        .expect("Failed to complete session");

    // Verify no active timer
    let active = timer_service
        .check_active_timer()
        .expect("Failed to check active timer");
    assert!(active.is_none());

    // Verify session was completed in database
    let completed_session = db.get_session(session_id).expect("Failed to get session");
    assert!(completed_session.is_some());
    let session = completed_session.unwrap();
    assert_eq!(
        session.status,
        pomodoro::models::session::SessionStatus::Completed
    );
}

#[test]
fn test_pause_resume_flow() {
    let (_temp_dir, db) = setup_test_db();
    let prefs = UserPreferences::default();
    let timer_service = TimerService::new(db, &prefs);

    // Start a session
    timer_service
        .start_session(SessionType::Work, 25)
        .expect("Failed to start session");

    // Pause the session
    timer_service.pause_session().expect("Failed to pause");

    // Check status
    let active = timer_service
        .check_active_timer()
        .expect("Failed to check active timer");
    assert!(active.is_some());
    let (session, state) = active.unwrap();
    assert_eq!(
        session.status,
        pomodoro::models::session::SessionStatus::Paused
    );
    assert_eq!(state.status, pomodoro::models::session::TimerStatus::Paused);

    // Resume the session
    timer_service.resume_session().expect("Failed to resume");

    // Check status again
    let active = timer_service
        .check_active_timer()
        .expect("Failed to check active timer");
    assert!(active.is_some());
    let (session, state) = active.unwrap();
    assert_eq!(
        session.status,
        pomodoro::models::session::SessionStatus::Active
    );
    assert_eq!(
        state.status,
        pomodoro::models::session::TimerStatus::Running
    );
}

#[test]
fn test_cancel_flow() {
    let (_temp_dir, db) = setup_test_db();
    let prefs = UserPreferences::default();
    let timer_service = TimerService::new(db.clone(), &prefs);

    // Start a session
    let (session_id, _) = timer_service
        .start_session(SessionType::Work, 25)
        .expect("Failed to start session");

    // Cancel the session
    timer_service
        .cancel_session()
        .expect("Failed to cancel session");

    // Verify no active timer
    let active = timer_service
        .check_active_timer()
        .expect("Failed to check active timer");
    assert!(active.is_none());

    // Verify session was cancelled
    let cancelled_session = db.get_session(session_id).expect("Failed to get session");
    assert!(cancelled_session.is_some());
    let session = cancelled_session.unwrap();
    assert_eq!(
        session.status,
        pomodoro::models::session::SessionStatus::Cancelled
    );
}

#[test]
fn test_preset_durations() {
    let standard = TimerPreset::standard();
    assert_eq!(standard.work_minutes, 25);
    assert_eq!(standard.short_break_minutes, 5);
    assert_eq!(standard.long_break_minutes, 15);

    let short = TimerPreset::short();
    assert_eq!(short.work_minutes, 15);
    assert_eq!(short.short_break_minutes, 3);
    assert_eq!(short.long_break_minutes, 10);

    let long = TimerPreset::long();
    assert_eq!(long.work_minutes, 50);
    assert_eq!(long.short_break_minutes, 10);
    assert_eq!(long.long_break_minutes, 30);
}

#[test]
fn test_get_duration_from_preset() {
    let preset = TimerPreset::standard();
    let prefs = UserPreferences::default();

    let work_duration = get_duration_from_preset(&preset, SessionType::Work, &prefs);
    assert_eq!(work_duration, 25);

    let short_break_duration = get_duration_from_preset(&preset, SessionType::ShortBreak, &prefs);
    assert_eq!(short_break_duration, 5);

    let long_break_duration = get_duration_from_preset(&preset, SessionType::LongBreak, &prefs);
    assert_eq!(long_break_duration, 15);
}

#[test]
fn test_preferences_override() {
    let preset = TimerPreset::standard();
    let mut prefs = UserPreferences::default();
    prefs.work_minutes = Some(30);

    let work_duration = get_duration_from_preset(&preset, SessionType::Work, &prefs);
    assert_eq!(work_duration, 30); // Should use preference override
}

#[test]
fn test_statistics_calculation() {
    use pomodoro::models::session::{SessionStatus, TimerSession};
    use pomodoro::models::statistics::UserStatistics;

    let mut sessions = Vec::new();

    // Add completed work sessions
    for _ in 0..3 {
        let mut session = TimerSession::new(SessionType::Work, 25);
        session.status = SessionStatus::Completed;
        sessions.push(session);
    }

    // Add completed break session
    let mut break_session = TimerSession::new(SessionType::ShortBreak, 5);
    break_session.status = SessionStatus::Completed;
    sessions.push(break_session);

    // Add cancelled session
    let mut cancelled = TimerSession::new(SessionType::Work, 25);
    cancelled.status = SessionStatus::Cancelled;
    sessions.push(cancelled);

    let stats = UserStatistics::calculate_from_sessions(&sessions);

    assert_eq!(stats.completed_work_sessions, 3);
    assert_eq!(stats.completed_break_sessions, 1);
    assert_eq!(stats.total_work_minutes, 75);
    assert_eq!(stats.total_break_minutes, 5);
    assert_eq!(stats.cancelled_sessions, 1);
    assert_eq!(stats.current_streak, 0); // Streak broken by cancelled session
}

#[test]
fn test_long_break_detection() {
    let (_temp_dir, db) = setup_test_db();
    let prefs = UserPreferences::default();
    let timer_service = TimerService::new(db.clone(), &prefs);

    // Complete 3 work sessions
    for _ in 0..3 {
        let (session_id, _) = timer_service
            .start_session(SessionType::Work, 25)
            .expect("Failed to start session");
        timer_service
            .complete_session(session_id, SessionType::Work)
            .expect("Failed to complete session");
    }

    // Should suggest short break
    let break_type = timer_service
        .determine_break_type()
        .expect("Failed to determine break type");
    assert_eq!(break_type, SessionType::ShortBreak);

    // Complete one more work session
    let (session_id, _) = timer_service
        .start_session(SessionType::Work, 25)
        .expect("Failed to start session");
    timer_service
        .complete_session(session_id, SessionType::Work)
        .expect("Failed to complete session");

    // Should suggest long break after 4th session
    let break_type = timer_service
        .determine_break_type()
        .expect("Failed to determine break type");
    assert_eq!(break_type, SessionType::LongBreak);
}
