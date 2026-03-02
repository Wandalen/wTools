# Parameter Group: Environment

- **Parameters:** `dir::`, `session_dir::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | `--dir` + `--session-dir` both specified → both appear in output | Corner Cases | P0 |
| CC-2 | `--dir` set, `--session-dir` omitted → only `cd` prefix set | Corner Cases | P0 |
| CC-3 | `--session-dir` + `--continue` → `CLAUDE_CODE_SESSION_DIR` set and `-c` flag added | Corner Cases | P1 |
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

## Output Format Note

- `--dir` → `cd <path>` line in `describe()` (appears after the env var block)
- `--session-dir` → `CLAUDE_CODE_SESSION_DIR=<path>` env var line in `describe_env()`
- Full output order: env var block (`CLAUDE_CODE_MAX_OUTPUT_TOKENS`, `CLAUDE_CODE_BASH_TIMEOUT`, etc., `CLAUDE_CODE_SESSION_DIR` if set) → `cd <path>` (if dir set) → `claude ...`
- Expected output blocks below show only the lines relevant to each test case.

---

## Corner Cases

### CC-1: `--dir` + `--session-dir` both specified → both appear in output

**Goal:** Verify that `--dir` and `--session-dir` can be combined and produce independent output in the correct format.
**Command:** `claude_runner "task" --dir /workspace --session-dir /sessions --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
CLAUDE_CODE_SESSION_DIR=/sessions
cd /workspace
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_SESSION_DIR=/sessions`
- stdout contains `cd /workspace`
- Both appear independently without conflict
**Pass Criteria:** Exit 0; both parameters produce correct output independently
**Source:** [Environment group](../../parameter_groups.md#group--2-environment)

---

### CC-2: `--dir` set, `--session-dir` omitted → only `cd` prefix set

**Goal:** Verify that omitting `--session-dir` while providing `--dir` results in only the `cd` prefix and no session dir env var.
**Command:** `claude_runner "task" --dir /workspace --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
cd /workspace
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /workspace`
- stdout does NOT contain `CLAUDE_CODE_SESSION_DIR`
**Pass Criteria:** Exit 0; only the provided parameter appears in output
**Source:** [Environment group](../../parameter_groups.md#group--2-environment)

---

### CC-3: `--session-dir` + `--continue` → session dir set and `-c` flag added

**Goal:** Verify the common workflow of resuming a session stored in a specific directory: both `CLAUDE_CODE_SESSION_DIR` and `-c` appear in the invocation.
**Command:** `claude_runner "Next step" --session-dir /sessions/proj --continue --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_SESSION_DIR=/sessions/proj
claude -c "Next step"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_SESSION_DIR=/sessions/proj`
- stdout contains ` -c` in the claude command
**Pass Criteria:** Exit 0; session dir and continue flag coexist correctly
**Source:** [Environment group](../../parameter_groups.md#group--2-environment); [continue:: parameter](../../params.md#parameter--3-continue)

---

### CC-4: Both paths nonexistent → adapter accepts both

**Goal:** Verify the adapter applies no existence check to either path; Claude Code is responsible for validation at runtime.
**Command:** `claude_runner "task" --dir /nonexistent/work --session-dir /nonexistent/sessions --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_SESSION_DIR=/nonexistent/sessions
cd /nonexistent/work
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_SESSION_DIR=/nonexistent/sessions`
- stdout contains `cd /nonexistent/work`
- No filesystem validation error from adapter
**Pass Criteria:** Exit 0; adapter defers path validation to Claude Code for both parameters
**Source:** [Environment group](../../parameter_groups.md#group--2-environment); [PathArg type](../../types.md#type--patharg)
