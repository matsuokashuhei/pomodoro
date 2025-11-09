# Research & Technology Decisions: CLI Pomodoro Timer

**Date**: November 9, 2025
**Feature**: CLI Pomodoro Timer
**Purpose**: Document technology choices, architecture decisions, and best practices for Rust-based CLI timer implementation

---

## 1. Audio Playback Library Selection

### Decision: **rodio**

### Rationale:
- **Cross-platform**: Works on Linux, macOS, and Windows without platform-specific code
- **Simple API**: Easy to play audio files with minimal setup (`rodio::Sink` for playback control)
- **Low overhead**: Suitable for occasional sound alerts, minimal memory footprint
- **Active maintenance**: Well-maintained crate with good community support
- **No external dependencies**: Doesn't require system audio frameworks beyond OS defaults

### Alternatives Considered:

| Library | Pros | Cons | Rejection Reason |
|---------|------|------|------------------|
| **soloud** | Feature-rich, game audio experience | C++ bindings, heavier weight, overkill for simple alerts | Unnecessary complexity for basic alert sounds |
| **cpal** | Low-level audio control, very flexible | Requires more manual setup, complex API for simple use case | Too low-level, would need to implement playback logic |
| **ears** | Simple API, easy to use | Less maintained, smaller community | Less reliable long-term support |

### Implementation Notes:
- Embed default alert sound as bytes using `include_bytes!` macro
- Support custom sound files via configuration
- Graceful fallback if audio system unavailable (silent notification)

---

## 2. CLI Framework Best Practices (clap)

### Decision: **clap v4 with derive macros**

### Rationale:
- **Type-safe**: Compile-time validation of CLI structure
- **Derive API**: Clean, declarative command definitions with `#[derive(Parser)]`
- **Subcommands**: Natural fit for `pomodoro start`, `pomodoro pause`, etc.
- **Help generation**: Automatic `--help` with rich formatting
- **Shell completion**: Built-in support for bash/zsh/fish completions

### Best Practices:
```rust
#[derive(Parser)]
#[command(name = "pomodoro")]
#[command(about = "CLI Pomodoro timer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { /* options */ },
    Pause,
    Resume,
    Cancel,
    Status,
    Stats,
}
```

### Alternatives Considered:
- **structopt**: Predecessor to clap v3+, now merged into clap
- **argh**: Lighter weight but less feature-complete

---

## 3. State Persistence Strategy (SQLite + rusqlite)

### Decision: **SQLite with rusqlite crate**

### Rationale:
- **Embedded database**: No separate server process, perfect for single-user CLI
- **ACID transactions**: Ensures data integrity even with abrupt termination
- **Cross-platform**: Works identically on all target platforms
- **Efficient**: Sub-50ms query times for small datasets
- **Schema evolution**: Migration support via SQL scripts

### Database Location:
- **Linux/macOS**: `~/.local/share/pomodoro/sessions.db` (XDG Base Directory)
- **Windows**: `%APPDATA%\pomodoro\sessions.db`
- Use `dirs` crate for cross-platform path resolution

### Schema Design:
```sql
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY,
    session_type TEXT NOT NULL,  -- 'work' or 'break'
    preset TEXT NOT NULL,         -- 'standard', 'short', 'long'
    duration_minutes INTEGER NOT NULL,
    start_time INTEGER NOT NULL,  -- Unix timestamp
    end_time INTEGER,             -- NULL if not completed
    status TEXT NOT NULL,         -- 'completed', 'cancelled'
    created_at INTEGER NOT NULL
);

CREATE TABLE timer_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Singleton row
    session_id INTEGER,
    remaining_seconds INTEGER,
    status TEXT,  -- 'running', 'paused', NULL
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE TABLE preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

### Alternatives Considered:
- **JSON files**: Simpler but no transactions, risk of corruption
- **PostgreSQL**: Overkill for single-user, requires separate installation

---

## 4. Async Runtime Strategy (tokio)

### Decision: **tokio with minimal features**

### Rationale:
- **Timer primitives**: `tokio::time::interval` for accurate tick generation
- **Async notifications**: Non-blocking desktop notification delivery
- **Standard in ecosystem**: Most Rust async code uses tokio
- **Feature flags**: Can enable only `time` and `rt` features to minimize binary size

### Timer Implementation Pattern:
```rust
use tokio::time::{interval, Duration};

async fn run_timer(duration_secs: u64) {
    let mut ticker = interval(Duration::from_secs(1));
    let mut remaining = duration_secs;

    while remaining > 0 {
        ticker.tick().await;
        remaining -= 1;
        update_display(remaining);
    }

    send_notification().await;
}
```

### Alternatives Considered:
- **async-std**: Similar functionality but smaller ecosystem
- **Synchronous approach**: Would block on notification delivery, less responsive

---

## 5. Progress Bar & Terminal UI (indicatif)

### Decision: **indicatif**

### Rationale:
- **Rich progress bars**: Smooth visual feedback with spinners and bars
- **Template system**: Customizable format strings for time display
- **Non-blocking**: Updates don't interfere with other operations
- **Widely used**: Battle-tested in many CLI tools (e.g., cargo itself)

### Display Format:
```text
🍅 Work Session [█████████████░░░░░░░] 15:23 remaining
```

### Alternatives Considered:
- **pbr**: Less feature-rich, fewer customization options
- **Custom terminal control**: Too much low-level work, reinventing the wheel

---

## 6. Desktop Notification Strategy (notify-rust)

### Decision: **notify-rust**

### Rationale:
- **Cross-platform**: Uses native notification systems (D-Bus on Linux, NSUserNotificationCenter on macOS, WinRT on Windows)
- **Simple API**: One-line notification delivery
- **Urgency levels**: Can set critical priority for timer completions
- **Timeout control**: Can specify notification duration

### Implementation:
```rust
use notify_rust::Notification;

