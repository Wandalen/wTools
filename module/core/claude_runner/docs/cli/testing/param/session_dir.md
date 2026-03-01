# Parameter: `session_dir::`

- **Type:** [`PathArg`](../../types.md#type--patharg)
- **Group:** [Environment](../../parameter_groups.md#group--2-environment)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | Absolute path → `CLAUDE_SESSION_DIR` set | Edge Cases | P0 |
| EC-2 | Relative path accepted | Edge Cases | P1 |
| EC-3 | Omitted → `CLAUDE_SESSION_DIR` not set | Edge Cases | P0 |
| EC-4 | `--session-dir` without value → error: `--session-dir requires a value` | Edge Cases | P0 |
| EC-5 | Duplicate `--session-dir` → last value wins | Edge Cases | P0 |
| EC-6 | Path with spaces accepted | Edge Cases | P1 |
| EC-7 | `--session-dir` + `--continue` → natural combination works | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 7 | 100% |
| **Total** | **7** | **100%** |

**Cross-References:**
- session_dir + dir interaction → `../param_group/environment.md` CC-1
- Integration with command → `../command/run.md` IT-11

---

## Edge Cases

### EC-1: Absolute path → `CLAUDE_SESSION_DIR` set

**Goal:** Verify an absolute path is forwarded as `CLAUDE_SESSION_DIR` in the process environment.
**Command:** `claude_runner "task" --session-dir /home/user/.sessions --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/home/user/.sessions
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_SESSION_DIR=/home/user/.sessions`
**Pass Criteria:** Exit 0; env var present with exact path
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir)

---

### EC-2: Relative path accepted

**Goal:** Verify a relative path is accepted without normalization by the adapter.
**Command:** `claude_runner "task" --session-dir ./sessions --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=./sessions
claude "task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_SESSION_DIR` set to the relative path string as-is
**Pass Criteria:** Exit 0; relative path forwarded verbatim
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir); [PathArg type](../../types.md#type--patharg)

---

### EC-3: Omitted → `CLAUDE_SESSION_DIR` not set

**Goal:** Verify omitting `--session-dir` results in no `CLAUDE_SESSION_DIR` in the environment output.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
claude "task"
```
**Verification:**
- Exit code is 0
- stdout does NOT contain `CLAUDE_SESSION_DIR`
**Pass Criteria:** Exit 0; no session-dir env var when parameter omitted
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir)

---

### EC-4: `--session-dir` without value → error

**Goal:** Verify `--session-dir` as the final argv element without a following value produces a parse error.
**Command:** `claude_runner "task" --session-dir`
**Expected Output (stderr):** `Error: --session-dir requires a value`
**Verification:**
- Exit code is 1
- stderr contains the error string
**Pass Criteria:** Exit 1; error text present on stderr
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir)

---

### EC-5: Duplicate `--session-dir` → last value wins

**Goal:** Verify that when `--session-dir` appears multiple times, the last value is used.
**Command:** `claude_runner "task" --session-dir /first --session-dir /last --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/last
claude "task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_SESSION_DIR` is `/last`, not `/first`
**Pass Criteria:** Exit 0; last-wins semantics enforced
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir)

---

### EC-6: Path with spaces accepted

**Goal:** Verify paths containing spaces are handled when quoted in the shell.
**Command:** `claude_runner "task" --session-dir "/my sessions/project" --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/my sessions/project
claude "task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_SESSION_DIR` preserves the internal spaces
**Pass Criteria:** Exit 0; path with spaces forwarded verbatim
**Source:** [PathArg type](../../types.md#type--patharg)

---

### EC-7: `--session-dir` + `--continue` → natural combination works

**Goal:** Verify the typical use case of specifying a session directory and continuing a previous conversation in it.
**Command:** `claude_runner "Next task" --session-dir /home/user/.sessions/proj --continue --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/home/user/.sessions/proj
claude --continue "Next task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_SESSION_DIR` set
- `--continue` present in claude command
**Pass Criteria:** Exit 0; both parameters active in same invocation
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir); [continue:: parameter](../../params.md#parameter--3-continue)
