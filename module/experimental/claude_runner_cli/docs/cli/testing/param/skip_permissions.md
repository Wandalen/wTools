# Parameter: `skip_permissions::`

- **Type:** `bool`
- **Group:** [Behavior Flags](../../parameter_groups.md#group--3-behavior-flags)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | `--skip-permissions` present → `--dangerously-skip-permissions` in claude argv | Edge Cases | P0 |
| EC-2 | `--skip-permissions` absent → flag not added to claude argv | Edge Cases | P0 |
| EC-3 | `--skip-permissions` is boolean (takes no value argument) | Edge Cases | P0 |
| EC-4 | `--skip-permissions` + `--dry-run` → flag visible in dry output | Edge Cases | P0 |
| EC-5 | `--skip-permissions` + `--message` → both work together | Edge Cases | P1 |
| EC-6 | Duplicate `--skip-permissions` idempotent (boolean stays true) | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 6 | 100% |
| **Total** | **6** | **100%** |

**Cross-References:**
- skip_permissions + dry interaction → `../param_group/behavior_flags.md` CC-2
- Integration with command → `../command/run.md` IT-9

---

## Edge Cases

### EC-1: `--skip-permissions` → `--dangerously-skip-permissions` in claude argv

**Goal:** Verify `--skip-permissions` maps to the longer `--dangerously-skip-permissions` flag in the assembled claude command.
**Command:** `claude_runner "task" --skip-permissions --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- stdout contains `--dangerously-skip-permissions` (not the shorter `--skip-permissions`)
**Pass Criteria:** Exit 0; flag translated to full claude flag name
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions)

---

### EC-2: `--skip-permissions` absent → flag not added

**Goal:** Verify omitting `--skip-permissions` produces a claude command without the permissions flag.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
claude "task"
```
**Verification:**
- Exit code is 0
- stdout does NOT contain `--dangerously-skip-permissions`
**Pass Criteria:** Exit 0; permissions flag absent from output
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions)

---

### EC-3: `--skip-permissions` is boolean (takes no value)

**Goal:** Verify `--skip-permissions` requires no following value; the next token is treated as a separate argument.
**Command:** `claude_runner --skip-permissions "My task" --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "My task"
```
**Verification:**
- Exit code is 0
- `My task` treated as positional message, not as value of `--skip-permissions`
**Pass Criteria:** Exit 0; boolean flag and positional message both parsed correctly
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions)

---

### EC-4: `--skip-permissions` + `--dry-run` → flag visible in dry output

**Goal:** Verify the combination of `--skip-permissions` and `--dry-run` shows the translated flag without executing.
**Command:** `claude_runner "task" --skip-permissions --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- `--dangerously-skip-permissions` present in stdout
- No claude process started
**Pass Criteria:** Exit 0; flag visible; no execution
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions); [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-5: `--skip-permissions` + `--message` → both work together

**Goal:** Verify `--skip-permissions` and `--message` can be combined without conflict.
**Command:** `claude_runner --skip-permissions --message "Automate everything" --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "Automate everything"
```
**Verification:**
- Exit code is 0
- Both `--dangerously-skip-permissions` and message text present in output
**Pass Criteria:** Exit 0; flag and message coexist
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions)

---

### EC-6: Duplicate `--skip-permissions` idempotent

**Goal:** Verify specifying `--skip-permissions` multiple times does not cause an error; boolean stays true.
**Command:** `claude_runner --skip-permissions --skip-permissions "task" --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions "task"
```
**Verification:**
- Exit code is 0
- `--dangerously-skip-permissions` appears exactly once (not duplicated)
**Pass Criteria:** Exit 0; duplicate boolean handled gracefully
**Source:** [skip_permissions:: parameter](../../params.md#parameter--5-skip_permissions)
