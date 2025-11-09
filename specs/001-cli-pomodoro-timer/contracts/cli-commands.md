# CLI Commands Contract: Pomodoro Timer

**Date**: November 9, 2025
**Feature**: CLI Pomodoro Timer
**Purpose**: Define command-line interface contracts, input/output formats, and error handling

---

## Command Overview

All commands follow the pattern: `pomodoro <command> [options]`

### Command List

| Command | Purpose | Priority |
|---------|---------|----------|
| `start` | Start a new work or break session | P1 |
| `pause` | Pause the current running timer | P3 |
| `resume` | Resume a paused timer | P3 |
| `cancel` | Cancel the current timer session | P3 |
| `status` | Display current timer status | P1 |
| `stats` | Display session statistics | P3 |
| `config` | View or modify preferences | P2 |

---

## 1. `pomodoro start`

Start a new timer session (work or break).

### Syntax

```bash
pomodoro start [--type <work|break>] [--preset <standard|short|long>] [--duration <minutes>]
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--type`, `-t` | `work \| break` | `work` | Type of session to start |
| `--preset`, `-p` | `standard \| short \| long` | From config | Timer preset to use |
| `--duration`, `-d` | Integer (1-120) | From preset | Override duration in minutes |

### Examples

```bash
# Start a standard work session (25 minutes)
pomodoro start

# Start a break session using standard preset (5 minutes)
pomodoro start --type break

# Start a work session with short preset (15 minutes)
pomodoro start --preset short

# Start a custom 30-minute work session
pomodoro start --duration 30
```

### Success Output

```
🍅 Work session started (25:00)
Press Ctrl+C to pause, or run 'pomodoro status' to check progress
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| Timer already running | 1 | `Error: A timer is already running. Use 'pomodoro status' to check or 'pomodoro cancel' to stop it.` |
| Invalid duration | 2 | `Error: Duration must be between 1 and 120 minutes` |
| Invalid type | 2 | `Error: Type must be 'work' or 'break'` |
| Invalid preset | 2 | `Error: Preset must be 'standard', 'short', or 'long'` |
| Database error | 3 | `Error: Failed to save timer state: <details>` |

### State Changes

- Creates new `TimerSession` record with `end_time = NULL`
- Creates `TimerState` singleton record with status `Running`
- Returns immediately (timer runs in background)

### Functional Requirements Mapping

- FR-001: Start work session with 25-minute default
- FR-002: Start break timer
- FR-015: Support preset selection
- FR-016: Duration customization via `--duration`

---

## 2. `pomodoro pause`

Pause the currently running timer.

### Syntax

```bash
pomodoro pause
```

### Options

None.

### Examples

```bash
pomodoro pause
```

### Success Output

```
⏸️  Timer paused at 18:32 remaining
Use 'pomodoro resume' to continue
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| No active timer | 1 | `Error: No timer is currently running` |
| Timer already paused | 1 | `Error: Timer is already paused` |
| Database error | 3 | `Error: Failed to save pause state: <details>` |

### State Changes

- Updates `TimerState.status` to `Paused`
- Updates `TimerState.updated_at` to current timestamp
- Preserves `remaining_seconds`

### Functional Requirements Mapping

- FR-006: Pause active timer

---

## 3. `pomodoro resume`

Resume a paused timer.

### Syntax

```bash
pomodoro resume
```

### Options

None.

### Examples

```bash
pomodoro resume
```

### Success Output

```
▶️  Timer resumed with 18:32 remaining
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| No active timer | 1 | `Error: No timer is currently paused` |
| Timer not paused | 1 | `Error: Timer is already running` |
| Database error | 3 | `Error: Failed to resume timer: <details>` |

### State Changes

- Updates `TimerState.status` to `Running`
- Updates `TimerState.updated_at` to current timestamp
- Timer countdown resumes from `remaining_seconds`

### Functional Requirements Mapping

- FR-007: Resume paused timer

---

## 4. `pomodoro cancel`

Cancel the current timer session.

### Syntax

```bash
pomodoro cancel [--force]
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--force`, `-f` | Flag | false | Skip confirmation prompt |

### Examples

```bash
# With confirmation prompt
pomodoro cancel

