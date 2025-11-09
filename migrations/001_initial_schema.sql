-- Sessions table: stores all timer sessions
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type TEXT NOT NULL CHECK(session_type IN ('work', 'short_break', 'long_break')),
    status TEXT NOT NULL CHECK(status IN ('active', 'paused', 'completed', 'cancelled')),
    duration_minutes INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    cancelled_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Timer state table: singleton to track current active timer
CREATE TABLE IF NOT EXISTS timer_state (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    session_id INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('running', 'paused', 'completed')),
    remaining_seconds INTEGER NOT NULL,
    last_updated_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

-- Preferences table: key-value storage for user preferences
CREATE TABLE IF NOT EXISTS preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);
CREATE INDEX IF NOT EXISTS idx_sessions_type ON sessions(session_type);
