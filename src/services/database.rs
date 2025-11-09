use crate::models::session::{SessionStatus, SessionType, TimerSession, TimerState, TimerStatus};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

pub struct DatabaseService {
    db_path: PathBuf,
}

impl DatabaseService {
    pub fn new(db_path: PathBuf) -> anyhow::Result<Self> {
        let service = Self { db_path };
        service.ensure_directory()?;
        service.run_migrations()?;
        Ok(service)
    }

    fn ensure_directory(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn run_migrations(&self) -> anyhow::Result<()> {
        let conn = self.get_connection()?;
        let migration_sql = include_str!("../../migrations/001_initial_schema.sql");
        conn.execute_batch(migration_sql)?;
        Ok(())
    }

    // Session CRUD operations
    pub fn create_session(&self, session: &TimerSession) -> anyhow::Result<i64> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT INTO sessions (session_type, status, duration_minutes, started_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                session.session_type.as_str(),
                session.status.as_str(),
                session.duration_minutes,
                session.started_at.to_rfc3339(),
                session.created_at.to_rfc3339(),
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_session(&self, id: i64) -> anyhow::Result<Option<TimerSession>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, session_type, status, duration_minutes, started_at, completed_at,
                    cancelled_at, created_at
             FROM sessions WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(TimerSession {
                id: Some(row.get(0)?),
                session_type: SessionType::parse(&row.get::<_, String>(1)?)
                    .unwrap_or(SessionType::Work),
                status: SessionStatus::parse(&row.get::<_, String>(2)?)
                    .unwrap_or(SessionStatus::Active),
                duration_minutes: row.get(3)?,
                started_at: row
                    .get::<_, String>(4)?
                    .parse::<DateTime<Utc>>()
                    .unwrap_or_else(|_| Utc::now()),
                completed_at: row
                    .get::<_, Option<String>>(5)?
                    .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
                cancelled_at: row
                    .get::<_, Option<String>>(6)?
                    .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
                created_at: row
                    .get::<_, String>(7)?
                    .parse::<DateTime<Utc>>()
                    .unwrap_or_else(|_| Utc::now()),
            })
        });

        match result {
            Ok(session) => Ok(Some(session)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn update_session_status(
        &self,
        id: i64,
        status: SessionStatus,
        timestamp: Option<DateTime<Utc>>,
    ) -> anyhow::Result<()> {
        let conn = self.get_connection()?;
        match status {
            SessionStatus::Completed => {
                let ts = timestamp.unwrap_or_else(Utc::now);
                conn.execute(
                    "UPDATE sessions SET status = ?1, completed_at = ?2 WHERE id = ?3",
                    params![status.as_str(), ts.to_rfc3339(), id],
                )?;
            }
            SessionStatus::Cancelled => {
                let ts = timestamp.unwrap_or_else(Utc::now);
                conn.execute(
                    "UPDATE sessions SET status = ?1, cancelled_at = ?2 WHERE id = ?3",
                    params![status.as_str(), ts.to_rfc3339(), id],
                )?;
            }
            _ => {
                conn.execute(
                    "UPDATE sessions SET status = ?1 WHERE id = ?2",
                    params![status.as_str(), id],
                )?;
            }
        }
        Ok(())
    }

    pub fn get_sessions_by_date(&self, date: DateTime<Utc>) -> anyhow::Result<Vec<TimerSession>> {
        let conn = self.get_connection()?;
        let start_of_day = date
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid start of day timestamp"))?;
        let end_of_day = date
            .date_naive()
            .and_hms_opt(23, 59, 59)
            .ok_or_else(|| anyhow::anyhow!("Invalid end of day timestamp"))?;

        let mut stmt = conn.prepare(
            "SELECT id, session_type, status, duration_minutes, started_at, completed_at,
                    cancelled_at, created_at
             FROM sessions
             WHERE started_at >= ?1 AND started_at <= ?2
             ORDER BY started_at ASC",
        )?;

        let sessions = stmt
            .query_map(
                params![
                    start_of_day.and_utc().to_rfc3339(),
                    end_of_day.and_utc().to_rfc3339()
                ],
                |row| {
                    Ok(TimerSession {
                        id: Some(row.get(0)?),
                        session_type: SessionType::parse(&row.get::<_, String>(1)?)
                            .unwrap_or(SessionType::Work),
                        status: SessionStatus::parse(&row.get::<_, String>(2)?)
                            .unwrap_or(SessionStatus::Active),
                        duration_minutes: row.get(3)?,
                        started_at: row
                            .get::<_, String>(4)?
                            .parse::<DateTime<Utc>>()
                            .unwrap_or_else(|_| Utc::now()),
                        completed_at: row
                            .get::<_, Option<String>>(5)?
                            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
                        cancelled_at: row
                            .get::<_, Option<String>>(6)?
                            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
                        created_at: row
                            .get::<_, String>(7)?
                            .parse::<DateTime<Utc>>()
                            .unwrap_or_else(|_| Utc::now()),
                    })
                },
            )?
            .collect::<Result<Vec<_>>>()?;

        Ok(sessions)
    }

    // Timer state operations (singleton)
    pub fn save_timer_state(&self, state: &TimerState) -> anyhow::Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO timer_state
             (id, session_id, status, remaining_seconds, last_updated_at)
             VALUES (1, ?1, ?2, ?3, ?4)",
            params![
                state.session_id,
                state.status.as_str(),
                state.remaining_seconds,
                state.last_updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_timer_state(&self) -> anyhow::Result<Option<TimerState>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT session_id, status, remaining_seconds, last_updated_at
             FROM timer_state WHERE id = 1",
        )?;

        let result = stmt.query_row([], |row| {
            Ok(TimerState {
                session_id: row.get(0)?,
                status: TimerStatus::parse(&row.get::<_, String>(1)?)
                    .unwrap_or(TimerStatus::Running),
                remaining_seconds: row.get(2)?,
                last_updated_at: row
                    .get::<_, String>(3)?
                    .parse::<DateTime<Utc>>()
                    .unwrap_or_else(|_| Utc::now()),
            })
        });

        match result {
            Ok(state) => Ok(Some(state)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn delete_timer_state(&self) -> anyhow::Result<()> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM timer_state WHERE id = 1", [])?;
        Ok(())
    }

    // Preferences operations
    pub fn set_preference(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let conn = self.get_connection()?;
        conn.execute(
            "INSERT OR REPLACE INTO preferences (key, value, updated_at)
             VALUES (?1, ?2, ?3)",
            params![key, value, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_preference(&self, key: &str) -> anyhow::Result<Option<String>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT value FROM preferences WHERE key = ?1")?;

        let result = stmt.query_row(params![key], |row| row.get(0));

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn count_completed_work_sessions_today(&self) -> anyhow::Result<i32> {
        let today = Utc::now();
        let start_of_day = today.date_naive().and_hms_opt(0, 0, 0).unwrap();

        let conn = self.get_connection()?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sessions
             WHERE session_type = 'work'
             AND status = 'completed'
             AND started_at >= ?1",
            params![start_of_day.and_utc().to_rfc3339()],
            |row| row.get(0),
        )?;

        Ok(count)
    }
}
