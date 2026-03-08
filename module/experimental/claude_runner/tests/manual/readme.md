# Manual Testing Plan: claude_runner CLI

## Prerequisites

- Claude Code binary in PATH: `which claude` returns a path
- API key configured: `ANTHROPIC_API_KEY` environment variable set
- Build binary: `cargo build -p claude_runner`
- Binary location: `target/debug/claude_runner` (or `/tmp/will_test_targets/*/debug/claude_runner` for workspace builds)

## Test Cases

### TC-1: Basic Execution
```sh
cargo run -p claude_runner -- --message "What is 2+2?"
```

**Expected:** Claude responds with "4" or equivalent. Exit code 0.

### TC-2: Continue Conversation
```sh
cargo run -p claude_runner -- --message "Remember number 42"
cargo run -p claude_runner -- --message "What number did I tell you?" --continue
```

**Expected:** Second invocation recalls "42". Exit code 0 on both.

### TC-3: Working Directory
```sh
cargo run -p claude_runner -- --message "List files in this directory" --dir /tmp
```

**Expected:** Claude lists files in `/tmp`. Exit code 0.

### TC-4: Skip Permissions
```sh
cargo run -p claude_runner -- --message "Run ls" --skip-permissions
```

**Expected:** Claude executes without permission prompts. Exit code 0.

### TC-5: Dry Run (No Claude Required)
```sh
cargo run -p claude_runner -- --message "test" --dir /tmp --continue --dry-run
```

**Expected:**
- Prints env var lines (CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000, etc.)
- Prints: `cd /tmp`
- Prints: `claude -c "test"`
- Does NOT invoke Claude binary
- Exit code 0

### TC-6: Help Output
```sh
cargo run -p claude_runner -- --help
```

**Expected:** Prints USAGE, OPTIONS table. Exit code 0.

### TC-7: Error on Unknown Flag
```sh
cargo run -p claude_runner -- --nonexistent-flag
```

**Expected:** Prints error to stderr. Exit code 1.

### TC-8: Max Tokens Override
```sh
cargo run -p claude_runner -- --message "hi" --max-tokens 50000 --dry-run
```

**Expected:** Dry-run output shows `CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000`.

### TC-9: Model Selection
```sh
cargo run -p claude_runner -- --message "hi" --model claude-haiku-4-5-20251001 --dry-run
```

**Expected:** Dry-run output shows `--model claude-haiku-4-5-20251001` in command.

### TC-10: Positional Message
```sh
cargo run -p claude_runner -- "What is Rust?" --dry-run
```

**Expected:** Dry-run shows `"What is Rust?"` in command. Exit code 0.

### TC-11: Verbose Mode
```sh
cargo run -p claude_runner --message "test" --verbose
```

**Expected:** Prints env vars and command to stderr before executing. Exit code 1 if Claude not available (nested session check).

### TC-12: Verbose with Dry Run
```sh
cargo run -p claude_runner --message "test" --verbose --dry-run
```

**Expected:** Dry-run wins - preview on stdout, stderr empty. Exit code 0.

### TC-13: Short Verbose Flag
```sh
cargo run -p claude_runner --message "test" -v
```

**Expected:** Same behavior as `--verbose`. Prints to stderr before executing.

### TC-14: Session Directory
```sh
cargo run -p claude_runner --message "test" --session-dir /tmp/sessions --dry-run
```

**Expected:** Dry-run output shows `CLAUDE_CODE_SESSION_DIR=/tmp/sessions`. Exit code 0.

### TC-15: Empty Message (dry-run)
```sh
cargo run -p claude_runner --dry-run
```

**Expected:** Dry-run output shows bare `claude` command with no message argument. Exit code 0.

### TC-16: Duplicate Message Flag
```sh
cargo run -p claude_runner --message "first" --message "second"
```

**Expected:** Error: "conflicts with a previously set message". Exit code 1.

### TC-17: Duplicate Dir Flag (last wins)
```sh
cargo run -p claude_runner --message "test" --dir /tmp --dir /other --dry-run
```

**Expected:** Dry-run shows `cd /other` (last value wins). Exit code 0.

### TC-18: Negative Max Tokens
```sh
cargo run -p claude_runner --message "test" --max-tokens -1
```

**Expected:** Error: "invalid --max-tokens value: -1". Exit code 1.

### TC-19: Max Tokens Overflow
```sh
cargo run -p claude_runner --message "test" --max-tokens 4294967296 --dry-run
```

**Expected:** Error: "invalid --max-tokens value: 4294967296". Exit code 1.

### TC-20: Missing Value for Flag
```sh
cargo run -p claude_runner --message "test" --dir
```

**Expected:** Error: "--dir requires a value". Exit code 1.

### TC-21: Positional + --message Conflict
```sh
cargo run -p claude_runner "first" --message "second"
```

**Expected:** Error: "conflicts with a previously set message". Exit code 1.

### TC-22: Zero Max Tokens
```sh
cargo run -p claude_runner --message "test" --max-tokens 0 --dry-run
```

**Expected:** Dry-run output shows `CLAUDE_CODE_MAX_OUTPUT_TOKENS=0`. Exit code 0.

### TC-23: Empty Values
```sh
cargo run -p claude_runner --message "test" --dir "" --session-dir "" --model "" --dry-run
```

**Expected:** Dry-run output shows empty dir, empty session dir, and `--model "test"`. Exit code 0.

### TC-24: Boolean Flag Idempotence
```sh
cargo run -p claude_runner --message "test" --verbose --verbose --verbose --dry-run
```

**Expected:** Duplicate boolean flags accepted (last-wins). Dry-run output shows verbose behavior. Exit code 0.

## Pass Criteria

All TC-1 through TC-24 must pass without unexpected errors or panics.
TC-5 through TC-10, TC-14, TC-15, TC-22, TC-23, TC-24 are runnable without a configured Claude API key.
TC-1, TC-2, TC-3, TC-4, TC-11, TC-12, TC-13 require Claude binary for full execution test.
