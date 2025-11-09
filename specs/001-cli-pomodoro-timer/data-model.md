# Data Model: CLI Pomodoro Timer

**Date**: November 9, 2025
**Feature**: CLI Pomodoro Timer
**Purpose**: Define data structures, database schema, and state management for timer sessions

---

## 1. Domain Entities

### 1.1 TimerSession

Represents a single Pomodoro work period or break period.

**Rust Structure:**
```rust
pub struct TimerSession {
    pub id: Option<i64>,           // Database ID, None for unsaved sessions
    pub session_type: SessionType,
    pub preset: TimerPreset,
    pub duration_minutes: u32,
    pub start_time: i64,           // Unix timestamp
    pub end_time: Option<i64>,     // Unix timestamp, None if not completed
    pub status: SessionStatus,
    pub created_at: i64,           // Unix timestamp
}

pub enum SessionType {
    Work,
    Break,
}

pub enum SessionStatus {
    Completed,
    Cancelled,
}

impl SessionType {
    pub fn as_str(&self) -> &str {
        match self {
            SessionType::Work => "work",
            SessionType::Break => "break",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "work" => Ok(SessionType::Work),
            "break" => Ok(SessionType::Break),
            _ => Err(format!("Invalid session type: {}", s)),
        }
    }
}
```

**Validation Rules:**
- `duration_minutes` must be > 0 and <= 120 (2 hours max)
- `start_time` must be <= current time
- `end_time`, if present, must be >= `start_time`
- `status` only applies to completed/cancelled sessions

**State Transitions:**
- A session starts with `end_time = None`, `status` undefined
- On completion: `end_time = now()`, `status = Completed`
- On cancellation: `end_time = now()`, `status = Cancelled`

---

### 1.2 TimerPreset

Represents the three preset configurations for work/break durations.

**Rust Structure:**
```rust
pub struct TimerPreset {
    pub name: PresetType,
    pub work_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
}

pub enum PresetType {
    Standard,  // 25/5/15
    Short,     // 15/3/10
    Long,      // 50/10/30
}

impl TimerPreset {
    pub fn standard() -> Self {
        Self {
            name: PresetType::Standard,
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
        }
    }

    pub fn short() -> Self {
        Self {
            name: PresetType::Short,
            work_minutes: 15,
            short_break_minutes: 3,
            long_break_minutes: 10,
        }
    }

    pub fn long() -> Self {
        Self {
            name: PresetType::Long,
            work_minutes: 50,
            short_break_minutes: 10,
            long_break_minutes: 30,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "standard" => Ok(Self::standard()),
            "short" => Ok(Self::short()),
            "long" => Ok(Self::long()),
            _ => Err(format!("Invalid preset: {}", s)),
        }
    }
}
```

**Validation Rules:**
- Preset values are immutable (hardcoded constants)
- Only three valid presets: standard, short, long

---

### 1.3 TimerState

Represents the current active timer state (singleton, max one active timer).

**Rust Structure:**
```rust
pub struct TimerState {
    pub session_id: i64,              // References TimerSession.id
    pub remaining_seconds: u32,
    pub status: TimerStatus,
    pub updated_at: i64,              // Unix timestamp
}

pub enum TimerStatus {
    Running,
    Paused,
}

impl TimerState {
    pub fn new(session_id: i64, duration_seconds: u32) -> Self {
        Self {
            session_id,
            remaining_seconds: duration_seconds,
            status: TimerStatus::Running,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn pause(&mut self) {
        self.status = TimerStatus::Paused;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn resume(&mut self) {
        self.status = TimerStatus::Running;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn tick(&mut self) -> bool {
        if self.status == TimerStatus::Running && self.remaining_seconds > 0 {
            self.remaining_seconds -= 1;
            self.updated_at = chrono::Utc::now().timestamp();
            return self.remaining_seconds == 0;
        }
        false
    }
}
```

**Validation Rules:**
- Only one `TimerState` can exist at a time (enforced by database constraint)
- `remaining_seconds` must be >= 0
- `session_id` must reference a valid session
- State is deleted when timer completes or is cancelled

**State Transitions:**
- Created: `Running` status with full duration
- Paused: `status = Paused`, `remaining_seconds` frozen
- Resumed: `status = Running`, countdown continues
- Completed: State deleted from database

---

### 1.4 UserStatistics

Represents aggregate statistics about user's Pomodoro usage.

