use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct DisplayService;

impl DisplayService {
    pub fn format_time(seconds: i32) -> String {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{:02}:{:02}", minutes, secs)
    }

    pub fn create_progress_bar(duration_seconds: i32) -> ProgressBar {
        let pb = ProgressBar::new(duration_seconds as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) - {eta} remaining")
                .unwrap()
                .progress_chars("=>-"),
        );
        pb
    }

    pub fn update_progress(pb: &ProgressBar, elapsed: i32, total: i32, message: &str) {
        pb.set_position(elapsed as u64);
        pb.set_length(total as u64);
        pb.set_message(message.to_string());
    }

    pub fn finish_progress(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(message.to_string());
    }

    pub fn format_session_type(session_type: &str) -> String {
        match session_type {
            "work" => "🍅 Work".to_string(),
            "short_break" => "☕ Short Break".to_string(),
            "long_break" => "🌟 Long Break".to_string(),
            _ => session_type.to_string(),
        }
    }

    pub fn format_session_status(status: &str) -> String {
        match status {
            "running" => "▶️  Running".to_string(),
            "paused" => "⏸️  Paused".to_string(),
            "completed" => "✅ Completed".to_string(),
            _ => status.to_string(),
        }
    }

    pub fn display_timer_info(
        session_type: &str,
        status: &str,
        remaining_seconds: i32,
        total_minutes: i32,
    ) {
        println!("\n┌─────────────────────────────────────┐");
        println!("│  Pomodoro Timer                     │");
        println!("├─────────────────────────────────────┤");
        println!(
            "│  Type:      {}  │",
            Self::format_session_type(session_type)
        );
        println!("│  Status:    {}  │", Self::format_session_status(status));
        println!(
            "│  Remaining: {}                │",
            Self::format_time(remaining_seconds)
        );
        println!("│  Duration:  {} minutes              │", total_minutes);
        println!("└─────────────────────────────────────┘\n");
    }

    pub fn sleep(duration: Duration) {
        std::thread::sleep(duration);
    }
}
