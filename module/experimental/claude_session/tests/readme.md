# tests/

This directory contains all functional and integration tests for the `claude_session` crate, which provides session-based programmatic access to Claude Code CLI with zero dependencies, session management, and command generation.

## Responsibility Table

| Test Suite | Responsibility | In Scope | Out of Scope (See) |
|------------|----------------|----------|-------------------|
| `detection_tests.rs` | Session detection | Claude Code session detection via ~/.claude/projects/{escaped_path}/*.jsonl files, path escaping (special chars ’dashes: /, _, ., @, #, %, &, space), non-conversation file exclusion (agent-*.jsonl files ignored, empty files ignored), conversation file validation (non-empty *.jsonl files count as conversations), session existence based on conversation history | Session lifecycle (’ session tests), Command generation (’ command tests) |
| `session_tests.rs` | Session lifecycle | SessionManager creation (new with sessions_root), session directory format ({sessions_root}/-{session_name}), session existence detection (.claude_history file presence), ensure_session idempotence (multiple calls safe), Strategy::Resume (preserves existing files in session directory), Strategy::Fresh (deletes all files in session directory), directory creation on demand, sessions_base_dir accessor | Session detection (’ detection tests), Command generation (’ command tests) |
| `command_tests.rs` | Command generation | ClaudeCommand generation (generate with working_dir, message, timeout_ms, Strategy), automatic -c flag (added when Claude conversation history exists in ~/.claude/projects), no -c flag (for new sessions without history), --dangerously-skip-permissions flag (always present), Strategy independence (Fresh vs Resume don't affect command generation, only directory lifecycle), show() format (cd + claude command, shell-compatible), timeout configuration (1000ms, 7_200_000ms, 14_400_000ms), special character handling (quotes, newlines), empty message (interactive mode, no message arg), long message support | Session detection (’ detection tests), Session lifecycle (’ session tests) |

## Organization (3 test files)

Tests organized by functional domain (see Responsibility Table above).

### Scope

This test suite covers the claude_session crate's programmatic Claude Code CLI access (3 test files):

**In Scope:**
- Session detection via Claude Code's internal storage:
  - Path: ~/.claude/projects/{escaped_path}/*.jsonl
  - Path escaping rule: Special chars (/, _, ., @, #, %, &, space) ’ dashes
  - Conversation file validation (non-empty *.jsonl files, NOT agent-*.jsonl, NOT empty files)
  - Session existence: true if any valid conversation files exist
- SessionManager lifecycle:
  - Session directory format: {sessions_root}/-{session_name} (hyphen prefix)
  - session_exists() checks for .claude_history file presence
  - ensure_session() with Strategy enum:
    - Strategy::Resume (preserves existing session directory files)
    - Strategy::Fresh (deletes all files in session directory via remove_dir_all ’ create_dir_all)
  - Idempotence (multiple ensure_session calls safe)
  - sessions_base_dir() accessor
- ClaudeCommand generation and formatting:
  - Core fields: working_dir, timeout_ms, args array
  - Automatic -c flag logic:
    - Adds -c when Claude conversation history exists (check_session_exists returns true)
    - Omits -c for new sessions without history
  - Always includes --dangerously-skip-permissions flag
  - Strategy independence (Fresh vs Resume only affect directory lifecycle, not command args)
  - show() format: "cd {working_dir}\nclaude {args}"
  - Timeout configuration (configurable, default 7_200_000ms = 2 hours)
  - Message handling:
    - Long messages preserved in full
    - Special characters (quotes, newlines) preserved
    - Empty message (interactive mode, args contains only --dangerously-skip-permissions)
- Testing infrastructure (tempfile::TempDir, Claude storage cleanup)

**Out of Scope:**
- Context injection from wplan (’ wplan_agent crate)
- Interactive terminal UI (’ terminal-based tools)
- Configuration hierarchy (’ config_hierarchy crate)
- Daemon integration (’ wplan_daemon crate)
- Actual Claude Code binary execution (tests generate commands, don't execute)
- Process management and output capture (core API only, execution delegated to callers)
- Session persistence beyond filesystem (no database, no network)

**Test Quality**: Uses real filesystem operations with tempfile::TempDir for isolation. Tests create and clean up Claude storage directories (~/.claude/projects/) during tests to simulate real Claude Code behavior. Session detection tests validate path escaping matches Claude Code's internal convention. Command tests verify -c flag logic matches Claude Code's continuation expectations. All tests independent with no shared state.

## Navigation

Test files follow domain-based naming that reflects the functionality being tested.