**Rust Structure:**
```rust
pub struct UserStatistics {
    pub date: String,                    // YYYY-MM-DD format
    pub completed_work_sessions: u32,
    pub completed_break_sessions: u32,
    pub total_work_minutes: u32,
    pub total_break_minutes: u32,
    pub cancelled_sessions: u32,
    pub current_streak: u32,             // Consecutive completed work sessions
}

impl UserStatistics {
    pub fn calculate_from_sessions(sessions: &[TimerSession], date: &str) -> Self {
        let day_sessions: Vec<_> = sessions.iter()
            .filter(|s| {
                let session_date = chrono::NaiveDateTime::from_timestamp_opt(s.start_time, 0)
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string();
                session_date == date
            })
            .collect();

        let completed_work = day_sessions.iter()
            .filter(|s| matches!(s.session_type, SessionType::Work) && matches!(s.status, SessionStatus::Completed))
            .count() as u32;

        let completed_breaks = day_sessions.iter()
            .filter(|s| matches!(s.session_type, SessionType::Break) && matches!(s.status, SessionStatus::Completed))
            .count() as u32;

        let work_minutes = day_sessions.iter()
            .filter(|s| matches!(s.session_type, SessionType::Work) && matches!(s.status, SessionStatus::Completed))
            .map(|s| s.duration_minutes)
            .sum();

        let break_minutes = day_sessions.iter()
            .filter(|s| matches!(s.session_type, SessionType::Break) && matches!(s.status, SessionStatus::Completed))
            .map(|s| s.duration_minutes)
            .sum();

        let cancelled = day_sessions.iter()
            .filter(|s| matches!(s.status, SessionStatus::Cancelled))
            .count() as u32;

        let streak = calculate_streak(&day_sessions);

        Self {
            date: date.to_string(),
            completed_work_sessions: completed_work,
            completed_break_sessions: completed_breaks,
            total_work_minutes: work_minutes,
            total_break_minutes: break_minutes,
            cancelled_sessions: cancelled,
            current_streak: streak,
        }
    }
}

fn calculate_streak(sessions: &[&TimerSession]) -> u32 {
    let mut streak = 0;
    for session in sessions.iter().rev() {
        if matches!(session.session_type, SessionType::Work) && matches!(session.status, SessionStatus::Completed) {
            streak += 1;
        } else if matches!(session.status, SessionStatus::Cancelled) {
            break;
        }
    }
    streak
}
```

**Validation Rules:**
- Statistics are computed on-demand from session history (not stored)
- Date must be valid YYYY-MM-DD format
- All counts must be >= 0

---

### 1.5 UserPreferences

Represents user configuration and preferences.

**Rust Structure:**
```rust
pub struct UserPreferences {
    pub preset: PresetType,
    pub sound_enabled: bool,
    pub custom_sound_path: Option<String>,
    pub notification_enabled: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preset: PresetType::Standard,
            sound_enabled: true,
            custom_sound_path: None,
            notification_enabled: true,
        }
    }
}

impl UserPreferences {
    pub fn load() -> Result<Self, anyhow::Error> {
        // Load from ~/.local/share/pomodoro/config.json
        // Fall back to defaults if file doesn't exist
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        // Save to ~/.local/share/pomodoro/config.json
    }
}
```

**Validation Rules:**
- `preset` must be one of: standard, short, long
- `custom_sound_path`, if present, must be a valid file path

---

## 2. Database Schema

### 2.1 SQLite Schema

```sql
-- Sessions table: Stores all completed and cancelled timer sessions
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type TEXT NOT NULL CHECK (session_type IN ('work', 'break')),
    preset TEXT NOT NULL CHECK (preset IN ('standard', 'short', 'long')),
    duration_minutes INTEGER NOT NULL CHECK (duration_minutes > 0 AND duration_minutes <= 120),
    start_time INTEGER NOT NULL,
    end_time INTEGER CHECK (end_time IS NULL OR end_time >= start_time),
    status TEXT CHECK (status IN ('completed', 'cancelled')),
    created_at INTEGER NOT NULL
);

-- Timer state table: Singleton row for current active timer
CREATE TABLE IF NOT EXISTS timer_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Enforces singleton
    session_id INTEGER NOT NULL,
    remaining_seconds INTEGER NOT NULL CHECK (remaining_seconds >= 0),
    status TEXT NOT NULL CHECK (status IN ('running', 'paused')),
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- Preferences table: Key-value store for user settings
CREATE TABLE IF NOT EXISTS preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_sessions_start_time ON sessions(start_time DESC);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);
CREATE INDEX IF NOT EXISTS idx_sessions_type_status ON sessions(session_type, status);
```

