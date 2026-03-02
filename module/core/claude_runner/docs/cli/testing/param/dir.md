# Parameter: `dir::`

- **Type:** [`PathArg`](../../types.md#type--patharg)
- **Group:** [Environment](../../parameter_groups.md#group--2-environment)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | Absolute path accepted | Edge Cases | P0 |
| EC-2 | Relative path accepted | Edge Cases | P0 |
| EC-3 | `~/` tilde path accepted (shell expands before reaching adapter) | Edge Cases | P1 |
| EC-4 | Path with spaces accepted | Edge Cases | P1 |
| EC-5 | Nonexistent path accepted by adapter (runtime failure deferred to Claude) | Edge Cases | P1 |
| EC-6 | `--dir` without value → error: `--dir requires a value` | Edge Cases | P0 |
| EC-7 | `-d` short form accepted | Edge Cases | P0 |
| EC-8 | Duplicate `--dir` → last value wins silently | Edge Cases | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 8 | 100% |
| **Total** | **8** | **100%** |

**Cross-References:**
- Dir + session_dir interaction → `../param_group/environment.md` CC-1
- Integration with command → `../command/run.md` IT-5, IT-6

---

## Output Format Note

`--dir` produces a `cd <path>` prefix line in `describe()`, **not** an environment variable.
Full dry-run output: env var block from `describe_env()` (always includes `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000`,
`CLAUDE_CODE_BASH_TIMEOUT=3600000`, `CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000`, `CLAUDE_CODE_AUTO_CONTINUE=true`,
`CLAUDE_CODE_TELEMETRY=false`) followed by the `cd <path>` line and then the `claude` invocation.
Expected output blocks below show only the lines relevant to each test case.

---

## Edge Cases

### EC-1: Absolute path accepted

**Goal:** Verify an absolute path is accepted and produces a `cd /tmp` prefix in the dry-run output.
**Command:** `claude_runner "task" --dir /tmp --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /tmp
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /tmp`
**Pass Criteria:** Exit 0; `cd` prefix present with exact path
**Source:** [dir:: parameter](../../params.md#parameter--2-dir)

---

### EC-2: Relative path accepted

**Goal:** Verify a relative path is accepted without normalization by the adapter.
**Command:** `claude_runner "task" --dir ./relative/path --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd ./relative/path
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd ./relative/path`
**Pass Criteria:** Exit 0; path forwarded verbatim (no resolution)
**Source:** [dir:: parameter](../../params.md#parameter--2-dir); [PathArg type](../../types.md#type--patharg)

---

### EC-3: Tilde path accepted

**Goal:** Verify shell-expanded tilde paths are accepted (the shell expands `~` before `claude_runner` sees the argument).
**Setup:** Shell must expand `~` (standard behavior in bash/zsh)
**Command:** `claude_runner "task" --dir ~/projects --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /home/<user>/projects
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /home/` (tilde-expanded absolute path, not literal `~`)
**Pass Criteria:** Exit 0; tilde-expanded path present in cd line
**Source:** [PathArg type](../../types.md#type--patharg)

---

### EC-4: Path with spaces accepted

**Goal:** Verify paths containing spaces are handled when quoted in the shell.
**Command:** `claude_runner "task" --dir "/path with spaces" --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /path with spaces
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /path with spaces` (unquoted — human-readable per FR-21)
**Pass Criteria:** Exit 0; path with spaces forwarded verbatim in cd line
**Source:** [PathArg type](../../types.md#type--patharg)

---

### EC-5: Nonexistent path accepted by adapter

**Goal:** Verify the adapter applies no filesystem existence check; Claude Code validates at runtime.
**Command:** `claude_runner "task" --dir /this/does/not/exist --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /this/does/not/exist
claude "task"
```
**Verification:**
- Exit code is 0 (adapter succeeds)
- stdout contains `cd /this/does/not/exist`
- No error from `claude_runner` itself
**Pass Criteria:** Exit 0; adapter defers existence validation to Claude Code
**Source:** [PathArg type](../../types.md#type--patharg)

---

### EC-6: `--dir` without value → error

**Goal:** Verify `--dir` as the final argv element without a following value produces a parse error.
**Command:** `claude_runner "task" --dir`
**Expected Output (stderr):** `Error: --dir requires a value`
**Verification:**
- Exit code is 1
- stderr contains the error string
**Pass Criteria:** Exit 1; error text present on stderr
**Source:** [dir:: parameter](../../params.md#parameter--2-dir)

---

### EC-7: `-d` short form accepted

**Goal:** Verify `-d` is a recognized alias for `--dir`.
**Command:** `claude_runner "task" -d /tmp --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /tmp
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /tmp`
- Output identical to EC-1
**Pass Criteria:** Exit 0; `-d` produces same cd prefix as `--dir`
**Source:** [dir:: parameter](../../params.md#parameter--2-dir)

---

### EC-8: Duplicate `--dir` → last value wins

**Goal:** Verify that when `--dir` appears multiple times, only the last value is used.
**Command:** `claude_runner "task" --dir /first --dir /last --dry-run`
**Expected Output:** *(env var block precedes; see format note above)*
```
cd /last
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `cd /last`
- stdout does NOT contain `cd /first`
**Pass Criteria:** Exit 0; last-wins semantics enforced silently
**Source:** [dir:: parameter](../../params.md#parameter--2-dir)
