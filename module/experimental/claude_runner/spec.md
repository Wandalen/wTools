# Spec: claude_runner

## Purpose

Provides reusable AI command YAML definitions and a constants-only public API for willbe
binaries. Any binary that uses unilang can register `.claude` and `.claude.help` commands
by pointing at `COMMANDS_YAML`.

`claude_runner` is a **pure constants provider** — it contains no routine logic, no willbe
dependencies, and no dream_agent coupling.

## Architecture

```
claude_runner (wtools lib)
  └─ COMMANDS_YAML : &str         → absolute path to claude.commands.yaml
  └─ build_command_registry_from  → builds CommandRegistry from pre-generated static map
  └─ claude.commands.yaml         → command parameter definitions (source of truth)

Routines (claude_routine, claude_help_routine) live in dream_agent (willbe).
dream_agent provides the handlers; claude_runner provides the YAML schema.
```

## Separation of Concerns

| Concern | Owner |
|---------|-------|
| YAML command parameter definitions | `claude_runner` (THIS crate) |
| Compile-time static registry generation | `claude_runner` build.rs |
| Runtime command handlers (routines) | `dream_agent` (willbe) |
| Claude Code process execution | `claude_runner_core` via `claude_runner_cli` subprocess |
| Session management | `dream_agent` |
| Context injection | `dream_agent` |

## Commands Defined

### `.claude`

Primary AI assistance command. Parameters defined in `claude.commands.yaml`. Handlers
live in `dream_agent::routines`.

### `.claude.help`

Displays usage documentation for `.claude` parameters. Handler lives in
`dream_agent::routines`.

## Public API

```rust
pub const COMMANDS_YAML: &str;                      // Absolute path to claude.commands.yaml
pub fn build_command_registry_from(commands) -> CommandRegistry;  // Build registry (handlers injected externally)
pub use wplan::execute_with_registry;               // Execute dispatch helper
```

### `COMMANDS_YAML`

Absolute path to the YAML command definitions file, computed at compile time via
`env!("CARGO_MANIFEST_DIR")`. Used by consumers in two ways:

**Build-time aggregation** (PHF static registry):
```rust
// In build.rs:
let ai_yaml = manifest_dir.parent().unwrap().join("claude_runner").join("claude.commands.yaml");
let commands = load_yaml_and_transform(&ai_yaml);
```

**Runtime aggregation:**
```rust
aggregator.add(claude_runner::COMMANDS_YAML);
```

### `build_command_registry_from`

Takes the compile-time generated `AGGREGATED_COMMANDS` static map and builds a
`CommandRegistry`. Handlers are injected by the caller (dream_agent provides them).
`claude_runner` itself has no knowledge of handler implementations.

## Constraints

- wtools workspace member (`module/core/claude_runner`)
- **Zero willbe dependencies** — MUST NOT import dream_agent, wplan, wplan_core, or any willbe crate
- All functionality behind `enabled` feature gate
- Routines module MUST NOT exist in this crate (routines belong to dream_agent)
- No `mod_interface` required at current module count

## Sibling Crates

All in `module/core/` alongside this crate:

| Crate | Role |
|-------|------|
| `claude_runner_cli` | Standalone unilang CLI binary (`claude_runner` binary name) |
| `claude_runner_core` | `ClaudeCommand` process execution library (builder pattern) |
| `claude_session` | Session storage path resolution and continuation detection |
| `claude_storage` | CLI tool for exploring Claude Code storage |
| `claude_storage_core` | Zero-dep core library for Claude storage access |

## Consumers

- `dream_agent` (willbe): imports `COMMANDS_YAML` for command schema; provides runtime handlers
- dream binary (willbe): uses `build_command_registry_from` after dream_agent injects handlers
- other willbe binaries: may use `COMMANDS_YAML` for runtime aggregation
