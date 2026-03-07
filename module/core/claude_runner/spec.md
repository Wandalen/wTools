# Spec: claude_runner

## Purpose

Provides reusable AI command routines (`.claude`, `.plan.claude`, `.claude.help`) as a
willbe library crate. Any binary that uses unilang can register these commands without
duplicating logic.

## Architecture

```
claude_runner
  └─ routines.rs
       ├─ claude_routine           → dream_agent::execute_claude → claude binary
       ├─ plan_claude_routine      → wplan_client::execute_cli → daemon queue
       ├─ claude_help_routine      → static help text
       ├─ expand_work_dir_pattern  → Vec<PathBuf> (FR-6: work_dir multi-value)
       ├─ resolve_file_reference   → String (FR-14: generic @file.ext)
       └─ resolve_params_at_prefix → batch @file resolution for forwarded params

claude_runner uses:
  dream_agent  (session management, ClaudeCommand execution)
  wplan_core   (DEFAULT_TOPIC, queue resolution, config dir for vars)
  wplan_client (execute_cli for .plan delegation)
  unilang      (VerifiedCommand, OutputData, ErrorData types)
  multiline_input (interactive message collection)
  libc         (TTY detection)
  glob         (work_dir glob pattern expansion)
  fs           (file reading for @file.ext resolution)
```

## Commands

### `.claude`

Primary AI assistance command. Delegates to `dream_agent::execute_claude` with full
parameter forwarding. Supports positional args, named parameters, and interactive mode.

### `.plan.claude`

Queues `.claude` invocations via wplan daemon. Constructs `<binary> .claude ...` command
strings and submits them via `.plan`. Supports multi-directory execution via `work_dir`.

### `.claude.help`

Displays static usage documentation for `.claude` parameters.

## Public API

```rust
pub mod routines;
pub const COMMANDS_YAML: &str;  // Absolute path to claude.commands.yaml

// Key public functions in routines:
pub fn expand_work_dir_pattern( pattern : &str ) -> Result< Vec< PathBuf >, String >;
pub fn resolve_file_reference( value : &str ) -> Result< String, String >;
```

### `expand_work_dir_pattern`

Expands `work_dir` parameter to list of directories. Supports `@varname` (variable lookup),
`@file.ext` (file list), glob patterns, and literal paths with auto-creation.
Returns `Vec<PathBuf>`. See dream spec FR-6.

### `resolve_file_reference`

Resolves `@file.ext` references for generic string parameters. If value starts with `@` and
contains `.` or `/`, reads the file and returns content as a single `String` (trailing
whitespace trimmed). Bare `@varname` (no extension, no `/`) passes through unchanged.
1MB size limit. See dream spec FR-14.

## Constraints

- wtools workspace (module/core/claude_runner)
- No `mod_interface` required at current module count
- All functionality behind `enabled` feature gate

## Sibling Crates

All in `module/core/` alongside this crate:

| Crate | Role |
|-------|------|
| `claude_runner_cli` | Standalone unilang CLI binary (`claude_runner` binary name) |
| `claude_runner_core` | `ClaudeCommand` process execution library (builder pattern) |
| `claude_session` | Session storage path resolution and continuation detection |
| `claude_storage` | CLI tool for exploring Claude Code storage |
| `claude_storage_core` | Zero-dep core library for Claude storage access |