### 2.2 Indexes Rationale

- `idx_sessions_start_time`: Optimizes statistics queries filtering by date
- `idx_sessions_status`: Speeds up filtering completed vs. cancelled sessions
- `idx_sessions_type_status`: Optimizes counting work vs. break sessions

### 2.3 Data Retention

- Sessions are retained indefinitely (small data footprint)
- Future enhancement: Optional auto-cleanup of sessions older than N days
- Database size estimate: ~100 bytes per session → 1MB for ~10,000 sessions

---

## 3. State Management

### 3.1 Timer State Lifecycle

```
[No Active Timer]
        |
        | pomodoro start
        v
[TimerState Created] → status: Running
        |
        |-- pomodoro pause → status: Paused
        |                     |
        |                     | pomodoro resume
        |                     v
        |← ─ ─ ─ ─ ─ ─ ─ status: Running
        |
        | Timer expires (remaining_seconds = 0)
        v
[TimerState Deleted]
[Session Updated] → end_time: now(), status: Completed
        |
        v
[Send Notification]
[Play Sound (if enabled)]
```

### 3.2 Break Transition Logic

```
After completing work session:
    completed_work_count = count(work sessions today with status=completed)

    if completed_work_count % 4 == 0:
        next_break_type = long_break
    else:
        next_break_type = short_break

    prompt_user("Start {next_break_type}?")
```

### 3.3 Persistence Recovery

When user runs any command:
1. Load `timer_state` from database
2. If exists:
   - Calculate elapsed time since `updated_at`
   - If `status = Running`: subtract elapsed time from `remaining_seconds`
   - If `remaining_seconds <= 0`: Timer expired while CLI was closed → complete session
3. Display current timer status

This ensures timers continue running even when terminal is closed.

---

## 4. Data Access Patterns

### 4.1 Common Queries

**Get current timer state:**
```sql
SELECT * FROM timer_state WHERE id = 1;
```

**Get today's sessions:**
```sql
SELECT * FROM sessions
WHERE start_time >= ? AND start_time < ?
ORDER BY start_time DESC;
-- Parameters: start_of_day_timestamp, end_of_day_timestamp
```

**Count completed work sessions today:**
```sql
SELECT COUNT(*) FROM sessions
WHERE session_type = 'work'
  AND status = 'completed'
  AND start_time >= ?;
-- Parameter: start_of_day_timestamp
```

**Get recent session history (last 10):**
```sql
SELECT * FROM sessions
ORDER BY start_time DESC
LIMIT 10;
```

### 4.2 Transaction Boundaries

**Starting a timer:**
```sql
BEGIN TRANSACTION;
-- 1. Check no timer is running
SELECT id FROM timer_state WHERE id = 1;
-- 2. Create session
INSERT INTO sessions (...) VALUES (...);
-- 3. Create timer state
INSERT INTO timer_state (id, session_id, ...) VALUES (1, last_insert_rowid(), ...);
COMMIT;
```

**Completing a timer:**
```sql
BEGIN TRANSACTION;
-- 1. Update session
UPDATE sessions SET end_time = ?, status = 'completed' WHERE id = ?;
-- 2. Delete timer state
DELETE FROM timer_state WHERE id = 1;
COMMIT;
```

---

## 5. Migration Strategy

### 5.1 Initial Migration (001)

File: `migrations/001_initial_schema.sql`
- Creates `sessions`, `timer_state`, `preferences` tables
- Creates indexes
- Inserts default preferences

### 5.2 Future Migrations

- Each migration numbered sequentially (002, 003, ...)
- Track applied migrations in `schema_migrations` table:
```sql
CREATE TABLE schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);
```

- Application checks current schema version on startup
- Applies pending migrations in order

---

## Summary

- **3 core domain entities**: TimerSession, TimerState, UserStatistics
- **2 supporting entities**: TimerPreset, UserPreferences
- **3 database tables**: sessions, timer_state, preferences
- **Singleton constraint**: Only one active timer via `timer_state.id = 1`
- **State persistence**: Timers survive terminal closure via database
- **Break logic**: Long break after every 4 completed work sessions
- **Statistics**: Computed on-demand from session history
