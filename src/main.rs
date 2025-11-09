use anyhow::Result;
use clap::Parser;
use pomodoro::cli::commands;
use pomodoro::cli::{Cli, Commands};
use pomodoro::config::UserPreferences;
use pomodoro::services::DatabaseService;
use std::path::PathBuf;
use std::sync::Arc;

fn get_data_dir() -> PathBuf {
    std::env::var("POMODORO_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("pomodoro")
        })
}

fn get_db_path() -> PathBuf {
    std::env::var("POMODORO_DB_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| get_data_dir().join("sessions.db"))
}

fn get_config_path() -> PathBuf {
    std::env::var("POMODORO_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| get_data_dir().join("config.json"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }

    let db_path = get_db_path();
    let config_path = get_config_path();

    // Initialize database
    let db = Arc::new(DatabaseService::new(db_path)?);

    // Load preferences
    let preferences = UserPreferences::load(&config_path)?;

    match cli.command {
        Commands::Start { r#type, preset } => {
            commands::handle_start(db, &preferences, r#type, preset)?;
        }
        Commands::Status { json } => {
            commands::handle_status(db, &preferences, json)?;
        }
        Commands::Pause => {
            commands::handle_pause(db, &preferences)?;
        }
        Commands::Resume => {
            commands::handle_resume(db, &preferences)?;
        }
        Commands::Cancel { force } => {
            commands::handle_cancel(db, &preferences, force)?;
        }
        Commands::Stats { date, json } => {
            commands::handle_stats(db, date, json)?;
        }
        Commands::Config { list, get, set } => {
            commands::handle_config(config_path, list, get, set)?;
        }
    }

    Ok(())
}
