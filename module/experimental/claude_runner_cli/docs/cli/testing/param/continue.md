# Parameter: `continue::`

- **Type:** `bool`
- **Group:** [Behavior Flags](../../parameter_groups.md#group--3-behavior-flags)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | `--continue` present → `-c` added to claude argv | Edge Cases | P0 |
| EC-2 | `-c` short form accepted | Edge Cases | P0 |
| EC-3 | `--continue` absent → flag not added to claude argv | Edge Cases | P0 |
| EC-4 | `--continue` is boolean (takes no value argument) | Edge Cases | P0 |
| EC-5 | `--continue` with a message → both work together | Edge Cases | P1 |
| EC-6 | Duplicate `--continue` idempotent (boolean stays true) | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 6 | 100% |
| **Total** | **6** | **100%** |

**Cross-References:**
- continue + dry interaction → `../param_group/behavior_flags.md` CC-1
- Integration with command → `../command/run.md` IT-7

---

## Edge Cases

### EC-1: `--continue` present → `-c` added to claude argv

**Goal:** Verify `--continue` causes the `-c` flag to appear in the assembled claude command.
**Command:** `claude_runner "Next step" --continue --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "Next step"
```
**Verification:**
- Exit code is 0
- stdout contains ` -c` in the claude invocation
**Pass Criteria:** Exit 0; `-c` present in dry-run output
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)

---

### EC-2: `-c` short form accepted

**Goal:** Verify `-c` is a recognized alias for `--continue`.
**Command:** `claude_runner "Next step" -c --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "Next step"
```
**Verification:**
- Exit code is 0
- Output identical to EC-1
**Pass Criteria:** Exit 0; `-c` behaves identically to `--continue`
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)

---

### EC-3: `--continue` absent → flag not added to claude argv

**Goal:** Verify that when `--continue` is omitted, the claude command contains no `-c` flag.
**Command:** `claude_runner "Fresh task" --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude "Fresh task"
```
**Verification:**
- Exit code is 0
- stdout does NOT contain ` -c`
**Pass Criteria:** Exit 0; `-c` absent from dry-run output
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)

---

### EC-4: `--continue` is boolean (takes no value argument)

**Goal:** Verify `--continue` requires no following value; the next token is treated as a separate argument.
**Command:** `claude_runner --continue "My task" --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "My task"
```
**Verification:**
- Exit code is 0
- `My task` is treated as the message (positional), not the value of `--continue`
**Pass Criteria:** Exit 0; boolean flag and positional message both parsed correctly
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)

---

### EC-5: `--continue` with a message → both work together

**Goal:** Verify that `--continue` and an explicit `--message` can be combined without conflict.
**Command:** `claude_runner --continue --message "Continue refactoring" --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "Continue refactoring"
```
**Verification:**
- Exit code is 0
- Both `-c` flag and message text present in output
**Pass Criteria:** Exit 0; continue flag and message coexist
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)

---

### EC-6: Duplicate `--continue` idempotent

**Goal:** Verify that specifying `--continue` multiple times does not cause an error; the boolean stays true.
**Command:** `claude_runner --continue --continue "task" --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude -c "task"
```
**Verification:**
- Exit code is 0
- `-c` appears exactly once in output (not duplicated)
**Pass Criteria:** Exit 0; duplicate boolean flag handled gracefully
**Source:** [continue:: parameter](../../params.md#parameter--3-continue)
