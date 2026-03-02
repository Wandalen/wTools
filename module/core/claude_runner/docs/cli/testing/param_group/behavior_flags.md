# Parameter Group: Behavior Flags

- **Parameters:** `continue::`, `skip_permissions::`, `dry::`, `verbose::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | `--continue` + `--dry-run` → dry output shows `-c` in claude command | Corner Cases | P0 |
| CC-2 | `--skip-permissions` + `--dry-run` → dry output shows `--dangerously-skip-permissions` | Corner Cases | P0 |
| CC-3 | All three flags simultaneously (`--continue --skip-permissions --dry-run`) | Corner Cases | P1 |
| CC-4 | No behavior flags → env var block + bare claude invocation | Corner Cases | P1 |
| CD-1 | `--dry-run` short-circuits execution regardless of other flags (including `--continue`, `--skip-permissions`) | Co-Dependencies | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Corner Cases | 4 | 100% |
| Co-Dependencies | 1 | 100% |
| **Total** | **5** | **100%** |

**Cross-References:**
- Per-parameter edge cases → `../param/continue.md`, `../param/skip_permissions.md`, `../param/dry.md`, `../param/verbose.md`
- Integration tests → `../command/run.md` IT-7, IT-9, IT-10

---

## Corner Cases

### CC-1: `--continue` + `--dry-run` → `-c` visible in dry output

**Goal:** Verify that the `--continue` flag appears as `-c` in the dry-run command representation, confirming it is assembled correctly before execution would happen.
**Command:** `claude_runner "Next step" --continue --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "Next step"
```
**Verification:**
- Exit code is 0
- stdout contains ` -c` in the claude command
- No claude process started
**Pass Criteria:** Exit 0; `-c` flag visible in dry-run; no execution
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-2: `--skip-permissions` + `--dry-run` → `--dangerously-skip-permissions` visible

**Goal:** Verify the flag translation (`--skip-permissions` → `--dangerously-skip-permissions`) is visible in the dry-run output.
**Command:** `claude_runner "task" --skip-permissions --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- stdout contains `--dangerously-skip-permissions` (the translated form)
- No claude process started
**Pass Criteria:** Exit 0; translated flag visible; no execution
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-3: All three flags simultaneously

**Goal:** Verify that `--continue`, `--skip-permissions`, and `--dry-run` can all be specified together; `--dry-run` suppresses execution while the other two appear in the command representation.
**Command:** `claude_runner "task" --continue --skip-permissions --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- stdout contains ` -c` in the claude command
- stdout contains `--dangerously-skip-permissions`
- No claude process started (dry-run short-circuits)
**Pass Criteria:** Exit 0; all three flags processed; only two appear in command (dry-run suppresses itself)
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### CC-4: No behavior flags → env var block + minimal invocation

**Goal:** Verify that omitting all behavior flags produces an env var block and a clean minimal command with no extra flags.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
claude "task"
```
*(where `[…]` = `CLAUDE_CODE_BASH_TIMEOUT=3600000`, `CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000`, `CLAUDE_CODE_AUTO_CONTINUE=true`, `CLAUDE_CODE_TELEMETRY=false`)*
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` (default env var)
- stdout does NOT contain ` -c`
- stdout does NOT contain `--dangerously-skip-permissions`
**Pass Criteria:** Exit 0; no behavior-flag pollution in clean invocation
**Source:** [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

## Co-Dependencies

### CD-1: `--dry-run` short-circuits execution regardless of other flags

**Goal:** Verify the dependency rule: `--dry-run` prevents actual claude execution regardless of which other flags are present, including flags that would otherwise cause execution (`--continue`, `--skip-permissions`).
**Command:** `claude_runner "dangerous task" --continue --skip-permissions --model claude-opus-4-6 --max-tokens 50000 --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000
[…]
claude -c --dangerously-skip-permissions --model claude-opus-4-6 "dangerous task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000`
- stdout contains ` -c`
- stdout contains `--dangerously-skip-permissions`
- stdout contains `claude-opus-4-6`
- No claude process spawned (critical — `--skip-permissions` would otherwise bypass confirmations)
**Pass Criteria:** Exit 0; `--dry-run` wins regardless of all other flags; no execution occurs
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)
