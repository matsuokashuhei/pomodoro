# Quick Start Guide: CLI Pomodoro Timer Development

**Date**: November 9, 2025
**Feature**: CLI Pomodoro Timer
**Purpose**: Get developers up and running quickly with build, test, and contribution workflows

---

## Prerequisites

### Required

- **Rust**: 1.75 or later (latest stable recommended)
  ```bash
  # Install via rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # Verify installation
  rustc --version
  cargo --version
  ```

- **SQLite**: 3.35 or later (usually pre-installed on Linux/macOS)
  ```bash
  # Verify SQLite version
  sqlite3 --version
  ```

### Platform-Specific Requirements

**Linux (Debian/Ubuntu):**
```bash
# For desktop notifications (D-Bus)
sudo apt-get install libdbus-1-dev

# For audio playback (ALSA)
sudo apt-get install libasound2-dev
```

**macOS:**
```bash
# No additional dependencies (uses native frameworks)
```

**Windows:**
```bash
# Install Visual Studio Build Tools or Visual Studio with C++ support
# Download from: https://visualstudio.microsoft.com/downloads/
```

---

## Initial Setup

### 1. Clone Repository

```bash
git clone https://github.com/matsuokashuhei/pomodoro.git
cd pomodoro
git checkout 001-cli-pomodoro-timer
```

### 2. Build Project

```bash
# Debug build (fast compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release
```

**Expected output:**
```
   Compiling pomodoro v0.1.0 (/path/to/pomodoro)
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
```

**Binary location:**
- Debug: `target/debug/pomodoro`
- Release: `target/release/pomodoro`

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_timer_accuracy

# Run integration tests only
cargo test --test '*'
```

### 4. Install Locally

```bash
# Install to ~/.cargo/bin/
cargo install --path .

# Verify installation
pomodoro --version
```

---

## Development Workflow

### Project Structure Navigation

```
src/
├── main.rs              # Entry point - modify for new commands
├── models/              # Data structures - add new entities here
│   ├── session.rs       # TimerSession, SessionType, SessionStatus
│   ├── preset.rs        # Timer presets (standard/short/long)
│   └── statistics.rs    # Statistics calculation
├── services/            # Business logic
│   ├── timer.rs         # Core timer logic
│   ├── database.rs      # SQLite operations
│   ├── notifier.rs      # Desktop notifications
│   └── audio.rs         # Sound alert playback
├── cli/                 # CLI interface
│   ├── commands.rs      # Command handlers
│   └── display.rs       # Progress bar, formatting
└── config.rs            # User preferences

tests/
├── integration/         # Full flow tests
│   ├── timer_flow.rs
│   ├── persistence.rs
│   └── commands.rs
└── fixtures/            # Test data
```

### Common Tasks

#### Add a New Command

1. **Define command in `src/cli/commands.rs`:**
   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       // ... existing commands
       NewCommand {
           #[arg(short, long)]
           option: String,
       },
   }
   ```

