# Spec: claude_runner

## Purpose

Provides two things in one crate:

1. **Library** — YAML command schema constants (`COMMANDS_YAML`) for unilang command registration.
2. **Binary** (`claude_runner`) — CLI that accepts `--flag value` argv and executes Claude Code
   via the `claude_runner_core` builder pattern.

`claude_runner` has zero willbe dependencies in its library surface. The binary depends on
`claude_runner_core` and `unilang` (gated behind the `enabled` feature).

## Architecture

```
claude_runner lib
  └─ COMMANDS_YAML : &str     → absolute path to claude.commands.yaml

claude_runner binary (CLI)
  └─ argv_to_unilang_tokens()     → --flag value → [".run", "key::value", ...]
  └─ build_registry()             → CommandRegistry with .run handler
  └─ ClaudeCommand builder        → via claude_runner_core
  └─ subprocess: claude           → the actual AI binary
```

**Two binaries use this crate differently:**

```
dream_agent (willbe) spawns subprocess:
  claude_runner --message X --dir Y [--continue] [--max-tokens N] ...
    → THIS binary (argv → unilang → ClaudeCommand → claude subprocess)

claude_runner_plugin (dream_agent binary) uses the lib constant:
  claude_runner::COMMANDS_YAML → compile-time YAML aggregation for wplan runner
```

## Separation of Concerns

| Concern | Owner |
|---------|-------|
| YAML command parameter definitions | `claude_runner` lib (THIS crate) |
| CLI flag-to-builder translation | `claude_runner` binary (THIS crate) |
| Claude Code process execution | `claude_runner_core` (builder pattern) |
| Wplan runner plugin (`.claude` command) | `claude_runner_plugin` binary in `dream_agent` |
| Runtime command handlers (routines) | `dream_agent::routines` (willbe) |
| Session management and context injection | `dream_agent` (willbe) |
| Session storage paths | `claude_session` |

## Commands (Binary)

### `.run` (default)

Execute Claude Code with the given parameters. Invoked implicitly when the first argv token
is not a flag.

```
claude_runner "Fix the bug" --dir /path/to/project
claude_runner --message "Explain this" --continue
claude_runner --message "test" --dry-run
```

### `.help`

Print usage. Triggered by `-h` / `--help`.

## CLI Flags

| Flag | Short | Type | Description |
|------|-------|------|-------------|
| `--message` | `-m` | string | Prompt text for Claude |
| `--dir` | `-d` | string | Working directory (default: `$CWD`) |
| `--continue` | `-c` | bool | Continue existing conversation |
| `--max-tokens` | | u32 | Max output tokens (default: 200000) |
| `--skip-permissions` | | bool | Skip tool permission prompts |
| `--dry-run` | | bool | Print command without executing |
| `--verbose` | `-v` | bool | Print command to stderr, then execute |
| `--session-dir` | | string | Session storage directory |
| `--model` | | string | Claude model override |
| `--help` | `-h` | bool | Show help |

## Public API (Library)

```rust
pub const COMMANDS_YAML : &str;   // Absolute path to claude.commands.yaml
```

### `COMMANDS_YAML`

Absolute path to the YAML command definitions file, computed at compile time via
`env!("CARGO_MANIFEST_DIR")`. Used by consumers for unilang command registration.

**Build-time aggregation (PHF static registry):**
```rust
// In build.rs of a runner plugin:
let yaml = claude_runner::COMMANDS_YAML;
aggregator.add( yaml );
```

**Runtime aggregation:**
```rust
aggregator.add( claude_runner::COMMANDS_YAML );
```

## Constraints

- wtools workspace member (`module/experimental/claude_runner`)
- **Zero willbe dependencies in lib** — `lib.rs` MUST NOT import dream_agent, wplan, wplan_core
- Binary deps (`claude_runner_core`, `unilang`, `error_tools`) MUST be optional, gated by `enabled`
- `routines.rs` MUST NOT exist in this crate — routines belong to `dream_agent`
- No `build.rs` — this crate no longer generates a static registry at build time

## Sibling Crates

All in `module/experimental/` alongside this crate:

| Crate | Role |
|-------|------|
| `claude_runner_core` | `ClaudeCommand` process execution library (builder pattern) |
| `claude_session` | Session storage path resolution and continuation detection |
| `claude_storage` | CLI tool for exploring Claude Code storage |
| `claude_storage_core` | Zero-dep core library for Claude storage access |

## Consumers

- `dream_agent` (willbe): spawns `claude_runner` as subprocess for Claude Code execution
- `claude_runner_plugin` (dream_agent binary): uses `COMMANDS_YAML` for wplan `.claude` command
