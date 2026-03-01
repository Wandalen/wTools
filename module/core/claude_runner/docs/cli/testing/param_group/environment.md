# Parameter Group: Environment

- **Parameters:** `dir::`, `session_dir::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | `--dir` + `--session-dir` both specified → both env vars set independently | Corner Cases | P0 |
| CC-2 | `--dir` set, `--session-dir` omitted → only `CLAUDE_DIR` set | Corner Cases | P0 |
| CC-3 | `--session-dir` + `--continue` → `CLAUDE_SESSION_DIR` set and `--continue` flag added | Corner Cases | P1 |
| CC-4 | Both paths nonexistent → adapter accepts both, Claude reports errors at runtime | Corner Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Corner Cases | 4 | 100% |
| **Total** | **4** | **100%** |

**Cross-References:**
- Per-parameter edge cases → `../param/dir.md`, `../param/session_dir.md`
- Integration tests → `../command/run.md` IT-5, IT-11

---

## Corner Cases

### CC-1: `--dir` + `--session-dir` both specified → both env vars set

**Goal:** Verify that `--dir` and `--session-dir` can be combined and produce two independent environment variables.
**Command:** `claude_runner "task" --dir /workspace --session-dir /sessions --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/workspace
CLAUDE_SESSION_DIR=/sessions
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains both `CLAUDE_DIR=/workspace` and `CLAUDE_SESSION_DIR=/sessions`
- Both appear before the claude command line
**Pass Criteria:** Exit 0; both env vars present independently
**Source:** [Environment group](../../parameter_groups.md#group--2-environment)

---

### CC-2: `--dir` set, `--session-dir` omitted → only `CLAUDE_DIR` set

**Goal:** Verify that omitting `--session-dir` while providing `--dir` results in only `CLAUDE_DIR` being set.
**Command:** `claude_runner "task" --dir /workspace --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/workspace
claude "task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_DIR=/workspace` present
- `CLAUDE_SESSION_DIR` absent from output
**Pass Criteria:** Exit 0; only the provided env var appears in output
**Source:** [Environment group](../../parameter_groups.md#group--2-environment)

---

### CC-3: `--session-dir` + `--continue` → session dir set and continue flag added

**Goal:** Verify the common workflow of resuming a session stored in a specific directory: both `CLAUDE_SESSION_DIR` and `--continue` appear in the invocation.
**Command:** `claude_runner "Next step" --session-dir /sessions/proj --continue --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/sessions/proj
claude --continue "Next step"
```
**Verification:**
- Exit code is 0
- `CLAUDE_SESSION_DIR=/sessions/proj` present in env vars block
- `--continue` present in claude command
**Pass Criteria:** Exit 0; session dir and continue flag coexist correctly
**Source:** [Environment group](../../parameter_groups.md#group--2-environment); [continue:: parameter](../../params.md#parameter--3-continue)

---

### CC-4: Both paths nonexistent → adapter accepts both

**Goal:** Verify the adapter applies no existence check to either path; Claude Code is responsible for validation at runtime.
**Command:** `claude_runner "task" --dir /nonexistent/work --session-dir /nonexistent/sessions --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/nonexistent/work
CLAUDE_SESSION_DIR=/nonexistent/sessions
claude "task"
```
**Verification:**
- Exit code is 0
- Both env vars present in output despite nonexistent paths
- No filesystem validation error from adapter
**Pass Criteria:** Exit 0; adapter defers path validation to Claude Code for both parameters
**Source:** [Environment group](../../parameter_groups.md#group--2-environment); [PathArg type](../../types.md#type--patharg)
