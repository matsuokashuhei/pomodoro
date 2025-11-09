# Tasks: CLI Pomodoro Timer

**Input**: Design documents from `/specs/001-cli-pomodoro-timer/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-commands.md, quickstart.md

**Tests**: No explicit test requirements found in the specification. Test tasks are omitted per guidelines.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Single Rust project structure:
- `src/` - Source code at repository root
- `tests/` - Integration tests
- `migrations/` - Database schema

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic Rust project structure

- [ ] T001 Create project directory structure: src/{models,services,cli}/, tests/integration/, migrations/
- [ ] T002 Initialize Cargo.toml with dependencies: clap, rusqlite, tokio, indicatif, notify-rust, rodio, chrono, serde, serde_json, dirs, anyhow
- [ ] T003 [P] Configure Cargo.toml release profile for optimization (opt-level="z", lto=true, strip=true)
- [ ] T004 [P] Create .gitignore for Rust project (target/, Cargo.lock for libraries)
- [ ] T005 [P] Setup rustfmt.toml for code formatting configuration
- [ ] T006 [P] Setup clippy configuration in .cargo/config.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T007 Create database migration system in migrations/001_initial_schema.sql with sessions, timer_state, preferences tables
- [ ] T008 [P] Create TimerSession model in src/models/session.rs with SessionType and SessionStatus enums
- [ ] T009 [P] Create TimerPreset model in src/models/preset.rs with PresetType enum and standard/short/long factory methods
- [ ] T010 [P] Create TimerState model in src/models/session.rs with TimerStatus enum and state management methods
- [ ] T011 [P] Create UserStatistics model in src/models/statistics.rs with calculation logic from sessions
- [ ] T012 [P] Create UserPreferences model in src/config.rs with default implementation and JSON serialization
- [ ] T013 Create DatabaseService in src/services/database.rs with connection management and migration runner
- [ ] T014 Implement database CRUD operations in src/services/database.rs for sessions table
- [ ] T015 Implement database operations in src/services/database.rs for timer_state singleton management
- [ ] T016 Implement database operations in src/services/database.rs for preferences key-value storage
- [ ] T017 [P] Create NotificationService in src/services/notifier.rs using notify-rust crate
- [ ] T018 [P] Create AudioService in src/services/audio.rs using rodio crate with embedded default sound
- [ ] T019 Create CLI argument parser structure in src/main.rs using clap derive macros with Commands enum
- [ ] T020 Create module declarations in src/lib.rs to export models and services for testing
- [ ] T021 [P] Setup error handling with anyhow in src/main.rs and context propagation

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Start and Complete a Work Session (Priority: P1) 🎯 MVP

**Goal**: Enable users to start a focused work session and receive notification when complete

**Independent Test**: Run `pomodoro start`, verify timer countdown display, wait for completion (or simulate elapsed time), verify notification appears

### Implementation for User Story 1

- [ ] T022 [P] [US1] Implement DisplayService in src/cli/display.rs with progress bar formatting using indicatif
- [ ] T023 [P] [US1] Implement time formatting functions in src/cli/display.rs for MM:SS format
- [ ] T024 [US1] Implement TimerService::start_session() in src/services/timer.rs to create session and timer_state records
- [ ] T025 [US1] Implement TimerService::check_active_timer() in src/services/timer.rs to prevent concurrent timers
- [ ] T026 [US1] Implement start command handler in src/cli/commands.rs for `pomodoro start` with type and preset options
- [ ] T027 [US1] Wire up start command in src/main.rs to call command handler
- [ ] T028 [US1] Implement status command handler in src/cli/commands.rs for `pomodoro status` with progress display
- [ ] T029 [US1] Wire up status command in src/main.rs to call status handler
- [ ] T030 [US1] Implement TimerService::calculate_remaining_time() in src/services/timer.rs for state-based time calculation
- [ ] T031 [US1] Implement TimerService::complete_session() in src/services/timer.rs to update session and send notification
- [ ] T032 [US1] Add completion check in status command to detect expired timers and trigger notifications
- [ ] T033 [US1] Add sound alert integration in TimerService::complete_session() when sound_enabled=true

**Checkpoint**: At this point, User Story 1 should be fully functional - users can start a timer, check status, and receive notifications

---

## Phase 4: User Story 2 - Take a Break After Work Session (Priority: P2)

**Goal**: Enable automatic break prompts and long break logic after 4 work sessions

**Independent Test**: Complete a work session, verify break prompt appears, start break, verify correct duration (5 min or 15 min after 4 sessions)

### Implementation for User Story 2

- [ ] T034 [US2] Implement TimerService::count_completed_work_sessions() in src/services/timer.rs to query today's completed work sessions
- [ ] T035 [US2] Implement TimerService::determine_break_type() in src/services/timer.rs with long break logic (every 4 sessions)
- [ ] T036 [US2] Add break prompt logic to TimerService::complete_session() in src/services/timer.rs when work session completes
- [ ] T037 [US2] Extend start command handler in src/cli/commands.rs to support --type break option
- [ ] T038 [US2] Add break session notifications in src/services/notifier.rs with appropriate messages
- [ ] T039 [US2] Update status display in src/cli/display.rs to distinguish work vs break sessions with icons

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - full Pomodoro cycle with breaks

---

## Phase 5: User Story 3 - Pause and Resume Timer (Priority: P3)

**Goal**: Enable users to pause and resume timers to handle interruptions

**Independent Test**: Start timer, run `pomodoro pause`, verify status shows paused, run `pomodoro resume`, verify timer continues from remaining time

### Implementation for User Story 3

- [ ] T040 [P] [US3] Implement TimerService::pause_session() in src/services/timer.rs to update timer_state status
- [ ] T041 [P] [US3] Implement TimerService::resume_session() in src/services/timer.rs to update timer_state status
- [ ] T042 [US3] Implement pause command handler in src/cli/commands.rs for `pomodoro pause`
- [ ] T043 [US3] Implement resume command handler in src/cli/commands.rs for `pomodoro resume`
- [ ] T044 [US3] Wire up pause command in src/main.rs to call pause handler
- [ ] T045 [US3] Wire up resume command in src/main.rs to call resume handler
- [ ] T046 [US3] Update status display in src/cli/display.rs to show paused state with ⏸️ icon
- [ ] T047 [US3] Add error handling in pause/resume handlers for invalid timer states

**Checkpoint**: All pause/resume functionality should work independently

---

## Phase 6: User Story 4 - Cancel Current Session (Priority: P3)

**Goal**: Enable users to cancel timers that are no longer needed

**Independent Test**: Start timer, run `pomodoro cancel`, verify session ends and is marked as cancelled (not counted in stats)

### Implementation for User Story 4

- [ ] T048 [US4] Implement TimerService::cancel_session() in src/services/timer.rs to update session status and delete timer_state
- [ ] T049 [US4] Implement cancel command handler in src/cli/commands.rs for `pomodoro cancel` with confirmation prompt
- [ ] T050 [US4] Wire up cancel command in src/main.rs to call cancel handler
- [ ] T051 [US4] Add --force flag support in cancel command handler to skip confirmation
- [ ] T052 [US4] Ensure cancelled sessions are excluded from statistics calculations in src/models/statistics.rs

**Checkpoint**: Cancel functionality should work independently

---

## Phase 7: User Story 5 - View Session Statistics (Priority: P3)

**Goal**: Display productivity statistics and session history

**Independent Test**: Complete multiple sessions (work and break), run `pomodoro stats`, verify accurate counts and durations

### Implementation for User Story 5

- [ ] T053 [P] [US5] Implement DatabaseService::get_sessions_by_date() in src/services/database.rs to query sessions for a specific date
- [ ] T054 [P] [US5] Implement UserStatistics::calculate_from_sessions() logic in src/models/statistics.rs with all metrics
- [ ] T055 [P] [US5] Implement streak calculation in src/models/statistics.rs for consecutive completed work sessions
- [ ] T056 [US5] Implement stats command handler in src/cli/commands.rs for `pomodoro stats` with --date option
- [ ] T057 [US5] Wire up stats command in src/main.rs to call stats handler
- [ ] T058 [US5] Create statistics display formatting in src/cli/display.rs with recent sessions list
- [ ] T059 [US5] Add --json flag support in stats command handler for machine-readable output
- [ ] T060 [US5] Add --json flag support in status command handler for machine-readable output

**Checkpoint**: All statistics and reporting functionality should work independently

---

## Phase 8: Configuration Management (Priority: P2)

**Goal**: Enable users to configure presets and preferences

**Independent Test**: Run `pomodoro config --list`, modify settings with --set, verify changes persist across commands

### Implementation for Configuration

- [ ] T061 [P] Implement UserPreferences::load() in src/config.rs to read from ~/.local/share/pomodoro/config.json
- [ ] T062 [P] Implement UserPreferences::save() in src/config.rs to write JSON configuration
- [ ] T063 Implement config command handler in src/cli/commands.rs for `pomodoro config` with --list, --get, --set options
- [ ] T064 Wire up config command in src/main.rs to call config handler
- [ ] T065 Add configuration validation in src/config.rs for preset values and file paths
- [ ] T066 Integrate UserPreferences into start command to use default preset from config
- [ ] T067 Add custom sound file loading in src/services/audio.rs when custom_sound_path is configured

**Checkpoint**: Configuration management should work independently

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T068 [P] Add comprehensive error messages for all error cases in src/cli/commands.rs per contracts/cli-commands.md
- [ ] T069 [P] Add --help documentation for all commands using clap attributes
- [ ] T070 [P] Add --version flag in src/main.rs with version from Cargo.toml
- [ ] T071 [P] Add --verbose flag support with RUST_LOG environment variable configuration
- [ ] T072 [P] Add environment variable support for POMODORO_DB_PATH and POMODORO_CONFIG_PATH
- [ ] T073 [P] Create integration test in tests/integration/timer_flow.rs for complete work session flow
- [ ] T074 [P] Create integration test in tests/integration/persistence.rs for state survival across CLI invocations
- [ ] T075 [P] Create integration test in tests/integration/commands.rs for all CLI commands
- [ ] T076 [P] Add unit tests in src/models/session.rs for TimerState tick and pause/resume logic
- [ ] T077 [P] Add unit tests in src/models/preset.rs for preset factory methods
- [ ] T078 [P] Add unit tests in src/models/statistics.rs for statistics calculation
- [ ] T079 [P] Create README.md with installation instructions and usage examples
- [ ] T080 [P] Add cross-platform path handling using dirs crate in src/config.rs and src/services/database.rs
- [ ] T081 Add graceful error handling for missing audio/notification dependencies
- [ ] T082 Run quickstart.md validation: build project, run all commands, verify outputs
- [ ] T083 Run cargo fmt to format all code
- [ ] T084 Run cargo clippy -- -D warnings to ensure no linting issues
- [ ] T085 Create release build with cargo build --release and verify binary size <5MB

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **User Story 2 (Phase 4)**: Depends on Foundational phase completion, integrates with User Story 1
- **User Story 3 (Phase 5)**: Depends on Foundational phase completion and User Story 1
- **User Story 4 (Phase 6)**: Depends on Foundational phase completion and User Story 1
- **User Story 5 (Phase 7)**: Depends on Foundational phase completion and User Story 1 (needs completed sessions)
- **Configuration (Phase 8)**: Depends on Foundational phase completion
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 completion logic but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Extends US1 timer but independently testable
- **User Story 4 (P3)**: Can start after Foundational (Phase 2) - Extends US1 timer but independently testable
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Reads data created by US1 but independently testable

### Within Each User Story

**User Story 1:**
1. Display and formatting services (T022-T023) can be parallel
2. Timer service core methods (T024-T026) must be sequential
3. Command handlers (T027-T029) depend on timer service
4. Completion and notification logic (T030-T033) builds on timer service

**User Story 2:**
1. Break logic calculations (T034-T035) can be parallel
2. Integration into completion flow (T036) depends on calculations
3. Command updates and display (T037-T039) can follow in any order

**User Story 3:**
1. Pause and resume services (T040-T041) can be parallel
2. Command handlers (T042-T043) can be parallel
3. Wiring (T044-T045) can be parallel
4. Display and error handling (T046-T047) can be parallel

**User Story 4:**
1. All tasks must be sequential (service → handler → wiring → flags → validation)

**User Story 5:**
1. Database queries and statistics calculation (T053-T055) can be parallel
2. Command handler (T056-T057) depends on statistics logic
3. Display and JSON support (T058-T060) can be parallel

### Parallel Opportunities

**Phase 1 (Setup):** T003, T004, T005, T006 can all run in parallel

**Phase 2 (Foundational):** T008, T009, T010, T011, T012, T017, T018, T021 can run in parallel (different model/service files)

**Phase 3 (User Story 1):** T022 and T023 can run in parallel

**Phase 5 (User Story 3):** T040 and T041 can run in parallel; T042 and T043 can run in parallel; T044 and T045 can run in parallel; T046 and T047 can run in parallel

**Phase 7 (User Story 5):** T053, T054, T055 can run in parallel; T058, T059, T060 can run in parallel

**Phase 8 (Configuration):** T061 and T062 can run in parallel

**Phase 9 (Polish):** T068, T069, T070, T071, T072, T073, T074, T075, T076, T077, T078, T079, T080 can all run in parallel (different files)

---

## Parallel Example: User Story 1

```bash
# Launch display services together (different concerns):
Task: "Implement DisplayService in src/cli/display.rs with progress bar formatting"
Task: "Implement time formatting functions in src/cli/display.rs for MM:SS format"

