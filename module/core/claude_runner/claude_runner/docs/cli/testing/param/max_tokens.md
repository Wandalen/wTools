# Parameter: `max_tokens::`

- **Type:** [`TokenCount`](../../types.md#type--tokencount)
- **Group:** [Resource Control](../../parameter_groups.md#group--4-resource-control)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | Valid decimal integer accepted | Edge Cases | P0 |
| EC-2 | Omitted → default 200000 used | Edge Cases | P0 |
| EC-3 | `"0"` → accepted (u32 zero, passed to claude) | Edge Cases | P1 |
| EC-4 | `"-1"` → rejected (negative fails u32 parse) | Edge Cases | P0 |
| EC-5 | `"abc"` → rejected: `invalid --max-tokens value: abc` | Edge Cases | P0 |
| EC-6 | u32::MAX `"4294967295"` → accepted | Edge Cases | P1 |
| EC-7 | Overflow `"4294967296"` → rejected (exceeds u32::MAX) | Edge Cases | P1 |
| EC-8 | Duplicate `--max-tokens` → last value wins | Edge Cases | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 8 | 100% |
| **Total** | **8** | **100%** |

**Cross-References:**
- max_tokens + model interaction → `../param_group/resource_control.md` CC-1
- Integration with command → `../command/run.md` IT-8

---

## Output Format Note

`--max-tokens` sets the `CLAUDE_CODE_MAX_OUTPUT_TOKENS` environment variable in `describe_env()` output —
it does **not** appear as a `--max-tokens` flag in the `claude` command line.
The env var is always present (default: `200000`). Override with `--max-tokens N`.

---

## Edge Cases

### EC-1: Valid decimal integer accepted

**Goal:** Verify a typical decimal integer is parsed and forwarded as `CLAUDE_CODE_MAX_OUTPUT_TOKENS=1000` env var.
**Command:** `claude_runner "task" --max-tokens 1000 --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=1000
[…]
claude "task"
```
*(where `[…]` = `CLAUDE_CODE_BASH_TIMEOUT=3600000`, `CLAUDE_CODE_BASH_MAX_TIMEOUT=7200000`, `CLAUDE_CODE_AUTO_CONTINUE=true`, `CLAUDE_CODE_TELEMETRY=false`)*
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=1000`
**Pass Criteria:** Exit 0; env var set with explicit value
**Source:** [max_tokens:: parameter](../../params.md#parameter--4-max_tokens)

---

### EC-2: Omitted → default 200000 used

**Goal:** Verify that omitting `--max-tokens` causes the default value (200000) to be set in `CLAUDE_CODE_MAX_OUTPUT_TOKENS`.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` even though the flag was not supplied
**Pass Criteria:** Exit 0; default value injected automatically
**Source:** [max_tokens:: parameter](../../params.md#parameter--4-max_tokens); [TokenCount type](../../types.md#type--tokencount)

---

### EC-3: `"0"` accepted (u32 zero)

**Goal:** Verify that the value `"0"` passes u32 parsing and is forwarded as env var (semantically questionable but not rejected by adapter).
**Command:** `claude_runner "task" --max-tokens 0 --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=0
[…]
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=0`
**Pass Criteria:** Exit 0; zero accepted by adapter (Claude may reject at runtime)
**Source:** [TokenCount type](../../types.md#type--tokencount)

---

### EC-4: `"-1"` rejected — negative fails u32 parse

**Goal:** Verify a negative value produces a parse error because it cannot be represented as u32.
**Command:** `claude_runner "task" --max-tokens -1`
**Expected Output (stderr):** `Error: invalid --max-tokens value: -1`
**Verification:**
- Exit code is 1
- stderr contains error including the invalid value
**Pass Criteria:** Exit 1; negative value rejected at parse time
**Source:** [TokenCount type](../../types.md#type--tokencount)

---

### EC-5: `"abc"` rejected — non-numeric

**Goal:** Verify a non-numeric value produces a descriptive parse error.
**Command:** `claude_runner "task" --max-tokens abc`
**Expected Output (stderr):** `Error: invalid --max-tokens value: abc`
**Verification:**
- Exit code is 1
- stderr contains `invalid --max-tokens value: abc`
**Pass Criteria:** Exit 1; exact error message format present on stderr
**Source:** [TokenCount type](../../types.md#type--tokencount)

---

### EC-6: u32::MAX accepted

**Goal:** Verify the maximum u32 value (4294967295) is accepted without overflow.
**Command:** `claude_runner "task" --max-tokens 4294967295 --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=4294967295
[…]
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=4294967295`
**Pass Criteria:** Exit 0; boundary value accepted
**Source:** [TokenCount type](../../types.md#type--tokencount)

---

### EC-7: Overflow `"4294967296"` rejected

**Goal:** Verify a value exceeding u32::MAX fails parsing.
**Command:** `claude_runner "task" --max-tokens 4294967296`
**Expected Output (stderr):** `Error: invalid --max-tokens value: 4294967296`
**Verification:**
- Exit code is 1
- stderr contains the overflow value in the error message
**Pass Criteria:** Exit 1; out-of-range value rejected
**Source:** [TokenCount type](../../types.md#type--tokencount)

---

### EC-8: Duplicate `--max-tokens` → last value wins

**Goal:** Verify that when `--max-tokens` appears multiple times, the last value is used.
**Command:** `claude_runner "task" --max-tokens 1000 --max-tokens 5000 --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=5000
[…]
claude "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=5000` (not 1000)
**Pass Criteria:** Exit 0; last-wins semantics enforced
**Source:** [max_tokens:: parameter](../../params.md#parameter--4-max_tokens)