2. **Implement handler in `src/cli/commands.rs`:**
   ```rust
   pub fn handle_new_command(option: &str) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

3. **Wire up in `src/main.rs`:**
   ```rust
   match cli.command {
       Commands::NewCommand { option } => {
           commands::handle_new_command(&option)?;
       }
       // ... existing matches
   }
   ```

4. **Add tests in `tests/integration/commands.rs`:**
   ```rust
   #[test]
   fn test_new_command() {
       Command::cargo_bin("pomodoro")
           .unwrap()
           .arg("new-command")
           .arg("--option")
           .arg("value")
           .assert()
           .success();
   }
   ```

#### Modify Timer Logic

Timer logic lives in `src/services/timer.rs`. Key functions:

- `TimerService::start_session()` - Create and start timer
- `TimerService::tick()` - Process one second tick
- `TimerService::check_completion()` - Check if timer expired

**Example: Add custom notification message**
```rust
// In src/services/timer.rs
impl TimerService {
    pub fn complete_session(&self, session: &TimerSession) -> Result<()> {
        let message = match session.session_type {
            SessionType::Work => "Great work! Time for a break.",
            SessionType::Break => "Break over. Ready to focus?",
        };

        self.notifier.send(message)?;
        // ... rest of completion logic
    }
}
```

#### Add Database Migration

1. **Create SQL file: `migrations/002_add_feature.sql`**
   ```sql
   -- Add new column
   ALTER TABLE sessions ADD COLUMN tags TEXT;

   -- Update schema version
   INSERT INTO schema_migrations (version, applied_at)
   VALUES (2, strftime('%s', 'now'));
   ```

2. **Update schema version check in `src/services/database.rs`**

3. **Test migration:**
   ```bash
   # Backup database
   cp ~/.local/share/pomodoro/sessions.db sessions.db.backup

   # Run application (auto-applies migrations)
   cargo run -- status

   # Verify schema
   sqlite3 ~/.local/share/pomodoro/sessions.db ".schema sessions"
   ```

---

## Testing Strategy

### Unit Tests

Test individual functions and logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_tick() {
        let mut state = TimerState::new(1, 1500);
        assert_eq!(state.remaining_seconds, 1500);

        state.tick();
        assert_eq!(state.remaining_seconds, 1499);
    }

    #[test]
    fn test_preset_durations() {
        let standard = TimerPreset::standard();
        assert_eq!(standard.work_minutes, 25);
        assert_eq!(standard.short_break_minutes, 5);
        assert_eq!(standard.long_break_minutes, 15);
    }
}
```

### Integration Tests

Test full command flows:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_start_and_status() {
    // Start timer
    Command::cargo_bin("pomodoro")
        .unwrap()
        .arg("start")
        .assert()
        .success()
        .stdout(predicate::str::contains("Work session started"));

    // Check status
    Command::cargo_bin("pomodoro")
        .unwrap()
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Work Session"));
}
```

### Test Database Isolation

Use test database to avoid polluting user data:

```rust
#[test]
fn test_with_isolated_db() {
    // Set test database path
    std::env::set_var("POMODORO_DB_PATH", "/tmp/test_pomodoro.db");

    // Run test
    // ...

    // Cleanup
    std::fs::remove_file("/tmp/test_pomodoro.db").ok();
}
```

---

## Code Quality Checks

### Run All Quality Checks

```bash
# Format code
cargo fmt

# Check formatting (CI mode)
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter with warnings as errors (CI mode)
cargo clippy -- -D warnings

# Run static analysis
cargo check

# Full CI check
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
```

### Pre-Commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format check
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "❌ Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Clippy
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Clippy checks failed. Fix warnings before committing."
    exit 1
fi

# Tests
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "❌ Tests failed. Fix failing tests before committing."
    exit 1
fi

echo "✅ All checks passed!"
```

Make executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## Debugging

### Enable Verbose Logging

```bash
# Set log level
RUST_LOG=debug cargo run -- start

# Or use verbose flag (once implemented)
cargo run -- start --verbose
```

### Inspect Database

```bash
# Open database in SQLite shell
sqlite3 ~/.local/share/pomodoro/sessions.db

# Useful queries
.schema                              # Show all tables
SELECT * FROM timer_state;           # Current timer
SELECT * FROM sessions ORDER BY start_time DESC LIMIT 10;  # Recent sessions
SELECT COUNT(*) FROM sessions WHERE status = 'completed';  # Total completed
```

### Debug Timer State

```bash
# Check if timer state exists
sqlite3 ~/.local/share/pomodoro/sessions.db "SELECT * FROM timer_state;"

# Check last session
sqlite3 ~/.local/share/pomodoro/sessions.db \
  "SELECT * FROM sessions ORDER BY start_time DESC LIMIT 1;"
```

### Reset Everything

```bash
# Remove database and config (fresh start)
rm -rf ~/.local/share/pomodoro/

# Rebuild and run
cargo build && cargo run -- start
```

