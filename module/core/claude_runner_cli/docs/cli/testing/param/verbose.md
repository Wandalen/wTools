# Parameter: `verbose::`

- **Aliases:** `-v`, `--verbose`
- **Type:** `bool` (flag, no value)
- **Default:** `false`
- **Group:** [Behavior Flags](../../parameter_groups.md#group--3-behavior-flags)
- **FR:** FR-12

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | `-v` short form accepted, exit 0 | Edge Cases | P0 |
| EC-2 | `--verbose` long form accepted, exit 0 | Edge Cases | P0 |
| EC-3 | verbose output goes to stderr | Edge Cases | P0 |
| EC-4 | `--verbose` + `--dry-run` → dry-run wins; stderr empty | Edge Cases | P0 |
| EC-5 | verbose does not write to stdout | Edge Cases | P0 |
| EC-6 | verbose with full flag set → complete preview on stderr | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 6 | 100% |
| **Total** | **6** | **100%** |

**Cross-References:**
- Behavior Flags group → `../param_group/behavior_flags.md`
- dry:: interaction → `dry.md`
- Integration tests → `../command/run.md` (IT-13)

---

## Edge Cases

### EC-1: `-v` short form accepted, exit 0

**Goal:** Verify the `-v` short flag is parsed without error and produces the same behavior as `--verbose`. Use `--dry-run` to make the test safe in CI environments without a live Claude process.
**Command:** `claude_runner -v --message "hi" --dry-run`
**Expected Output:** (stdout) dry-run preview, exit 0. `-v` sets verbose=true, but dry-run wins and exits first; no error from `-v` parsing.
**Verification:**
- Exit code is 0
- stdout contains the assembled command (dry-run output)
- stderr is empty
- `-v` flag accepted without "unknown argument" error
**Pass Criteria:** Exit 0; `-v` parses cleanly alongside `--dry-run`
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); FR-12

---

### EC-2: `--verbose` long form accepted, exit 0

**Goal:** Verify `--verbose` is accepted as the canonical long form and produces identical behavior to `-v`.
**Command:** `claude_runner --verbose --message "hi" --dry-run`
**Expected Output:** (stdout) dry-run preview, exit 0
**Verification:**
- Exit code is 0
- stdout output identical to EC-1
- `--verbose` accepted without error
**Pass Criteria:** Exit 0; `--verbose` long form parses cleanly; identical result to `-v`
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); FR-12

---

### EC-3: verbose output goes to stderr

**Goal:** Verify that `--verbose` writes the assembled env + command description to **stderr**, not stdout. Ensures Claude's real stdout output remains uncontaminated by the preview.
**Setup:** Claude binary may or may not be installed. `eprintln!` writes before `execute()` is called, so the preview always appears in stderr regardless of Claude availability.
**Command:** `claude_runner --message "verbose-preview" --verbose`
**Verification:**
- stderr contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=` (env var from `describe_env()`)
- stderr contains `claude` (the command from `describe()`)
- The preview is present in stderr even if the process exits non-zero (no claude binary in CI)
**Pass Criteria:** stderr has preview; preview written before execution attempt
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); FR-12

---

### EC-4: `--verbose` + `--dry-run` → dry-run wins; stderr empty

**Goal:** Verify that when both `--verbose` and `--dry-run` are set, `--dry-run` takes precedence: the preview goes to stdout and the `eprintln!` stderr path is never reached.
**Command:** `claude_runner --message "task" --dry-run --verbose`
**Expected Output:** (stdout) assembled command preview; exit 0
**Verification:**
- Exit code is 0
- stdout contains the assembled command (dry-run behavior)
- stderr is empty (the `eprintln!` line is never reached when `is_dry = true`)
- No execution of Claude Code
**Pass Criteria:** Exit 0; dry-run wins; stdout has preview; stderr empty
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); [dry:: parameter](dry.md)

---

### EC-5: verbose does not write to stdout

**Goal:** Verify that `--verbose` does not pollute stdout. Scripts that capture Claude's output via stdout piping must see only Claude's real response, not the diagnostic preview.
**Command:** `claude_runner --message "verbose-stdout-clean" --verbose`
**Verification:**
- stdout does NOT contain `CLAUDE_CODE_MAX_OUTPUT_TOKENS=`
- stdout does NOT contain a `cd ` line (from `describe()`)
- Claude's real output (if any) flows to stdout without verbose noise
**Pass Criteria:** stdout is free of env var lines and command description; verbose goes to stderr only
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); FR-12

---

### EC-6: verbose with full flag set → complete preview on stderr

**Goal:** Verify that when all behavior-modifying flags are combined with `--verbose`, the stderr preview accurately reflects all of them — proving the preview is derived from the builder state, not from a hardcoded template.
**Command:** `claude_runner "task" --dir /tmp --continue --skip-permissions --model claude-opus-4-6 --max-tokens 50000 --verbose`
**Verification (stderr):**
- Contains `cd /tmp` (working directory prefix from `describe()`)
- Contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000` (explicit override, not default)
- Contains `-c` (continue flag)
- Contains `--dangerously-skip-permissions` (translated from `--skip-permissions`)
- Contains `claude-opus-4-6` (model name)
- Contains `"task"` (quoted message)
**Pass Criteria:** All five flags present in stderr preview; preview correctly reflects the full builder state
**Source:** [verbose:: parameter](../../params.md#parameter--9-verbose); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)
