# Parameter: `model::`

- **Type:** [`ModelName`](../../types.md#type--modelname)
- **Group:** [Resource Control](../../parameter_groups.md#group--4-resource-control)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | `--model <name>` → `--model <name>` added to claude argv | Edge Cases | P0 |
| EC-2 | Omitted → no `--model` flag in claude argv | Edge Cases | P0 |
| EC-3 | `--model` without value → error: `--model requires a value` | Edge Cases | P0 |
| EC-4 | `--model ""` empty string → accepted by adapter, rejected by Claude at runtime | Edge Cases | P1 |
| EC-5 | Duplicate `--model` → last value wins | Edge Cases | P0 |
| EC-6 | `--model` + `--dry-run` → `--model` visible in dry output | Edge Cases | P0 |
| EC-7 | Unknown model name → accepted by adapter, rejected by Claude at runtime | Edge Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 7 | 100% |
| **Total** | **7** | **100%** |

**Cross-References:**
- model + max_tokens interaction → `../param_group/resource_control.md` CC-1
- Integration with command → `../command/run.md` IT-12

---

## Edge Cases

### EC-1: `--model <name>` → `--model <name>` in claude argv

**Goal:** Verify a model name is forwarded verbatim as `--model NAME` in the assembled claude command.
**Command:** `claude_runner "task" --model claude-opus-4-6 --dry-run`
**Expected Output:**
```
claude --model claude-opus-4-6 "task"
```
**Verification:**
- Exit code is 0
- `--model claude-opus-4-6` present in stdout
**Pass Criteria:** Exit 0; model flag with exact name present in output
**Source:** [model:: parameter](../../params.md#parameter--8-model)

---

### EC-2: Omitted → no `--model` flag in claude argv

**Goal:** Verify that omitting `--model` produces a claude command without a `--model` flag.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
claude "task"
```
**Verification:**
- Exit code is 0
- stdout does NOT contain `--model`
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` (default env var)
**Pass Criteria:** Exit 0; no model flag when parameter omitted
**Source:** [model:: parameter](../../params.md#parameter--8-model)

---

### EC-3: `--model` without value → error

**Goal:** Verify `--model` as the final argv element without a following value produces a parse error.
**Command:** `claude_runner "task" --model`
**Expected Output (stderr):** `Error: --model requires a value`
**Verification:**
- Exit code is 1
- stderr contains the error string
**Pass Criteria:** Exit 1; error text present on stderr
**Source:** [model:: parameter](../../params.md#parameter--8-model)

---

### EC-4: `--model ""` empty string → accepted by adapter

**Goal:** Verify an empty string is accepted by the adapter and forwarded (Claude Code validates model names at runtime).
**Command:** `claude_runner "task" --model "" --dry-run`
**Expected Output:**
```
claude --model  "task"
```
**Verification:**
- Exit code is 0 (adapter does not validate model names)
- `--model` present in output with empty value
**Pass Criteria:** Exit 0; adapter defers model validation to Claude Code
**Source:** [ModelName type](../../types.md#type--modelname)

---

### EC-5: Duplicate `--model` → last value wins

**Goal:** Verify that when `--model` appears multiple times, the last value is used.
**Command:** `claude_runner "task" --model claude-haiku-4-5-20251001 --model claude-opus-4-6 --dry-run`
**Expected Output:**
```
claude --model claude-opus-4-6 "task"
```
**Verification:**
- Exit code is 0
- `--model claude-opus-4-6` present (not haiku)
**Pass Criteria:** Exit 0; last-wins semantics enforced
**Source:** [model:: parameter](../../params.md#parameter--8-model)

---

### EC-6: `--model` + `--dry-run` → model flag visible

**Goal:** Verify model selection is clearly visible in dry-run output for auditing purposes.
**Command:** `claude_runner "task" --model claude-sonnet-4-6 --dry-run`
**Expected Output:**
```
claude --model claude-sonnet-4-6 "task"
```
**Verification:**
- Exit code is 0
- `--model claude-sonnet-4-6` appears in stdout
**Pass Criteria:** Exit 0; model flag visible for pre-execution verification
**Source:** [model:: parameter](../../params.md#parameter--8-model); [dry:: parameter](../../params.md#parameter--6-dry)

---

### EC-7: Unknown model name → accepted by adapter

**Goal:** Verify an unknown model name passes through the adapter without error; Claude Code rejects it at runtime.
**Command:** `claude_runner "task" --model totally-made-up-model --dry-run`
**Expected Output:**
```
claude --model totally-made-up-model "task"
```
**Verification:**
- Exit code is 0 (adapter does not validate model names)
- Unknown model name forwarded verbatim
**Pass Criteria:** Exit 0; adapter passes unknown model names through
**Source:** [ModelName type](../../types.md#type--modelname)
