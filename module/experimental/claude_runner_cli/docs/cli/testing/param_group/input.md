# Parameter Group: Input

- **Parameters:** `message::`
- **Commands:** `.run`

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| CC-1 | Positional + `--message` simultaneously → conflict error | Corner Cases | P0 |
| CC-2 | No message at all → valid (Claude runs without a prompt) | Corner Cases | P1 |
| CC-3 | Message with shell-significant characters passed verbatim | Corner Cases | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Corner Cases | 3 | 100% |
| **Total** | **3** | **100%** |

**Cross-References:**
- Per-parameter edge cases → `../param/message.md`
- Integration tests → `../command/run.md`

---

## Corner Cases

### CC-1: Positional + `--message` simultaneously → conflict error

**Goal:** Verify that supplying a message via both a positional argument and `--message` is rejected with a clear conflict error.
**Command:** `claude_runner "positional message" --message "explicit message"`
**Expected Output (stderr):** `Error: --message conflicts with a previously set message (positional or duplicate --message)`
**Verification:**
- Exit code is 1
- stderr contains the conflict error text
- No claude process started
**Pass Criteria:** Exit 1; conflict detected and reported; message source ambiguity resolved by rejection
**Source:** [Input group](../../parameter_groups.md#group--1-input); [message:: parameter](../../params.md#parameter--1-message)

---

### CC-2: No message at all → valid invocation

**Goal:** Verify that omitting the message entirely is a valid invocation; Claude Code runs without a pre-supplied prompt.
**Command:** `claude_runner --dry-run`
**Expected Output:**
```
CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000
[…]
claude
```
**Verification:**
- Exit code is 0
- No error about missing message
- stdout contains `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` (default env var)
- stdout contains `claude` (command without message argument)
**Pass Criteria:** Exit 0; message-less invocation accepted by adapter
**Source:** [Input group](../../parameter_groups.md#group--1-input)

---

### CC-3: Message with shell-significant characters passed verbatim

**Goal:** Verify characters that are special in bash (semicolons, pipes, ampersands, dollar signs) survive the shell-quoting round-trip and appear intact in the token.
**Command:** `claude_runner "Run: echo \$HOME && ls | head -5" --dry-run`
**Expected Output:**
```
claude "Run: echo $HOME && ls | head -5"
```
**Verification:**
- Exit code is 0
- The full message including `$HOME`, `&&`, `|` appears in stdout
- No shell interpretation of the embedded special characters
**Pass Criteria:** Exit 0; special characters preserved verbatim as part of the message token
**Source:** [Input group](../../parameter_groups.md#group--1-input); [MessageText type](../../types.md#type--messagetext)
