# claude_runner

Reusable `.claude` and `.plan.claude` unilang commands for willbe binaries.

## Purpose

Provides AI command routines that any willbe binary can register without duplicating
YAML definitions or routine logic. Wraps `dream_agent` with unilang command infrastructure.

## Files

| File | Responsibility |
|------|---------------|
| `Cargo.toml` | Crate manifest with optional feature-gated deps |
| `src/lib.rs` | Public API: `routines` module + `COMMANDS_YAML` constant |
| `src/routines.rs` | `.claude`, `.plan.claude`, `.claude.help` routine implementations |
| `claude.commands.yaml` | Unilang command definitions for all three commands |

## Usage

**Build-time aggregation** (dream-style):
```rust
// In build.rs:
let claude_runner_yaml = manifest_dir.parent().unwrap().join( "claude_runner" ).join( "claude.commands.yaml" );
let ai_commands = load_yaml_and_transform( &claude_runner_yaml );
```

**Runtime registration** (pr_review-style):
```rust
aggregator.add( claude_runner::COMMANDS_YAML );
```

**Registry mapping:**
```rust
".claude"      => claude_runner::routines::claude_routine,
".plan.claude" => claude_runner::routines::plan_claude_routine,
".claude.help" => claude_runner::routines::claude_help_routine,
```
