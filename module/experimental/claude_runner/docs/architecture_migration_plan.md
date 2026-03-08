# Implementation Plan: Subprocess Architecture Migration

## Summary

Remove the backward cross-repo dependency `claude_runner (wtools) → dream_agent (willbe)`
by moving routines to dream_agent and replacing the internal Rust dep with subprocess
invocation of the `claude_runner` binary.

**Branch:** `cleaning_5` (or new branch from it)
**Scope:** `wtools/module/core/claude_runner/` + `willbe/module/dream_agent/`
**TDD:** Red tests exist at `dream_agent/tests/responsibility_subprocess_invocation_test.rs`

---

## Problem Statement

### Current (wrong) architecture

```
claude_runner lib (wtools)        dream_agent (willbe)
   ├─ routines.rs                    ├─ execute_claude()
   │   └─ calls dream_agent::        │   └─ ClaudeCommand::new() [claude_runner_core]
   │      execute_claude()           │       └─ Command::new("claude")
   └─ COMMANDS_YAML                  └─ Cargo.toml: claude_runner_core = { workspace }
```

Two violations:
1. `claude_runner` (wtools) → `dream_agent` (willbe): backward cross-repo dep
2. `dream_agent` (willbe) → `claude_runner_core` (wtools): execution Rust dep (should be subprocess)

### Target architecture

```
claude_runner lib (wtools)        dream_agent (willbe)
   └─ COMMANDS_YAML only            ├─ routines.rs (MOVED from claude_runner)
                                    │   └─ spawns subprocess: claude_runner binary
                                    ├─ execute_claude()
                                    │   └─ std::process::Command::new("claude_runner")
                                    └─ Cargo.toml: NO claude_runner_core dep
```

---

## Validation / TDD Gate

These red tests must be GREEN before the migration is considered complete:

```bash
cd willbe/dev && cargo nextest run -p dream_agent --test responsibility_subprocess_invocation_test
```

All 6 tests must pass:
- `no_claude_runner_core_dependency` — Cargo.toml clean
- `has_subprocess_invocation` — subprocess call present
- `no_claude_command_builder_in_src` — no ClaudeCommand::new() in src
- `subprocess_passes_max_tokens` — --max-tokens flag present
- `routines_module_exists_in_dream_agent` — routines.rs exists
- `documentation_shows_subprocess_architecture` — readme.md updated

---

## Phase A: Move routines from claude_runner to dream_agent

### A1. Copy routines.rs

Copy `wtools/module/core/claude_runner/src/routines.rs` to
`willbe/module/dream_agent/src/routines.rs`.

