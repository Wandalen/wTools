### Issue

Properties don't work.

### Log

```
┌────────────────── user1@workstation-2025-07-19-13-39 (master) ──────────────────
└ ~/pro/lib/willbe/module/claude_storage ─> claude_storage .show
Project: Path("/home/user1/pro/lib/willbe/module/claude_storage/-default_topic")
Storage: "/home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-claude-storage--default-topic"

Sessions: 12 (Main: 2, Agent: 10)
Total Entries: 357

Sessions:
  - 79f86582-1435-442c-935a-13f8d874918a (347 entries, last: 2025-12-02T17:28:45.876Z)
  - 38809aee-8384-413c-9e41-813cef040eb7 (0 entries, last: unknown)

┌────────────────── user1@workstation-2025-07-19-13-39 (master) ──────────────────
└ ~/pro/lib/willbe/module/claude_storage ─> claude_storage .show.help
Command: .show
Description: Display session or project details (location-aware, adapts to parameters)
Hint: Show session or project
Version: 1.2.0

Arguments:
  session_id (String, optional)
    Session ID to display (omit to show project)
  project (String, optional)
    Project ID (default: current directory)
  verbosity (Integer, optional) [default: 1]
    Output detail level (0-5, default: 1)
    Aliases: v
  entries (Boolean, optional) [default: 0]
    Show all entries in session

Examples:
  .show
  .show session_id::abc123
  .show project::/home/user1/project
  .show session_id::abc123 project::/home/user1/project
  .show session_id::abc123 entries::1

Usage:
  .show  # Execute command
  .show.help  # Show this help
  .show ??  # Alternative help access

┌────────────────── user1@workstation-2025-07-19-13-39 (master) ──────────────────
└ ~/pro/lib/willbe/module/claude_storage ─> claude_storage .show entries::1
Project: Path("/home/user1/pro/lib/willbe/module/claude_storage/-default_topic")
Storage: "/home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-claude-storage--default-topic"

Sessions: 12 (Main: 2, Agent: 10)
Total Entries: 357

Sessions:
  - 79f86582-1435-442c-935a-13f8d874918a (347 entries, last: 2025-12-02T17:28:45.876Z)
  - 38809aee-8384-413c-9e41-813cef040eb7 (0 entries, last: unknown)

┌────────────────── user1@workstation-2025-07-19-13-39 (master) ──────────────────
└ ~/pro/lib/willbe/module/claude_storage ─>
```

---

## Fix Documentation

### Root Cause

The `entries` parameter was being **parsed** but **not validated** for compatibility with other parameters. When `.show entries::1` was called without `session_id`, the implementation followed this path:

1. Parsed `entries::1` correctly (line 334 of `src/cli/mod.rs`)
2. Fell into "Case 1" (no parameters → show project) in the routing logic
3. Called `show_project_for_cwd_impl(verbosity)` which **does not accept** the `show_entries` parameter
4. Parameter was **silently ignored** - no error, no effect

This is a **"garbage parameter"** bug - the parameter is syntactically valid but has no semantic effect in certain contexts.

### Why Not Caught

The command definition (`unilang.commands.yaml`) declares `entries` as a valid parameter for `.show`, but doesn't specify that it requires `session_id` to be meaningful. No runtime validation existed to enforce this constraint.

Integration/unit tests focused on the "happy path" where `entries` is used correctly with `session_id`, missing the invalid case.

### Fix Applied

**File:** `/home/user1/pro/lib/willbe/module/claude_storage/src/cli/mod.rs`
**Location:** Lines 336-357
**Approach:** Parameter validation

Added validation logic that rejects `entries::1` when `session_id` is not provided:

```rust
// Fix(issue-001): Validate entries parameter requires session_id
if show_entries && session_id.is_none()
{
  return Err
  (
    ErrorData::new
    (
      ErrorCode::InternalError,
      "Parameter 'entries' requires 'session_id'. \
       Use '.show session_id::<id> entries::1' to display session entries."
        .to_string()
    )
  );
}
```

### Prevention

**Design Rule:** When parameter P only works with parameter Q, add explicit validation:
```rust
if param_p.is_some() && param_q.is_none() {
  return Err(...);  // Fail fast with clear message
}
```

**Testing Rule:** Test invalid parameter combinations, not just valid ones:
- ✅ Test: `entries::1` WITH `session_id` (valid)
- ✅ Test: `entries::1` WITHOUT `session_id` (should error)

**Documentation Rule:** Command help should specify parameter dependencies:
```
entries (Boolean, optional)
  Show all entries in session
  ⚠️  Requires: session_id
```

### Pitfall

**Silent garbage parameters are worse than syntax errors.** Users will:
1. Waste time debugging why parameter has no effect
2. Lose confidence in the tool
3. Assume the feature is broken rather than misused

Always validate semantic constraints at the earliest possible point (parameter parsing) with clear error messages pointing to correct usage.

### Verification

**Test 1 - Invalid usage (should fail):**
```bash
$ cargo run -- .show entries::1
Error: Execution error: Execution Error: Parameter 'entries' requires 'session_id'. Use '.show session_id::<id> entries::1' to display session entries.
```
✅ Correctly rejects invalid combination with helpful message

**Test 2 - Valid usage (should work):**
```bash
$ cargo run -- .show session_id::79f86582-1435-442c-935a-13f8d874918a entries::1
Session: 79f86582-1435-442c-935a-13f8d874918a
...
Entries:
1. [User] ...
2. [Assistant] ...
...
440. [Assistant] 723eb2b2-a0f5-45f8-a1c5-9227ce731bd3 (2025-12-02T18:27:54.150Z)
```
✅ Correctly displays all entries when session_id is provided
