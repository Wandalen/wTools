# Parameter: `message::`

- **Type:** [`MessageText`](../../types.md#type--messagetext)
- **Group:** [Input](../../parameter_groups.md#group--1-input)
- **Commands:** 1 (`.run`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| EC-1 | Positional argument accepted as message | Edge Cases | P0 |
| EC-2 | `-m` short form accepted | Edge Cases | P0 |
| EC-3 | `--message` long form accepted | Edge Cases | P0 |
| EC-4 | Message with spaces preserved verbatim | Edge Cases | P0 |
| EC-5 | Message with special characters (quotes, semicolons) accepted | Edge Cases | P1 |
| EC-6 | Very long message (500+ chars) accepted | Edge Cases | P1 |
| EC-7 | `--message` without value → error: `--message requires a value` | Edge Cases | P0 |
| EC-8 | Positional + `--message` simultaneously → conflict error | Edge Cases | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Edge Cases | 8 | 100% |
| **Total** | **8** | **100%** |

**Cross-References:**
- Conflict with positional → `../param_group/input.md` CC-1
- Integration with command → `../command/run.md` IT-2, IT-3, IT-4

---

## Edge Cases

### EC-1: Positional argument accepted as message

**Goal:** Verify the adapter treats the first non-flag argv element as the message and forwards it to claude.
**Command:** `claude_runner "Fix the bug" --dry-run`
**Expected Output:**
```
claude "Fix the bug"
```
**Verification:**
- Exit code is 0
- stdout contains the message text `Fix the bug`
**Pass Criteria:** Exit 0; message appears in dry-run command output
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### EC-2: `-m` short form accepted

**Goal:** Verify `-m` is a recognized alias for `--message` and produces identical tokens.
**Command:** `claude_runner -m "Fix the bug" --dry-run`
**Expected Output:**
```
claude "Fix the bug"
```
**Verification:**
- Exit code is 0
- Output identical to EC-1
**Pass Criteria:** Exit 0; `-m` accepted without error
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### EC-3: `--message` long form accepted

**Goal:** Verify `--message` long form is recognized and equivalent to `-m`.
**Command:** `claude_runner --message "Fix the bug" --dry-run`
**Expected Output:**
```
claude "Fix the bug"
```
**Verification:**
- Exit code is 0
- Output identical to EC-1 and EC-2
**Pass Criteria:** Exit 0; all three message-setting paths produce identical output
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### EC-4: Message with spaces preserved verbatim

**Goal:** Verify multi-word messages with internal spaces are passed as a single token, not re-split.
**Command:** `claude_runner "Fix the big red bug today" --dry-run`
**Expected Output:**
```
claude "Fix the big red bug today"
```
**Verification:**
- Exit code is 0
- Entire multi-word phrase appears intact in output
- No splitting on spaces
**Pass Criteria:** Exit 0; message with internal spaces preserved verbatim
**Source:** [message:: parameter](../../params.md#parameter--1-message); [MessageText type](../../types.md#type--messagetext)

---

### EC-5: Message with special characters accepted

**Goal:** Verify the adapter passes shell-significant characters verbatim without interpretation.
**Command:** `claude_runner "Fix it; check src/*.rs" --dry-run`
**Expected Output:**
```
claude "Fix it; check src/*.rs"
```
**Verification:**
- Exit code is 0
- Semicolons and glob characters preserved unmodified
**Pass Criteria:** Exit 0; special characters in message preserved
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### EC-6: Very long message accepted

**Goal:** Verify no adapter-level length constraint is applied to the message.
**Command:** `claude_runner "$(python3 -c "print('Explain ' + 'x ' * 100)")" --dry-run`
**Expected Output:** stdout contains the full 600+ character message
**Verification:**
- Exit code is 0
- No truncation error from the adapter
- Full message text present in output
**Pass Criteria:** Exit 0; adapter accepts message of arbitrary length
**Source:** [MessageText type](../../types.md#type--messagetext)

---

### EC-7: `--message` without value → error

**Goal:** Verify `--message` as the final argv element without a following value produces a parse error.
**Command:** `claude_runner --message`
**Expected Output (stderr):** `Error: --message requires a value`
**Verification:**
- Exit code is 1
- stderr contains the error string
- No claude process started
**Pass Criteria:** Exit 1; exact error text present on stderr
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### EC-8: Positional + `--message` simultaneously → conflict error

**Goal:** Verify that supplying both a positional argument and `--message` flag is rejected.
**Command:** `claude_runner "positional" --message "also message"`
**Expected Output (stderr):** `Error: --message conflicts with a previously set message`
**Verification:**
- Exit code is 1
- stderr contains the conflict error
- No claude process started
**Pass Criteria:** Exit 1; conflict error present on stderr
**Source:** [message:: parameter](../../params.md#parameter--1-message); [Input group](../../parameter_groups.md#group--1-input)