**Key changes in the copy:**
1. Remove the `use unilang::...` that includes `phf` (keep only what's needed)
2. Replace all calls to `dream_agent::execute_claude(args)` with subprocess invocation
   (the function now lives IN dream_agent, not as an external call)
3. Remove `wplan_core::resolve_file_reference` call — replace with local impl or move
   the logic inline (see A2)

The `delegate_to_agent()` helper becomes `delegate_to_subprocess()` which calls the
subprocess instead of `dream_agent::execute_claude()`.

### A2. Handle `resolve_file_reference` and `expand_work_dir_pattern`

These utilities are currently in `claude_runner/src/routines.rs` as pub fns exported
from the lib. After moving routines to dream_agent:

Option A2a (preferred): Keep them in `claude_runner/src/routines.rs` and re-export from
`claude_runner` lib. dream_agent calls them via subprocess arg (pass resolved content
as `--message` flag value). This is cleaner: file resolution happens in dream_agent
before spawning the subprocess.

Option A2b: Move them to `dream_agent/src/routines.rs`. This adds more code to dream_agent
but keeps all routing logic together.

**Recommended:** A2a — file resolution is an orchestration concern that belongs in dream_agent.
The resolved content is then passed as the `--message` arg to the subprocess.

### A3. Expose routines module from dream_agent lib.rs

In `dream_agent/src/lib.rs`:
```rust
#[ cfg( feature = "enabled" ) ]
pub mod routines;
```

### A4. Verify dream binary still works

The dream binary registers commands from claude_runner::COMMANDS_YAML (for schema) and
maps handlers from dream_agent::routines. Verify the dream binary's handler registration
code (likely in dream/src/main.rs or similar) continues to work.

If dream binary currently does:
```rust
use claude_runner::routines;
```
Change to:
```rust
use dream_agent::routines;
```

---

## Phase B: Replace ClaudeCommand::new() with subprocess invocation in dream_agent

### B1. Update execute_claude() in dream_agent/src/cli.rs

Replace the `ClaudeCommand::new().with_*().execute()` chain with subprocess construction:

```rust
fn spawn_claude_runner(params: &ClaudeParams, message: &str, session_dir: Option<&Path>) -> Result<String, String>
{
  let mut cmd = std::process::Command::new("claude_runner");

  // Required flags
  cmd.arg("--message").arg(message);
  cmd.arg("--dir").arg(&params.working_directory);
  cmd.arg("--max-tokens").arg("200000");

  // Optional flags
  if params.continuation
  {
    cmd.arg("--continue");
  }
  if let Some(dir) = session_dir
  {
    cmd.arg("--session-dir").arg(dir);
  }
  if params.verbosity >= 2
  {
    cmd.arg("--verbose");
  }
  if params.dry_run
  {
    cmd.arg("--dry-run");
  }
  if let Some(model) = &params.model
  {
    cmd.arg("--model").arg(model);
  }
  if params.skip_permissions
  {
    cmd.arg("--skip-permissions");
  }

  // For interactive mode, inherit stdin/stdout/stderr
  // For non-interactive, capture stdout and return it

  if params.interactive
  {
    let status = cmd.status().map_err(|e| format!("claude_runner not in PATH: {e}"))?;
    if status.success() { Ok(String::new()) } else { Err(format!("claude_runner exited {}", status.code().unwrap_or(-1))) }
  }
  else
  {
    let output = cmd.output().map_err(|e| format!("claude_runner not in PATH: {e}"))?;
    if output.status.success()
    {
      Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    else
    {
      Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
  }
}
```

### B2. Update verbosity_to_log_level return type

Currently: `fn verbosity_to_log_level(verbosity: u8) -> claude_runner::LogLevel`

After: `fn verbosity_to_log_level(verbosity: u8) -> u8` (returns verbose count for subprocess)

Or simply remove it and inline the verbosity → `--verbose` flag logic in `spawn_claude_runner`.
The function is exported from dream_agent (`pub use cli::verbosity_to_log_level`) so if other
callers depend on it, keep it but change the return type to `u8`.

### B3. Update dream_agent/Cargo.toml

Remove:
```toml
claude_runner_core = { workspace = true, optional = true }
```

Remove from `enabled` feature list:
```toml
"dep:claude_runner_core",
```

**Before removing**, verify no other code in dream_agent src/ uses `claude_runner_core::*`.
The only usage should be in `cli.rs` (ClaudeCommand builder) which is being replaced.

---

## Phase C: Clean up claude_runner lib

### C1. Remove dream_agent and wplan from claude_runner/Cargo.toml

Current deps that reference willbe:
```toml
dream_agent = { path = "...", version = "~0.1.39", optional = true }
wplan_core  = { path = "...", version = "~0.1.0", optional = true }
wplan       = { path = "...", version = "~0.1.0", optional = true }
```

Remove all three from `[dependencies]` and from the `enabled` feature list.

### C2. Remove routines module from claude_runner lib

In `claude_runner/src/lib.rs`:
- Remove `pub mod routines;` declaration
- Remove `pub use wplan::execute_with_registry;` (wplan dep is gone)
- Remove `build_command_registry_from` function — it maps routines by name. Routines are
  no longer in claude_runner. This function should move to dream_agent or be removed.

Decision point: `build_command_registry_from` maps command names to handler functions.
After migration, the dream binary should get handlers from dream_agent and map them.
Options:
- Option C2a: Move `build_command_registry_from` to dream_agent
- Option C2b: Remove it; dream binary creates the registry inline

**Recommended:** C2a — keep registry building encapsulated in dream_agent.

### C3. Delete claude_runner/src/routines.rs

The file is moved to dream_agent. Delete it from wtools.

### C4. Verify claude_runner lib compiles

```bash
cd wtools/dev && cargo check -p claude_runner
```

Target: compiles with zero warnings, zero willbe path deps.

---

## Phase D: Update dream binary (willbe/module/dream)

### D1. Update handler registration

Find where dream binary registers `.claude` handlers. Currently likely:
```rust
use claude_runner::routines::{ claude_routine, claude_help_routine };
```
Change to:
```rust
use dream_agent::routines::{ claude_routine, claude_help_routine };
```

### D2. Remove claude_runner lib dep if no longer needed

If dream binary only used claude_runner for routines (and still uses COMMANDS_YAML):
- Keep `claude_runner` as dep for COMMANDS_YAML path (for yaml schema)
- Remove if dream binary aggregates yaml directly via path

---

## Phase E: Update willbe workspace Cargo.toml

### E1. Verify workspace dep entries

In `willbe/dev/Cargo.toml`, check `[workspace.dependencies]`:
- `claude_runner_core` dep entry: can remain (other crates may use it)
- `claude_runner` dep entry: keep (dream_agent may still import for COMMANDS_YAML if needed)

### E2. Dream_agent Cargo.toml post-change deps

After migration, dream_agent's `[dependencies]` should only contain:
```toml
claude_session    = { workspace = true, optional = true }
wplan_core        = { workspace = true, features = ["enabled"], optional = true }
unilang           = { workspace = true, optional = true }
config_hierarchy  = { workspace = true, features = [ "file_ops" ], optional = true }
dirs              = { workspace = true, optional = true }
serde_json        = { workspace = true, optional = true }
```

Note: `claude_runner_core` is REMOVED. `claude_session` remains (storage paths only).

---

## Phase F: Migrate existing tests

### F1. Parameter forwarding regression test

`dream_agent/tests/parameter_forwarding_regression.rs` currently tests that dream_agent
correctly converts parameters to `ClaudeCommand::new().with_*()` builder calls.

After migration, update tests to verify subprocess argument construction:
- `--dir`, `--message`, `--continue`, `--max-tokens 200000`, `--verbose` flags

### F2. End-to-end integration test

`dream_agent/tests/end_to_end_integration.rs` likely exercises execute_claude() end-to-end.
The dry run mode tests may need updating if they check for `ClaudeCommand` output format.

### F3. Responsibility builder pattern test

`dream_agent/tests/responsibility_builder_pattern_usage_test.rs` must be **deleted** once
subprocess tests are green. Verify the file is removed, not just cleared.

### F4. Test compatibility

The strategy, session lifecycle, context injection, and continuation tests should work
unchanged since they test behavior at the execute_claude() boundary, not the implementation.
Run them after migration to confirm:

```bash
cd willbe/dev && cargo nextest run -p dream_agent
```

---

## Phase G: Verification

### G1. Per-crate checks

```bash
# wtools
cd /path/to/wtools/dev
cargo check -p claude_runner
cargo nextest run -p claude_runner  # should still pass
cargo clippy -p claude_runner --all-targets --all-features -- -D warnings

# willbe
cd /path/to/willbe/dev
cargo check -p dream_agent
cargo nextest run -p dream_agent  # all tests including subprocess test
cargo clippy -p dream_agent --all-targets --all-features -- -D warnings
```

### G2. Subprocess test gate

```bash
cd willbe/dev && cargo nextest run -p dream_agent --test responsibility_subprocess_invocation_test
```
All 6 tests must pass GREEN.

### G3. Full test suite

```bash
# wtools
cd wtools/dev && w3 .test l::3

# willbe
cd willbe/dev && w3 .test l::3
```

### G4. Architecture validation

After migration, verify:
```bash
# No willbe path deps in claude_runner Cargo.toml
grep "willbe" wtools/dev/module/core/claude_runner/Cargo.toml  # → no output

# No claude_runner_core dep in dream_agent Cargo.toml
grep "claude_runner_core" willbe/dev/module/dream_agent/Cargo.toml  # → no output

# Subprocess invocation exists
grep "claude_runner" willbe/dev/module/dream_agent/src/cli.rs  # → shows subprocess call
```

---

## Estimated Changes

| File | Change |
|------|--------|
| `willbe/module/dream_agent/src/routines.rs` | CREATE — moved from claude_runner |
| `willbe/module/dream_agent/src/cli.rs` | MODIFY — replace ClaudeCommand builder with subprocess |
| `willbe/module/dream_agent/src/lib.rs` | MODIFY — add `pub mod routines` |
| `willbe/module/dream_agent/src/log.rs` | MODIFY — change verbosity_to_log_level return type |
| `willbe/module/dream_agent/Cargo.toml` | MODIFY — remove claude_runner_core dep |
| `wtools/module/core/claude_runner/src/routines.rs` | DELETE |
| `wtools/module/core/claude_runner/src/lib.rs` | MODIFY — remove routines mod, remove wplan re-export |
| `wtools/module/core/claude_runner/Cargo.toml` | MODIFY — remove dream_agent, wplan, wplan_core |
| `willbe/module/dream/src/main.rs` (or similar) | MODIFY — update handler import to dream_agent::routines |
| `dream_agent/tests/responsibility_builder_pattern_usage_test.rs` | DELETE (after G2 passes) |
| `dream_agent/tests/parameter_forwarding_regression.rs` | MODIFY — update for subprocess |

---

## Rollback Plan

If the migration breaks the dream binary:
1. Check `which claude_runner` — binary must be in PATH
2. Install it: `cargo install --path wtools/module/core/claude_runner_cli`
3. If subprocess invocation fails with "not found", add PATH resolution in execute_claude()
4. Git revert individual files if needed (each phase is a separate commit)

---

## Pitfalls

| # | Pitfall | Mitigation |
|---|---------|------------|
| P1 | `claude_runner` binary not in PATH at runtime | dream_agent should give clear error: "claude_runner not found in PATH. Install: cargo install claude_runner" |
| P2 | `verbosity_to_log_level` return type change breaks callers | Check all callers of this function; update call sites |
| P3 | Interactive mode: subprocess must inherit TTY | Use `cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit())` for interactive mode |
| P4 | Session dir path escaping | claude_runner_cli must handle session-dir path correctly; verify it passes to `--session-dir` unchanged |
| P5 | Context injection: newlines in `--message` | Test that multi-line messages (context + user message) survive subprocess arg passing |
| P6 | wplan dep removal breaks `execute_with_registry` re-export | Find all callers; move re-export or inline usage in dream binary |
| P7 | `expand_work_dir_pattern` and `resolve_file_reference` move | These must move to dream_agent; claude_runner/spec.md and claude_runner/src/lib.rs no longer export them |
