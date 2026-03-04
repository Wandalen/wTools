# Manual Testing Plan: claude_runner CLI

## Prerequisites

- Claude Code binary in PATH: `which claude` returns a path
- API key configured: `ANTHROPIC_API_KEY` environment variable set
- Build binary: `cargo build -p claude_runner`
- Binary location: `target/debug/claude_runner`

## Test Cases

### TC-1: Basic Execution

```sh
cargo run -p claude_runner -- --message "What is 2+2?"
```

**Expected:** Claude responds with "4" or equivalent. Exit code 0.

### TC-2: Continue Conversation

```sh
cargo run -p claude_runner -- --message "Remember the number 42"
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

## Pass Criteria

All TC-1 through TC-10 must pass without unexpected errors or panics.
TC-5 through TC-10 are runnable without a configured Claude API key.