# Skip confirmation
pomodoro cancel --force
```

### Success Output

```
Cancel current timer? (y/N): y
❌ Timer cancelled
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| No active timer | 1 | `Error: No timer is currently active` |
| User declines | 0 | `Timer not cancelled` |
| Database error | 3 | `Error: Failed to cancel timer: <details>` |

### State Changes

- Updates `TimerSession.end_time` to current timestamp
- Updates `TimerSession.status` to `Cancelled`
- Deletes `TimerState` record
- Session not counted toward statistics

### Functional Requirements Mapping

- FR-008: Cancel active timer

---

## 5. `pomodoro status`

Display current timer status and remaining time.

### Syntax

```bash
pomodoro status [--json]
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--json` | Flag | false | Output in JSON format |

### Examples

```bash
# Human-readable output
pomodoro status

# JSON output for scripting
pomodoro status --json
```

### Success Output (Running Timer)

```
🍅 Work Session (Standard Preset)
[█████████████░░░░░░░] 18:32 remaining
Status: Running
Started: 2025-11-09 14:30:00
```

### Success Output (Paused Timer)

```
⏸️  Work Session (Standard Preset)
[█████████████░░░░░░░] 18:32 remaining
Status: Paused
Started: 2025-11-09 14:30:00
```

### Success Output (No Timer)

```
💤 No timer active
Use 'pomodoro start' to begin a session
```

### JSON Output Format

```json
{
  "active": true,
  "session_type": "work",
  "preset": "standard",
  "duration_minutes": 25,
  "remaining_seconds": 1112,
  "status": "running",
  "start_time": "2025-11-09T14:30:00Z"
}
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| Database error | 3 | `Error: Failed to read timer state: <details>` |

### State Changes

None (read-only operation)

### Functional Requirements Mapping

- FR-004: Display remaining time
- FR-012: Check timer status
- FR-017: Display time in MM:SS with progress bar

---

## 6. `pomodoro stats`

Display session statistics.

### Syntax

```bash
pomodoro stats [--date <YYYY-MM-DD>] [--json]
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--date`, `-d` | Date string | Today | Show stats for specific date |
| `--json` | Flag | false | Output in JSON format |

### Examples

```bash
# Today's statistics
pomodoro stats

# Specific date
pomodoro stats --date 2025-11-08

# JSON format
pomodoro stats --json
```

### Success Output

```
📊 Statistics for 2025-11-09

Work Sessions:     6 completed
Break Sessions:    5 completed
Total Focus Time:  2h 30m
Total Break Time:  25m
Cancelled:         1 session
Current Streak:    3 sessions

Recent Sessions:
  14:30 - 14:55  Work (25m) ✓
  14:55 - 15:00  Break (5m) ✓
  15:00 - 15:25  Work (25m) ✓
  ...
```

### JSON Output Format

```json
{
  "date": "2025-11-09",
  "completed_work_sessions": 6,
  "completed_break_sessions": 5,
  "total_work_minutes": 150,
  "total_break_minutes": 25,
  "cancelled_sessions": 1,
  "current_streak": 3,
  "recent_sessions": [
    {
      "start_time": "2025-11-09T14:30:00Z",
      "end_time": "2025-11-09T14:55:00Z",
      "type": "work",
      "duration_minutes": 25,
      "status": "completed"
    }
  ]
}
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| Invalid date format | 2 | `Error: Date must be in YYYY-MM-DD format` |
| Database error | 3 | `Error: Failed to retrieve statistics: <details>` |

### State Changes

None (read-only operation)

### Functional Requirements Mapping

- FR-009: Track and display completed sessions
- FR-014: View session statistics for current day

---

## 7. `pomodoro config`

View or modify user preferences.

### Syntax

```bash
pomodoro config [--set <key> <value>] [--get <key>] [--list]
```

### Options

| Option | Type | Description |
|--------|------|-------------|
| `--set`, `-s` | key value | Set configuration value |
| `--get`, `-g` | key | Get configuration value |
| `--list`, `-l` | Flag | List all configuration |

