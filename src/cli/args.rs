use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pomodoro")]
#[command(about = "A CLI Pomodoro Timer", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a new timer session
    Start {
        #[arg(short, long, help = "Session type: work, break")]
        r#type: Option<String>,

        #[arg(short, long, help = "Preset: standard, short, long")]
        preset: Option<String>,
    },

    /// Show current timer status
    Status {
        #[arg(long, help = "Output in JSON format")]
        json: bool,
    },

    /// Pause the current timer
    Pause,

    /// Resume a paused timer
    Resume,

    /// Cancel the current session
    Cancel {
        #[arg(short, long, help = "Skip confirmation prompt")]
        force: bool,
    },

    /// Show session statistics
    Stats {
        #[arg(short, long, help = "Date to show stats for (YYYY-MM-DD)")]
        date: Option<String>,

        #[arg(long, help = "Output in JSON format")]
        json: bool,
    },

    /// Manage configuration
    Config {
        #[arg(long, help = "List all configuration values")]
        list: bool,

        #[arg(long, help = "Get a specific configuration value")]
        get: Option<String>,

        #[arg(long, help = "Set a configuration value (format: key=value)", value_parser = parse_key_val)]
        set: Option<(String, String)>,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid format: '{}'. Expected key=value", s));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