# After timer service complete, these can be parallel:
Task: "Implement start command handler in src/cli/commands.rs"
Task: "Implement status command handler in src/cli/commands.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T021) - CRITICAL, blocks all stories
3. Complete Phase 3: User Story 1 (T022-T033)
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Run `cargo run -- start`
   - Check status with `cargo run -- status`
   - Verify notification appears when timer completes
   - Test with different presets
5. Deploy/demo if ready - you have a working Pomodoro timer!

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready (21 tasks)
2. Add User Story 1 → Test independently → Deploy/Demo (MVP! - 12 more tasks, total 33)
3. Add User Story 2 → Test independently → Deploy/Demo (Adds break cycle - 6 more tasks, total 39)
4. Add Configuration → Enable user customization (7 more tasks, total 46)
5. Add User Story 3 → Enable pause/resume (8 more tasks, total 54)
6. Add User Story 4 → Enable cancel (5 more tasks, total 59)
7. Add User Story 5 → Add statistics (8 more tasks, total 67)
8. Add Polish → Production ready (18 more tasks, total 85)

Each increment adds value without breaking previous functionality!

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (critical path)
2. Once Foundational is done, distribute:
   - Developer A: User Story 1 (core timer)
   - Developer B: Configuration (T061-T067)
   - Developer C: User Story 5 (statistics, minimal dependency on US1)
3. After User Story 1 is complete:
   - Developer D: User Story 2 (break logic)
   - Developer E: User Story 3 (pause/resume)
   - Developer F: User Story 4 (cancel)
4. All converge on Polish phase together

---

## Notes

- [P] tasks = different files, no dependencies, safe for parallel work
- [Story] label maps task to specific user story for traceability and independent testing
- Each user story should be independently completable and testable
- Commit after each task or logical group of tasks
- Stop at any checkpoint to validate story independently
- All file paths are relative to repository root
- Database file location: `~/.local/share/pomodoro/sessions.db` (Linux/macOS), `%APPDATA%\pomodoro\sessions.db` (Windows)
- Config file location: `~/.local/share/pomodoro/config.json`
- Run `cargo test` frequently to catch regressions early
- Run `cargo clippy` before committing to catch common mistakes
- See quickstart.md for detailed development workflow and debugging tips