### Configuration Keys

| Key | Type | Values | Default | Description |
|-----|------|--------|---------|-------------|
| `preset` | string | `standard`, `short`, `long` | `standard` | Default timer preset |
| `sound_enabled` | boolean | `true`, `false` | `true` | Enable sound alerts |
| `notification_enabled` | boolean | `true`, `false` | `true` | Enable desktop notifications |
| `custom_sound_path` | string | File path | null | Custom alert sound file |

### Examples

```bash
# List all configuration
pomodoro config --list

# Get specific value
pomodoro config --get preset

# Set preset to short
pomodoro config --set preset short

# Enable sound alerts
pomodoro config --set sound_enabled true

# Set custom sound
pomodoro config --set custom_sound_path /path/to/alert.mp3
```

### Success Output (List)

```
Current Configuration:
  preset: standard
  sound_enabled: true
  notification_enabled: true
  custom_sound_path: (not set)
```

### Success Output (Get)

```
preset: standard
```

### Success Output (Set)

```
✓ Configuration updated: preset = short
```

### Error Cases

| Error | Exit Code | Output |
|-------|-----------|--------|
| Invalid key | 2 | `Error: Unknown configuration key: <key>` |
| Invalid value | 2 | `Error: Invalid value for <key>: <value>` |
| Invalid file path | 2 | `Error: File not found: <path>` |
| Database error | 3 | `Error: Failed to save configuration: <details>` |

### State Changes

- Updates `preferences` table with new values
- Changes take effect immediately for new timer sessions
- Does not affect currently running timer

### Functional Requirements Mapping

- FR-016: Select preset
- FR-020: Enable/disable sound alerts

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | User error (invalid state, user cancelled) |
| 2 | Invalid input (bad arguments, validation failure) |
| 3 | System error (database, I/O, external dependency failure) |

---

## Global Options

Available for all commands:

| Option | Description |
|--------|-------------|
| `--help`, `-h` | Show help message |
| `--version`, `-V` | Show version information |
| `--verbose`, `-v` | Enable verbose logging |

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `POMODORO_DB_PATH` | Override database location | `~/.local/share/pomodoro/sessions.db` |
| `POMODORO_CONFIG_PATH` | Override config location | `~/.local/share/pomodoro/config.json` |
| `POMODORO_LOG_LEVEL` | Set log level | `info` |

---

## Timer Background Process

### Implementation Note

The timer must continue running even when the CLI exits. Two approaches:

**Approach 1: State-based (Chosen)**
- CLI writes timer state to database and exits immediately
- Any future CLI invocation recalculates remaining time based on elapsed time
- No background process needed
- Pros: Simple, robust, survives system restarts
- Cons: Requires active polling for notifications

**Approach 2: Background daemon**
- Start background process on `pomodoro start`
- Daemon handles timer ticks and notifications
- CLI communicates via IPC (sockets/files)
- Pros: Real-time notifications
- Cons: More complex, process management issues

**Chosen**: Approach 1 for simplicity and robustness. Notifications handled by periodic check or on next CLI invocation.

---

## Notification Triggers

Notifications sent when:
1. Work session completes → "Time for a break!"
2. Break completes → "Ready to work?"
3. Long break available → "Time for a long break!"

### Notification Format

```rust
Notification::new()
    .summary("Pomodoro Timer")
    .body(message)
    .icon("timer")
    .urgency(Urgency::Critical)
    .timeout(5000)  // 5 seconds
    .show()?;
```

### Sound Alert

If enabled, play sound after notification:
- Default: Embedded bell sound (assets/alert.wav)
- Custom: User-specified file from config

---

## Summary

- **7 primary commands**: start, pause, resume, cancel, status, stats, config
- **State-based timer**: No background daemon, robust persistence
- **Rich output**: Progress bars, emojis, color-coded status
- **JSON support**: Machine-readable output for scripting
- **Comprehensive errors**: Clear messages with exit codes
- **Cross-platform**: Works identically on Linux, macOS, Windows
