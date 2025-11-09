# 🍅 Pomodoro Timer CLI

A command-line Pomodoro timer to boost your productivity with the Pomodoro Technique. Track work sessions, take breaks, and monitor your productivity statistics - all from your terminal.

## Features

- ⏰ **Start/Stop Timer**: Begin work or break sessions with customizable durations
- ⏸️ **Pause/Resume**: Handle interruptions without losing your session
- 📊 **Statistics**: Track completed sessions, work time, and productivity streaks
- 🔔 **Notifications**: Get desktop notifications when sessions complete
- 🎵 **Sound Alerts**: Audio notifications (with custom sound support)
- ⚙️ **Configurable**: Customize durations, presets, and preferences
- 💾 **Persistent State**: Session state survives across CLI invocations
- 🌈 **Pretty Display**: Clean, emoji-enhanced terminal UI

## Installation

### Prerequisites

- Rust 1.75 or later
- ALSA development libraries (Linux)
- DBus (for desktop notifications on Linux)

On Debian/Ubuntu:
```bash
apt-get install libasound2-dev libdbus-1-dev
```

### Build from Source

```bash
git clone https://github.com/yourusername/pomodoro.git
cd pomodoro
cargo build --release
```

The binary will be available at `target/release/pomodoro`. You can copy it to a directory in your PATH:

```bash
sudo cp target/release/pomodoro /usr/local/bin/
```

## Quick Start

### Start a Work Session

```bash
# Start with default preset (25 minutes)
pomodoro start

# Start with a specific preset
pomodoro start --preset short    # 15 minutes
pomodoro start --preset long     # 50 minutes
```

### Check Timer Status

```bash
pomodoro status

# Output in JSON format
pomodoro status --json
```

### Pause and Resume

```bash
# Pause the current timer
pomodoro pause

# Resume a paused timer
pomodoro resume
```

### Cancel a Session

```bash
# Cancel with confirmation
pomodoro cancel

# Cancel without confirmation
pomodoro cancel --force
```

### View Statistics

```bash
# Show today's statistics
pomodoro stats

# Show stats for a specific date
pomodoro stats --date 2025-11-09

# Output in JSON format
pomodoro stats --json
```

### Configuration

```bash
# List all configuration
pomodoro config --list

# Get a specific value
pomodoro config --get default_preset

# Set a value
pomodoro config --set default_preset=short
pomodoro config --set work_minutes=30
pomodoro config --set sound_enabled=false
```

## Usage

### The Pomodoro Technique

1. Start a 25-minute work session
2. Work without interruption
3. Take a 5-minute break when the timer completes
4. After 4 work sessions, take a 15-minute long break

### Commands

| Command | Description |
|---------|-------------|
| `start` | Start a new timer session |
| `status` | Show current timer status |
| `pause` | Pause the current timer |
| `resume` | Resume a paused timer |
| `cancel` | Cancel the current session |
| `stats` | Show session statistics |
| `config` | Manage configuration |

### Start Options

- `--type <TYPE>`: Session type (`work`, `break`, `short_break`, `long_break`)
- `--preset <PRESET>`: Duration preset (`standard`, `short`, `long`)

### Configuration Keys

- `default_preset`: Default preset to use (`standard`, `short`, `long`)
- `sound_enabled`: Enable/disable sound alerts (`true`, `false`)
- `notification_enabled`: Enable/disable desktop notifications (`true`, `false`)
- `custom_sound_path`: Path to custom alert sound file
- `work_minutes`: Override work session duration
- `short_break_minutes`: Override short break duration
- `long_break_minutes`: Override long break duration

## Presets

| Preset | Work | Short Break | Long Break |
|--------|------|-------------|------------|
| Standard | 25 min | 5 min | 15 min |
| Short | 15 min | 3 min | 10 min |
| Long | 50 min | 10 min | 30 min |

## Environment Variables

- `POMODORO_DB_PATH`: Custom path for the database file
- `POMODORO_CONFIG_PATH`: Custom path for the configuration file
- `POMODORO_DATA_DIR`: Custom directory for all data files

Default locations:
- Linux: `~/.local/share/pomodoro/`
- macOS: `~/Library/Application Support/pomodoro/`
- Windows: `%APPDATA%\pomodoro\`

## Examples

### Typical Workflow

```bash
# Start a work session
pomodoro start

# Check progress
pomodoro status

# When complete, start a break
pomodoro start --type break

# View your productivity
pomodoro stats
```

### Custom Configuration

```bash
# Use shorter durations
pomodoro config --set work_minutes=20
pomodoro config --set short_break_minutes=4

# Disable sounds but keep notifications
pomodoro config --set sound_enabled=false
pomodoro config --set notification_enabled=true

# Set a custom alert sound
pomodoro config --set custom_sound_path=/path/to/sound.mp3
```

## Development

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Check Code Quality

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

## Acknowledgments

- Inspired by the Pomodoro Technique® developed by Francesco Cirillo
- Built with Rust 🦀

## Spec Kit

```
curl -LsSf https://astral.sh/uv/install.sh | sh
source $HOME/.local/bin/env
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git
```
