# spec

- **Version:** 0.1
- **Date:** 2026-03-01
- **Project Name:** claude_runner — Claude Code CLI Launcher
- **Type:** Rust Binary Crate

## Workspace Affiliation

- **Workspace:** wtools (`~/pro/lib/wip_core/wtools/dev`)
- **Location within workspace:** `module/experimental/claude_runner_cli`
- **Pattern:** CLI utility — thin wrapper over `claude_runner_core` library
- **Package name:** `claude_runner_cli` (Cargo package); **binary name:** `claude_runner_cli` (installed binary)
- **Availability:** Built as binary; not publishable as a library dependency

## Project Goal

Provide a command-line interface for executing Claude Code with all configuration
options exposed as CLI flags, delegating all process execution to `claude_runner_core`.

## Design Principles

### Thin CLI Wrapper

`claude_runner` is a **pure argument-parsing front-end**. It owns no execution logic.
All process spawning lives exclusively in `claude_runner_core::ClaudeCommand::execute()`.

**Separation of Concerns:**
- **claude_runner** (THIS crate): Argument parsing, user-facing output, exit codes
- **claude_runner_core**: Process execution, builder pattern, token limits

### Single Responsibility: Argument Parsing Only

`claude_runner` does exactly one thing: translate CLI flags into `ClaudeCommand` builder calls.

## In Scope

- Argument parsing via `std::env::args()`
- Help text output (`--help`)
- Dry-run mode (`--dry-run`): print command without invoking Claude
- Verbose mode (`--verbose`): print command to stderr, then execute
- Exit code propagation from Claude process
- Error messages to stderr

## Out of Scope

- Process execution → delegated to `claude_runner_core`
- Session storage → delegated to `claude_session`
- Configuration loading → delegated to `config_hierarchy`
- Interactive TUI → out of scope entirely

## Functional Requirements

### FR-1: Message Argument
`claude_runner` must accept `-m/--message <MSG>` or a positional argument as the Claude prompt.
Providing both (in any order) or duplicate `--message` flags must produce an error and exit 1.

### FR-2: Working Directory
`claude_runner` must accept `-d/--dir <PATH>` to set the Claude Code working directory.

### FR-3: Continue Conversation
`claude_runner` must accept `-c/--continue` to pass `with_continue_conversation(true)`.

### FR-4: Max Output Tokens
`claude_runner` must accept `--max-tokens <N>` (u32) to override the default 200K token limit.

### FR-5: Skip Permissions
`claude_runner` must accept `--skip-permissions` to add `--dangerously-skip-permissions`.

### FR-6: Dry Run
`claude_runner` must accept `--dry-run` to print env vars and command line without executing.

### FR-7: Session Directory
`claude_runner` must accept `--session-dir <PATH>` to override session storage location.

### FR-8: Model Selection
`claude_runner` must accept `--model <NAME>` to select the Claude model.

### FR-9: Help Output
`claude_runner` must accept `-h/--help` and print usage documentation to stdout with exit 0.

### FR-10: Error Handling
`claude_runner` must print errors to stderr and exit with code 1 on failure.

### FR-11: Exit Code Propagation
When Claude exits non-zero, `claude_runner` must exit with code 1 and print the exit code.

### FR-12: Verbose Mode
`claude_runner` must accept `-v/--verbose` to print the assembled environment variables
and command line to **stderr** before executing. Unlike `--dry-run`, execution proceeds
normally after printing. When both `--verbose` and `--dry-run` are specified, `--dry-run`
takes precedence and `--verbose` is a no-op.

## Non-Functional Requirements

### NFR-1: Minimal Dependencies
`claude_runner` must depend only on: `claude_runner_core`, `error_tools`, standard library.

### NFR-2: No Execution Logic
`claude_runner` must contain zero `Command::new("claude")` calls — all execution is in `claude_runner_core`.

### NFR-3: Feature Compliance
`claude_runner` must declare `default`, `enabled`, and `full` features per wtools workspace standard.

## CLI Usage

```sh
claude_runner [OPTIONS] [MESSAGE]

OPTIONS:
  -m, --message <MSG>        Prompt message for Claude
  -d, --dir <PATH>           Working directory (default: current dir)
  -c, --continue             Continue existing conversation
      --max-tokens <N>       Max output tokens (default: 200000)
      --skip-permissions     Skip tool permission prompts
      --dry-run              Print command without executing
  -v, --verbose              Print command to stderr, then execute
      --session-dir <PATH>   Session storage directory
      --model <NAME>         Claude model to use
  -h, --help                 Show this help
```

## Examples

```sh
# Basic execution
claude_runner --message "Explain this code"

# Continue conversation
claude_runner -m "What did I say before?" --continue

# Working directory + dry run
claude_runner --message "Fix the bug" --dir /path/to/project --dry-run

# Skip permissions for automation
claude_runner --message "Run tests" --skip-permissions
```

## Dependency: claude_runner_core

- **Purpose:** All Claude Code process execution
- **API Used:** `ClaudeCommand::new().with_*().execute()` builder chain
- **Critical Rule:** `Command::new("claude")` lives ONLY in `claude_runner_core`
