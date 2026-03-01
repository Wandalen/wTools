# Parameter: `dry::`

- **Type:** `bool`
- **Group:** [Behavior Flags](../../parameter_groups.md#group--3-behavior-flags)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | `--dry-run` present → prints command, no claude process started | Edge Cases | P0 |
| EC-2 | `--dry-run` absent → claude executes normally | Edge Cases | P0 |
| EC-3 | `--dry-run` + `--dir` → `CLAUDE_DIR=...` appears in output | Edge Cases | P0 |
| EC-4 | `--dry-run` + `--message` → message appears in command output | Edge Cases | P0 |
| EC-5 | `--dry-run` + `--model` → `--model` flag appears in command output | Edge Cases | P0 |
| EC-6 | `--dry-run` no options → bare `claude` command shown | Edge Cases | P1 |
| EC-7 | `--dry-run` always exits 0 regardless of other flags | Edge Cases | P0 |
| EC-8 | `--dry-run` + `--continue` → `--continue` appears in command output | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 8 | 100% |
| **Total** | **8** | **100%** |

**Cross-References:**
- dry short-circuits all flags → `../param_group/behavior_flags.md` CD-1
- Integration with command → `../command/run.md` IT-10

---

## Edge Cases

### EC-1: `--dry-run` present → prints command, no claude process started

**Goal:** Verify `--dry-run` causes `claude_runner` to print the assembled environment and command to stdout and exit without spawning a claude process.
**Command:** `claude_runner "Fix the bug" --dry-run`
**Expected Output:**
```
claude "Fix the bug"
```
**Verification:**
- Exit code is 0
- stdout contains the claude command string
- No claude process is spawned (verify with process monitor or by checking that `claude` binary is not called)
**Pass Criteria:** Exit 0; command printed; no claude execution
**Source:** [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-2: `--dry-run` absent → claude executes normally

**Goal:** Verify omitting `--dry-run` results in actual claude execution (not just printing).
**Setup:** Requires `claude` CLI installed and authenticated
**Command:** `claude_runner "Print hello" 2>&1`
**Expected Output:** Claude's response streamed to stdout
**Verification:**
- Exit code is 0 (assuming claude succeeds)
- stdout contains Claude's response text
- No dry-run output format (no bare `claude` command line)
**Pass Criteria:** Exit 0; real Claude response present
**Source:** [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-3: `--dry-run` + `--dir` → `CLAUDE_DIR` in output

**Goal:** Verify that `--dir` combined with `--dry-run` shows `CLAUDE_DIR=...` as an env var prefix in output.
**Command:** `claude_runner "task" --dir /tmp/project --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/tmp/project
claude "task"
```
**Verification:**
- Exit code is 0
- First line of stdout is `CLAUDE_DIR=/tmp/project`
- Second line is the claude command
**Pass Criteria:** Exit 0; env var line present before command line
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [dir:: parameter](../../params.md#parameter--2-dir)

---

### EC-4: `--dry-run` + `--message` → message in command output

**Goal:** Verify message text appears in the dry-run command representation.
**Command:** `claude_runner --message "Refactor the parser" --dry-run`
**Expected Output:**
```
claude "Refactor the parser"
```
**Verification:**
- Exit code is 0
- Message text present in stdout
**Pass Criteria:** Exit 0; message visible in dry-run output
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [message:: parameter](../../params.md#parameter--1-message)

---

### EC-5: `--dry-run` + `--model` → `--model` flag in output

**Goal:** Verify model selection is visible in dry-run output.
**Command:** `claude_runner "task" --model claude-opus-4-6 --dry-run`
**Expected Output:**
```
claude --model claude-opus-4-6 "task"
```
**Verification:**
- Exit code is 0
- `--model claude-opus-4-6` present in stdout
**Pass Criteria:** Exit 0; model flag visible in command representation
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [model:: parameter](../../params.md#parameter--8-model)

---

### EC-6: `--dry-run` no options → bare `claude` command

**Goal:** Verify that `--dry-run` with no other flags produces a minimal bare `claude` command.
**Command:** `claude_runner --dry-run`
**Expected Output:**
```
claude --max-tokens 200000
```
**Verification:**
- Exit code is 0
- stdout starts with `claude`
- Only the default `--max-tokens 200000` present (no other flags)
**Pass Criteria:** Exit 0; minimal command with only defaults shown
**Source:** [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-7: `--dry-run` always exits 0

**Goal:** Verify `--dry-run` exits 0 regardless of other flag combinations (no execution, no potential failure).
**Command:** `claude_runner --dry-run --model nonexistent-model-xyz`
**Expected Output:**
```
claude --model nonexistent-model-xyz --max-tokens 200000
```
**Verification:**
- Exit code is 0
- No error from adapter (unknown model is not validated at parse time)
**Pass Criteria:** Exit 0; dry-run never fails due to parameter values
**Source:** [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-8: `--dry-run` + `--continue` → `--continue` in output

**Goal:** Verify `--continue` flag appears in the dry-run command output.
**Command:** `claude_runner "Next step" --continue --dry-run`
**Expected Output:**
```
claude --continue "Next step"
```
**Verification:**
- Exit code is 0
- `--continue` present in stdout
**Pass Criteria:** Exit 0; continue flag visible in dry-run representation
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [continue:: parameter](../../params.md#parameter--3-continue)