---

## Performance Profiling

### Build with Profiling Symbols

```toml
# Add to Cargo.toml
[profile.release]
debug = true  # Include debug symbols in release build
```

### Benchmark Timer Accuracy

```rust
use std::time::{Duration, Instant};

#[test]
fn benchmark_timer_accuracy() {
    let start = Instant::now();
    let mut state = TimerState::new(1, 60);

    for _ in 0..60 {
        std::thread::sleep(Duration::from_secs(1));
        state.tick();
    }

    let elapsed = start.elapsed();
    let drift = elapsed.as_secs() as i64 - 60;

    println!("Expected: 60s, Actual: {}s, Drift: {}s", elapsed.as_secs(), drift);
    assert!(drift.abs() <= 1, "Timer drift exceeded 1 second");
}
```

---

## Common Issues & Solutions

### Issue: "Failed to open database"

**Cause**: Database directory doesn't exist
**Solution**:
```bash
mkdir -p ~/.local/share/pomodoro/
```

### Issue: "No such file: libdbus-1.so"

**Cause**: Missing D-Bus library (Linux)
**Solution**:
```bash
sudo apt-get install libdbus-1-dev
cargo clean
cargo build
```

### Issue: Notifications not appearing

**Cause**: Notification permissions or desktop environment issue
**Debug**:
```bash
# Test notification directly
notify-send "Test" "This is a test notification"

# Check notification daemon
ps aux | grep notif
```

### Issue: Audio not playing

**Cause**: Missing audio backend or permissions
**Debug**:
```bash
# Linux: Check ALSA
aplay /usr/share/sounds/alsa/Front_Center.wav

# macOS: Check system audio
afplay /System/Library/Sounds/Ping.aiff
```

---

## Release Process

### 1. Version Bump

Update `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

### 2. Create Release Build

```bash
# Build optimized binary
cargo build --release --locked

# Strip binary (reduce size)
strip target/release/pomodoro

# Check size
ls -lh target/release/pomodoro
```

### 3. Cross-Compile for Other Platforms

```bash
# Install cross-compilation tool
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for macOS
cross build --release --target x86_64-apple-darwin

# Build for Windows
cross build --release --target x86_64-pc-windows-msvc
```

### 4. Create GitHub Release

```bash
# Tag release
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# Create release with binaries (use GitHub CLI or web UI)
gh release create v0.2.0 \
  target/release/pomodoro-linux \
  target/release/pomodoro-macos \
  target/release/pomodoro-windows.exe
```

---

## Contributing Guidelines

### Code Style

- Follow Rust standard style (`cargo fmt`)
- Maximum line length: 100 characters
- Use descriptive variable names
- Add doc comments for public APIs

### Commit Messages

Format: `<type>: <description>`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code refactoring
- `perf`: Performance improvement

Example:
```
feat: add long break after 4 completed sessions

Implements FR-003 by tracking completed work sessions and automatically
triggering a 15-minute long break after every 4 sessions.
```

### Pull Request Process

1. Create feature branch: `git checkout -b feat/my-feature`
2. Make changes and commit
3. Run full test suite: `cargo test`
4. Run quality checks: `cargo fmt && cargo clippy`
5. Push and create PR
6. Address review feedback
7. Squash commits before merge

---

## Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [clap Documentation](https://docs.rs/clap/)
- [rusqlite Documentation](https://docs.rs/rusqlite/)

### Project Documentation

- [Feature Specification](./spec.md)
- [Implementation Plan](./plan.md)
- [Data Model](./data-model.md)
- [CLI Commands Contract](./contracts/cli-commands.md)

### Getting Help

- Check existing issues: [GitHub Issues](https://github.com/matsuokashuhei/pomodoro/issues)
- Ask questions: [GitHub Discussions](https://github.com/matsuokashuhei/pomodoro/discussions)

---

**Next Steps**: After setup is complete, review the [Implementation Plan](./plan.md) for detailed architecture and proceed with development tasks.
