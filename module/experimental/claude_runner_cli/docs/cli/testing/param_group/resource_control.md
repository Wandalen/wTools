# Parameter Group: Resource Control

- **Parameters:** `max_tokens::`, `model::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | `--max-tokens` + `--model` both specified → both flags in claude argv | Corner Cases | P0 |
| CC-2 | `--max-tokens 1` (minimal) with large message → accepted | Corner Cases | P1 |
| CC-3 | `--model <invalid>` + `--dry-run` → dry prints command without validating model | Corner Cases | P1 |
| CC-4 | Explicit `--max-tokens 200000` vs omitted → identical behavior (same default) | Corner Cases | P2 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Corner Cases | 4 | 100% |
| **Total** | **4** | **100%** |

**Cross-References:**
- Per-parameter edge cases → `../param/max_tokens.md`, `../param/model.md`
- Integration tests → `../command/run.md` IT-8, IT-12

---

## Corner Cases

### CC-1: `--max-tokens` + `--model` both specified → both appear in output

**Goal:** Verify that specifying both `--max-tokens` and `--model` together produces output with the token env var and the model flag.
**Command:** `claude_runner "task" --max-tokens 50000 --model claude-opus-4-6 --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000
[…]
claude --model claude-opus-4-6 "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=50000` (env var, not a command flag)
- stdout contains `--model claude-opus-4-6` (in the claude command)
- Both appear without conflict
**Pass Criteria:** Exit 0; token count in env var; model flag in command
**Source:** [Resource Control group](../../parameter_groups.md#group--4-resource-control)

---

### CC-2: `--max-tokens 1` (minimal) with large message → accepted

**Goal:** Verify the adapter applies no semantic validation of the token count against message length; the pair is forwarded to Claude Code as-is.
**Command:** `claude_runner "$(python3 -c "print('word ' * 200)")" --max-tokens 1 --dry-run`
**Expected Output:** *(key lines shown)*
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=1
[…]
claude "<large message>"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=1`
- Full large message present in stdout
- No adapter-level rejection of the tiny token count vs. large input
**Pass Criteria:** Exit 0; adapter does not validate token count against message size
**Source:** [Resource Control group](../../parameter_groups.md#group--4-resource-control); [TokenCount type](../../types.md#type--tokencount)

---

### CC-3: `--model <invalid>` + `--dry-run` → prints command without validating model

**Goal:** Verify that an invalid model name combined with `--dry-run` produces output without any model validation, demonstrating that dry-run shows what would be attempted.
**Command:** `claude_runner "task" --model definitely-not-a-real-model --dry-run`
**Expected Output:** *(env var block precedes)*
```
claude --model definitely-not-a-real-model "task"
```
**Verification:**
- Exit code is 0
- stdout contains `--model definitely-not-a-real-model` in the claude command
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` (default)
- No validation error from adapter
**Pass Criteria:** Exit 0; adapter defers model validation to Claude Code; dry-run shows the would-be command
**Source:** [Resource Control group](../../parameter_groups.md#group--4-resource-control); [ModelName type](../../types.md#type--modelname)

---

### CC-4: Explicit `--max-tokens 200000` vs omitted → identical behavior

**Goal:** Verify that explicitly providing the default value of `--max-tokens` produces an identical command to omitting it (confirming the default is 200000 and is always injected).
**Command A (explicit):** `claude_runner "task" --max-tokens 200000 --dry-run`
**Command B (omitted):** `claude_runner "task" --dry-run`
**Expected Output (both):**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
claude "task"
```
**Verification:**
- Exit code is 0 for both
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` for both
- stdout is identical for both commands
**Pass Criteria:** Exit 0; default value (200000) always applied; explicit and implicit paths produce identical output
**Source:** [Resource Control group](../../parameter_groups.md#group--4-resource-control); [TokenCount type](../../types.md#type--tokencount)
