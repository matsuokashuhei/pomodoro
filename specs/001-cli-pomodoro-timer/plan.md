# Implementation Plan: CLI Pomodoro Timer

**Branch**: `001-cli-pomodoro-timer` | **Date**: November 9, 2025 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-cli-pomodoro-timer/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a command-line Pomodoro timer in Rust that manages work/break sessions with configurable presets, progress visualization, desktop notifications, and SQLite-based state persistence. The tool will support standard Pomodoro intervals (25/5/15 minutes) plus short and long presets, with pause/resume capabilities and session statistics tracking.

## Technical Context

**Language/Version**: Rust 1.75+ (latest stable)
**Primary Dependencies**:
- `clap` (CLI argument parsing)
- `rusqlite` (SQLite database access)
- `notify-rust` (desktop notifications)
- `tokio` (async runtime for timers)
- `indicatif` (progress bars and spinners)
- `chrono` (date/time handling)
- `serde` / `serde_json` (configuration serialization)
- `rodio` (audio playback for optional sound alerts)

**Storage**: SQLite database for session history and state persistence
**Testing**: `cargo test` (unit tests), integration tests in `tests/` directory
**Target Platform**: Linux, macOS, Windows (cross-platform CLI)
**Project Type**: Single binary CLI application
**Performance Goals**:
- Command response time <100ms for all operations
- Timer tick accuracy ±1 second over 25 minutes
- Notification delivery <2 seconds after completion
- Database operations <50ms

**Constraints**:
- Must survive terminal closure (background daemon or state persistence)
- Memory usage <50MB during operation
- Must work offline (no network dependencies)
- Single user, single machine (no distributed state)

**Scale/Scope**:
- Small codebase (<5k LOC)
- Single executable binary
- Local SQLite database (<100MB)
- Hundreds of sessions per day maximum

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Code Quality Discipline** — Will use `rustfmt` for formatting, `clippy` for linting, and `cargo check` for static analysis. All code must pass `cargo clippy -- -D warnings` before merge. Complex timer logic and state management will include doc comments explaining Pomodoro technique rules and persistence strategies.

- [x] **Testing Reliability** — Unit tests for timer calculations, state transitions, and database operations. Integration tests for full command flows (start→complete, pause→resume, cancel). Regression tests for edge cases: concurrent timer prevention, terminal closure recovery, time accuracy over long durations. Critical flows covered: start work session, complete with notification, break transitions, 4-session long break trigger.

- [x] **User Experience Consistency** — CLI will use consistent command patterns (`pomodoro start`, `pomodoro pause`, etc.). Progress bar updates every second with MM:SS format. Status messages appear immediately (<100ms). Notifications follow system native patterns. Keyboard-accessible (CLI inherently keyboard-driven). Clear error messages for invalid states (e.g., "Timer already running").

- [x] **Performance Accountability** — Performance targets documented in Technical Context: <100ms command response, ±1s timer accuracy, <2s notification delivery, <50ms database ops. Will benchmark timer tick precision over 25-minute runs. Memory usage monitored via `cargo build --release` metrics. Performance regression detected via integration test timing assertions.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
pomodoro/
├── Cargo.toml              # Project manifest with dependencies
├── Cargo.lock              # Dependency lock file
├── src/
│   ├── main.rs            # CLI entry point, argument parsing
│   ├── lib.rs             # Library exports for testing
│   ├── models/
│   │   ├── mod.rs         # Module declarations
│   │   ├── session.rs     # TimerSession, SessionType, SessionStatus
│   │   ├── preset.rs      # TimerPreset (standard/short/long)
│   │   └── statistics.rs  # UserStatistics aggregate data
│   ├── services/
│   │   ├── mod.rs
│   │   ├── timer.rs       # Core timer logic, tick management
│   │   ├── database.rs    # SQLite operations, migrations
│   │   ├── notifier.rs    # Desktop notification integration
│   │   └── audio.rs       # Optional sound alert playback
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands.rs    # Command definitions (start, pause, status, etc.)
│   │   └── display.rs     # Progress bar, status formatting
│   └── config.rs          # User preferences, preset selection
├── tests/
│   ├── integration/
│   │   ├── timer_flow.rs  # Full work/break cycle tests
│   │   ├── persistence.rs # State survival tests
│   │   └── commands.rs    # CLI command integration tests
│   └── fixtures/
│       └── test.db        # Test database setup
├── migrations/
│   └── 001_initial_schema.sql  # Database schema
└── README.md              # Build and usage instructions
```

**Structure Decision**: Single Rust project with binary crate. Using standard Cargo layout with domain-driven module organization: `models/` for data structures, `services/` for business logic, `cli/` for user interface. Integration tests in `tests/` directory can import from `src/lib.rs`. Database migrations tracked separately for schema evolution.

## Complexity Tracking

No constitution violations. All gates passed with appropriate implementation strategies.