Notification::new()
    .summary("Pomodoro Complete")
    .body("Time for a break!")
    .urgency(notify_rust::Urgency::Critical)
    .show()?;
```

### Alternatives Considered:
- **Manual system calls**: Platform-specific, high maintenance burden
- **No notifications**: Violates FR-018 requirement

---

## 7. Date/Time Handling (chrono)

### Decision: **chrono**

### Rationale:
- **Timestamp management**: Easy Unix timestamp conversion for database storage
- **Duration arithmetic**: Calculate session lengths and statistics
- **Time zone awareness**: Handle daylight saving transitions correctly
- **Industry standard**: Most widely used Rust date/time library

### Alternatives Considered:
- **time crate**: Newer, more focused, but less ecosystem support currently
- **std::time**: Too low-level for date calculations

---

## 8. Configuration Management

### Decision: **JSON files with serde_json**

### Rationale:
- **Human-readable**: Easy for users to edit preferences
- **Schema validation**: serde provides type safety
- **Simple**: No need for complex config formats (TOML/YAML overkill for few settings)

### Configuration Structure:
```json
{
  "preset": "standard",
  "sound_enabled": true,
  "custom_sound_path": null,
  "notification_enabled": true
}
```

### Location: Same directory as database (`~/.local/share/pomodoro/config.json`)

---

## 9. Error Handling Strategy

### Decision: **anyhow for application errors, thiserror for library errors**

### Rationale:
- **anyhow**: Simple error handling in main.rs and CLI commands, good error messages
- **thiserror**: Type-safe errors for library code (models, services)
- **Context propagation**: Use `.context()` to add helpful error messages

### Pattern:
```rust
use anyhow::{Context, Result};

fn start_timer() -> Result<()> {
    let db = open_database()
        .context("Failed to open sessions database")?;
    // ...
}
```

---

## 10. Testing Strategy

### Decision: **Multi-layer testing approach**

### Unit Tests:
- Timer calculation logic (duration, remaining time)
- State transition validation
- Preset configuration parsing

### Integration Tests:
- Full command flows using `assert_cmd` crate
- Database persistence across restarts
- Multi-session scenarios (4 sessions → long break)

### Test Database:
- Use in-memory SQLite (`:memory:`) for fast tests
- Fixture data in `tests/fixtures/`

### Mocking:
- Mock notification system to avoid desktop spam during tests
- Mock audio playback to avoid sound during CI

---

## Summary of Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.x | CLI argument parsing |
| rusqlite | 0.30+ | SQLite database access |
| tokio | 1.x | Async runtime for timers |
| indicatif | 0.17+ | Progress bars and spinners |
| notify-rust | 4.x | Desktop notifications |
| rodio | 0.17+ | Audio playback |
| chrono | 0.4+ | Date/time handling |
| serde / serde_json | 1.x | Configuration serialization |
| anyhow | 1.x | Error handling |
| dirs | 5.x | Cross-platform directory paths |

### Development Dependencies:
- assert_cmd | 2.x | CLI testing
- predicates | 3.x | Test assertions

---

## Performance Benchmarking Plan

1. **Timer Accuracy**: Run 25-minute sessions, measure drift from expected completion time
2. **Command Response**: Time each CLI command execution (target <100ms)
3. **Database Operations**: Profile SQLite query times under load (100s of sessions)
4. **Memory Usage**: Monitor RSS during long-running timer with `cargo build --release`

---

## Security Considerations

1. **SQL Injection**: rusqlite parameterized queries prevent injection (no raw SQL from user input)
2. **File Permissions**: Ensure database file has user-only read/write (0600 on Unix)
3. **No Network**: Offline-only tool reduces attack surface
4. **No Sensitive Data**: Only stores session times, no PII

---

## Deployment & Distribution

### Build Targets:
- x86_64-unknown-linux-gnu (Linux)
- x86_64-apple-darwin (macOS Intel)
- aarch64-apple-darwin (macOS Apple Silicon)
- x86_64-pc-windows-msvc (Windows)

### Distribution:
- GitHub Releases with pre-built binaries
- cargo install pomodoro (if published to crates.io)
- Package managers: homebrew (macOS), AUR (Arch Linux), chocolatey (Windows)

### Binary Size Optimization:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

Expected binary size: 3-5 MB (statically linked, all dependencies included)

---

**Conclusion**: All technical uncertainties resolved. Rust ecosystem provides mature, cross-platform solutions for all requirements. Ready to proceed to Phase 1 design.
