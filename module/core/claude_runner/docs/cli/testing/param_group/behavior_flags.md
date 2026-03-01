# Parameter Group: Behavior Flags

- **Parameters:** `continue::`, `skip_permissions::`, `dry::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | `--continue` + `--dry-run` → dry output shows `--continue` in claude command | Corner Cases | P0 |
| CC-2 | `--skip-permissions` + `--dry-run` → dry output shows `--dangerously-skip-permissions` | Corner Cases | P0 |
| CC-3 | All three flags simultaneously (`--continue --skip-permissions --dry-run`) | Corner Cases | P1 |
| CC-4 | No behavior flags → minimal invocation (no extra flags in claude argv) | Corner Cases | P1 |
| CD-1 | `--dry-run` short-circuits execution regardless of other flags (including `--continue`, `--skip-permissions`) | Co-Dependencies | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Corner Cases | 4 | 100% |
| Co-Dependencies | 1 | 100% |
| **Total** | **5** | **100%** |

**Cross-References:**
- Per-parameter edge cases → `../param/continue.md`, `../param/skip_permissions.md`, `../param/dry.md`
- Integration tests → `../command/run.md` IT-7, IT-9, IT-10

---

## Corner Cases

### CC-1: `--continue` + `--dry-run` → `--continue` visible in dry output

**Goal:** Verify that the `--continue` flag appears in the dry-run command representation, confirming it is assembled correctly before execution would happen.
**Command:** `claude_runner "Next step" --continue --dry-run`
**Expected Output:**
```
claude --continue "Next step"
```
**Verification:**
- Exit code is 0
- `--continue` present in stdout
- No claude process started
**Pass Criteria:** Exit 0; continue flag visible in dry-run; no execution
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-2: `--skip-permissions` + `--dry-run` → `--dangerously-skip-permissions` visible

**Goal:** Verify the flag translation (`--skip-permissions` → `--dangerously-skip-permissions`) is visible in the dry-run output.
**Command:** `claude_runner "task" --skip-permissions --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- `--dangerously-skip-permissions` (the translated form) present in stdout
- No claude process started
**Pass Criteria:** Exit 0; translated flag visible; no execution
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-3: All three flags simultaneously

**Goal:** Verify that `--continue`, `--skip-permissions`, and `--dry-run` can all be specified together; `--dry-run` suppresses execution while the other two appear in the command representation.
**Command:** `claude_runner "task" --continue --skip-permissions --dry-run`
**Expected Output:**
```
claude --continue --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- Both `--continue` and `--dangerously-skip-permissions` present in stdout
- No claude process started (dry-run short-circuits)
**Pass Criteria:** Exit 0; all three flags processed; only two appear in command (dry-run suppresses itself)
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-4: No behavior flags → minimal invocation

**Goal:** Verify that omitting all three behavior flags produces a clean minimal command with no extra flags.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- `--continue` absent from stdout
- `--dangerously-skip-permissions` absent from stdout
- Only `--max-tokens` (the default) present
**Pass Criteria:** Exit 0; no behavior-flag pollution in clean invocation
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

## Co-Dependencies

### CD-1: `--dry-run` short-circuits execution regardless of other flags

**Goal:** Verify the dependency rule: `--dry-run` prevents actual claude execution regardless of which other flags are present, including flags that would otherwise cause execution (`--continue`, `--skip-permissions`).
**Command:** `claude_runner "dangerous task" --continue --skip-permissions --model claude-opus-4-6 --max-tokens 50000 --dry-run`
**Expected Output:**
```
claude --continue --dangerously-skip-permissions --model claude-opus-4-6 --max-tokens 50000 "dangerous task"
```
**Verification:**
- Exit code is 0
- All assembled flags visible in stdout
- No claude process spawned (critical — `--skip-permissions` would otherwise bypass confirmations)
**Pass Criteria:** Exit 0; `--dry-run` wins regardless of all other flags; no execution occurs
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)
