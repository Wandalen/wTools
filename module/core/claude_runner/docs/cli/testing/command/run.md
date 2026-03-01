# Command: `.run`

- **Purpose:** Execute Claude Code with configurable parameters
- **Parameters:** 8 (see [params.md](../../params.md))
- **Exit Codes:** 0 (success) | 1 (argument error) | 2 (Claude runtime error)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| IT-1 | Bare invocation — no message, no flags | Integration Tests | P1 |
| IT-2 | Positional message argument | Integration Tests | P0 |
| IT-3 | `-m` short form for message | Integration Tests | P0 |
| IT-4 | `--message` long form | Integration Tests | P0 |
| IT-5 | `--dir` sets working directory | Integration Tests | P0 |
| IT-6 | `-d` short form for `--dir` | Integration Tests | P1 |
| IT-7 | `--continue` passes `--continue` to claude | Integration Tests | P0 |
| IT-8 | `--max-tokens` passes integer value | Integration Tests | P0 |
| IT-9 | `--skip-permissions` → `--dangerously-skip-permissions` | Integration Tests | P0 |
| IT-10 | `--dry-run` prints command, exits 0, no execution | Integration Tests | P0 |
| IT-11 | `--session-dir` sets `CLAUDE_SESSION_DIR` env var | Integration Tests | P1 |
| IT-12 | `--model` passes `--model NAME` to claude | Integration Tests | P1 |
| CSB-1 | Duplicate `--dir` → last value wins | Command-Specific Behavior | P0 |
| CSB-2 | Positional + `--message` simultaneously → conflict error | Command-Specific Behavior | P0 |
| CSB-3 | Claude stderr forwarded to stderr (not mixed with stdout) | Command-Specific Behavior | P1 |
| CSB-4 | Non-zero Claude exit code → `claude_runner` exits non-zero | Command-Specific Behavior | P0 |
| RWS-1 | Developer runs quick fix on current project | Real-World Scenarios | P1 |
| RWS-2 | CI pipeline with full parameter set | Real-World Scenarios | P1 |
| RWS-3 | Preview invocation with `--dry-run`, then execute for real | Real-World Scenarios | P1 |
| RWS-4 | Multi-session iterative workflow with `--continue` | Real-World Scenarios | P1 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Integration Tests | 12 | 100% |
| Command-Specific Behavior | 4 | 100% |
| Real-World Scenarios | 4 | 100% |
| **Total** | **20** | **100%** |

**Cross-References:**
- Parameter edge cases → `../param/*.md`
- Group corner cases → `../param_group/*.md`

---

## Integration Tests

### IT-1: Bare invocation — no message, no flags

**Goal:** Verify the adapter initializes cleanly with no arguments, assembles the minimal command (only the default `--max-tokens`), and delegates to Claude Code without error.
**Setup:** Use `--dry-run` to verify command assembly without requiring a live Claude process.
**Command:** `claude_runner --dry-run`
**Expected Output:**
```
claude --max-tokens 200000
```
**Verification:**
- Exit code is 0
- stdout contains `claude --max-tokens 200000`
- No message argument appended
- No extra flags present
**Pass Criteria:** Exit 0; default-only command assembled; no message argument when none supplied
**Source:** [`.run` command](../../commands.md#command--1-run); [message:: parameter](../../params.md#parameter--1-message)

---

### IT-2: Positional message argument

**Goal:** Verify the first non-flag argv token is captured as the message and forwarded to Claude Code as a quoted argument.
**Command:** `claude_runner "Fix the null pointer in user_service.rs" --dry-run`
**Expected Output:**
```
claude --max-tokens 200000 "Fix the null pointer in user_service.rs"
```
**Verification:**
- Exit code is 0
- stdout contains the full message string in quotes
- `--max-tokens 200000` present (default injection confirmed)
**Pass Criteria:** Exit 0; positional message forwarded verbatim; default token limit applied
**Source:** [`.run` command](../../commands.md#command--1-run); [message:: parameter](../../params.md#parameter--1-message)

---

### IT-3: `-m` short form for message

**Goal:** Verify the `-m` short flag is an alias for `--message` and produces identical output to the positional form.
**Command:** `claude_runner -m "Fix the null pointer in user_service.rs" --dry-run`
**Expected Output:**
```
claude --max-tokens 200000 "Fix the null pointer in user_service.rs"
```
**Verification:**
- Exit code is 0
- stdout identical to IT-2 output
- `-m` accepted without error
**Pass Criteria:** Exit 0; `-m` alias produces same assembled command as positional argument
**Source:** [message:: parameter](../../params.md#parameter--1-message); [Input group](../../parameter_groups.md#group--1-input)

---

### IT-4: `--message` long form

**Goal:** Verify the `--message` long flag is accepted and produces identical output to the positional and `-m` forms.
**Command:** `claude_runner --message "Fix the null pointer in user_service.rs" --dry-run`
**Expected Output:**
```
claude --max-tokens 200000 "Fix the null pointer in user_service.rs"
```
**Verification:**
- Exit code is 0
- stdout identical to IT-2 and IT-3 output
- `--message` accepted without error
**Pass Criteria:** Exit 0; `--message` long form produces same assembled command as positional and short forms
**Source:** [message:: parameter](../../params.md#parameter--1-message)

---

### IT-5: `--dir` sets working directory env var

**Goal:** Verify `--dir` produces a `CLAUDE_DIR` environment variable line in the dry-run output, preceding the claude command.
**Command:** `claude_runner "task" --dir /workspace --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/workspace
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_DIR=/workspace` as a separate leading line
- `CLAUDE_DIR` line appears before the `claude` invocation line
- `--max-tokens 200000` present
**Pass Criteria:** Exit 0; `CLAUDE_DIR` env var set; correct ordering in output
**Source:** [dir:: parameter](../../params.md#parameter--2-dir); [Environment group](../../parameter_groups.md#group--2-environment)

---

### IT-6: `-d` short form for `--dir`

**Goal:** Verify `-d` is an alias for `--dir` and produces identical output.
**Command:** `claude_runner "task" -d /workspace --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/workspace
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- stdout identical to IT-5 output
- `-d` accepted without error
**Pass Criteria:** Exit 0; `-d` alias produces same `CLAUDE_DIR` env var as `--dir`
**Source:** [dir:: parameter](../../params.md#parameter--2-dir)

---

### IT-7: `--continue` passes `--continue` to claude

**Goal:** Verify `--continue` appears in the assembled claude argv, enabling Claude Code to resume the most recent conversation in the session directory.
**Command:** `claude_runner "Next step" --continue --dry-run`
**Expected Output:**
```
claude --continue --max-tokens 200000 "Next step"
```
**Verification:**
- Exit code is 0
- `--continue` present in claude argv (before `--max-tokens`)
- `--max-tokens 200000` present
**Pass Criteria:** Exit 0; `--continue` forwarded verbatim to claude argv
**Source:** [continue:: parameter](../../params.md#parameter--3-continue); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### IT-8: `--max-tokens` passes integer value to claude

**Goal:** Verify an explicit `--max-tokens` value overrides the default (200000) and is forwarded directly to claude.
**Command:** `claude_runner "task" --max-tokens 50000 --dry-run`
**Expected Output:**
```
claude --max-tokens 50000 "task"
```
**Verification:**
- Exit code is 0
- `--max-tokens 50000` present (not 200000)
- No double `--max-tokens` injection
**Pass Criteria:** Exit 0; explicit token limit replaces default; single `--max-tokens` in output
**Source:** [max_tokens:: parameter](../../params.md#parameter--5-max_tokens); [Resource Control group](../../parameter_groups.md#group--4-resource-control)

---

### IT-9: `--skip-permissions` → `--dangerously-skip-permissions`

**Goal:** Verify the adapter translates `--skip-permissions` to `--dangerously-skip-permissions` in the assembled claude argv, confirming the flag rename abstraction.
**Command:** `claude_runner "task" --skip-permissions --dry-run`
**Expected Output:**
```
claude --dangerously-skip-permissions --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- `--dangerously-skip-permissions` present in stdout (the translated form)
- `--skip-permissions` NOT present in stdout (raw form not forwarded)
- `--max-tokens 200000` present
**Pass Criteria:** Exit 0; flag translated correctly; dangerous form forwarded, safe alias not leaked
**Source:** [skip_permissions:: parameter](../../params.md#parameter--4-skip_permissions); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### IT-10: `--dry-run` prints command, exits 0, no execution

**Goal:** Verify `--dry-run` causes the adapter to print the assembled command representation to stdout and exit 0 without spawning a claude process.
**Command:** `claude_runner "task" --dry-run`
**Expected Output:**
```
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- stdout contains the assembled command string
- No claude process spawned (verifiable by absence of Claude Code's characteristic output)
- `--dry-run` does not appear in the printed command (it is an adapter-level flag only)
**Pass Criteria:** Exit 0; command printed without execution; `--dry-run` excluded from assembled claude argv
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### IT-11: `--session-dir` sets `CLAUDE_SESSION_DIR` env var

**Goal:** Verify `--session-dir` produces a `CLAUDE_SESSION_DIR` environment variable line in the dry-run output.
**Command:** `claude_runner "task" --session-dir /home/user/.sessions/proj --dry-run`
**Expected Output:**
```
CLAUDE_SESSION_DIR=/home/user/.sessions/proj
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- stdout contains `CLAUDE_SESSION_DIR=/home/user/.sessions/proj` as a leading line
- `CLAUDE_SESSION_DIR` appears before the `claude` invocation line
**Pass Criteria:** Exit 0; `CLAUDE_SESSION_DIR` env var set with exact path
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir); [Environment group](../../parameter_groups.md#group--2-environment)

---

### IT-12: `--model` passes `--model NAME` to claude

**Goal:** Verify `--model` produces a `--model <name>` flag pair in the assembled claude argv.
**Command:** `claude_runner "task" --model claude-opus-4-6 --dry-run`
**Expected Output:**
```
claude --model claude-opus-4-6 --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- `--model claude-opus-4-6` present in stdout (flag and value together)
- `--model` appears before `--max-tokens` in the argv ordering
**Pass Criteria:** Exit 0; model flag forwarded as a two-token pair (`--model <name>`)
**Source:** [model:: parameter](../../params.md#parameter--8-model); [Resource Control group](../../parameter_groups.md#group--4-resource-control)

---

## Command-Specific Behavior

### CSB-1: Duplicate `--dir` → last value wins

**Goal:** Verify that when `--dir` is repeated, the adapter applies last-wins semantics: only the final value is used for `CLAUDE_DIR`.
**Command:** `claude_runner "task" --dir /first --dir /last --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/last
claude --max-tokens 200000 "task"
```
**Verification:**
- Exit code is 0
- `CLAUDE_DIR=/last` present in output
- `CLAUDE_DIR=/first` absent from output
- No error or warning about duplication
**Pass Criteria:** Exit 0; last-wins semantics applied silently; first value discarded
**Source:** [dir:: parameter](../../params.md#parameter--2-dir); EC-7 in `../param/dir.md`

---

### CSB-2: Positional + `--message` simultaneously → conflict error

**Goal:** Verify that supplying a message via both positional argument and `--message` is rejected as a conflict, preventing ambiguous message sourcing.
**Command:** `claude_runner "positional message" --message "explicit message"`
**Expected Output (stderr):**
```
Error: --message conflicts with a previously set message (positional or duplicate --message)
```
**Verification:**
- Exit code is 1
- stderr contains the conflict error text
- stdout is empty
- No claude process started
**Pass Criteria:** Exit 1; conflict detected before execution; unambiguous error on stderr
**Source:** [Input group](../../parameter_groups.md#group--1-input); [message:: parameter](../../params.md#parameter--1-message); CC-1 in `../param_group/input.md`

---

### CSB-3: Claude stderr forwarded to stderr (not mixed with stdout)

**Goal:** Verify the adapter preserves stream separation: output Claude Code writes to stderr reaches the caller's stderr, not stdout, so log capture pipelines remain clean.
**Setup:** Requires `claude` CLI installed and authenticated. Run with a prompt known to produce diagnostic output on stderr.
**Command:** `claude_runner "task" 2>/tmp/runner_stderr 1>/tmp/runner_stdout`
**Verification:**
- Any stderr output from claude appears in `/tmp/runner_stderr`
- `/tmp/runner_stdout` does not contain claude's stderr content
- Stream mixing does not occur
**Pass Criteria:** Claude stderr → caller stderr; stdout stays clean of stderr content
**Source:** [`.run` command](../../commands.md#command--1-run)

---

### CSB-4: Non-zero Claude exit code → `claude_runner` exits non-zero

**Goal:** Verify the adapter propagates Claude Code's non-zero exit code so callers can detect failure; `claude_runner` must not swallow the exit code.
**Setup:** Requires `claude` CLI installed. Trigger a Claude runtime error (e.g., authentication failure, network timeout).
**Command:** `claude_runner "task"  # with claude failing internally`
**Verification:**
- `echo $?` after the command returns a non-zero value
- Exit code matches or wraps Claude Code's exit code
- `claude_runner` does not exit 0 when the underlying claude process fails
**Pass Criteria:** Non-zero exit propagated; `claude_runner` exit reflects claude subprocess outcome
**Source:** [`.run` command](../../commands.md#command--1-run); Exit Codes: 0 | 1 | 2

---

## Real-World Scenarios

### RWS-1: Developer runs quick fix on current project

**Goal:** Verify the typical developer workflow: specify a task message and a working directory, then preview the assembled command before executing.
**Command (preview):** `claude_runner "Fix the null pointer in user_service.rs" --dir ~/projects/myapp --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/home/user/projects/myapp
claude --max-tokens 200000 "Fix the null pointer in user_service.rs"
```
**Verification:**
- Exit code is 0
- `CLAUDE_DIR` set to the expanded home path
- Message and working dir both present
- `--max-tokens 200000` present (default)
**Pass Criteria:** Exit 0; dir and message assembled correctly; tilde expanded in CLAUDE_DIR path
**Source:** [`.run` command](../../commands.md#command--1-run); [dir:: parameter](../../params.md#parameter--2-dir)

---

### RWS-2: CI pipeline with full parameter set

**Goal:** Verify a fully-specified CI invocation (model, max-tokens, skip-permissions, dir) assembles a correct command with all parameters present and none conflicting.
**Command:** `claude_runner "Review changes and add tests" --dir /workspace --model claude-opus-4-6 --max-tokens 100000 --skip-permissions --dry-run`
**Expected Output:**
```
CLAUDE_DIR=/workspace
claude --dangerously-skip-permissions --model claude-opus-4-6 --max-tokens 100000 "Review changes and add tests"
```
**Verification:**
- Exit code is 0
- `CLAUDE_DIR=/workspace` present as env var line
- `--dangerously-skip-permissions` present (translated from `--skip-permissions`)
- `--model claude-opus-4-6` present
- `--max-tokens 100000` present (explicit, overrides default 200000)
- Message present
**Pass Criteria:** Exit 0; all five parameters active simultaneously with no conflict or ordering error
**Source:** [`.run` command](../../commands.md#command--1-run); [parameter_groups.md](../../parameter_groups.md)

---

### RWS-3: Preview invocation with `--dry-run`, then execute for real

**Goal:** Verify the two-step workflow: dry-run to inspect the command, then the same invocation without `--dry-run` to actually execute it.
**Step 1 (preview):**
```bash
claude_runner "Refactor the auth module" --dir ~/projects/app --continue --dry-run
```
**Step 1 Expected Output:**
```
CLAUDE_DIR=/home/user/projects/app
claude --continue --max-tokens 200000 "Refactor the auth module"
```
**Step 2 (execute — requires `claude` CLI):**
```bash
claude_runner "Refactor the auth module" --dir ~/projects/app --continue
```
**Verification:**
- Step 1: Exit 0; dry-run output matches expected; no claude process spawned
- Step 2: Exit 0; claude process starts with the exact command from step 1's preview
- The two invocations are identical except for the presence/absence of `--dry-run`
**Pass Criteria:** Dry-run output matches the actual command executed; `--dry-run` is the only difference between preview and execution
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [Behavior Flags group](../../parameter_groups.md#group--3-behavior-flags)

---

### RWS-4: Multi-session iterative workflow with `--continue`

**Goal:** Verify the iterative session pattern: first turn establishes a session in a directory, subsequent turns use `--continue` + `--session-dir` to resume it, producing correct env vars and flags at each step.
**Step 1 (start session):**
```bash
claude_runner "Start: analyze the codebase" --session-dir ~/.sessions/proj --dry-run
```
**Step 1 Expected Output:**
```
CLAUDE_SESSION_DIR=/home/user/.sessions/proj
claude --max-tokens 200000 "Start: analyze the codebase"
```
**Step 2 (resume session):**
```bash
claude_runner "Continue: fix the bug found in step 1" --session-dir ~/.sessions/proj --continue --dry-run
```
**Step 2 Expected Output:**
```
CLAUDE_SESSION_DIR=/home/user/.sessions/proj
claude --continue --max-tokens 200000 "Continue: fix the bug found in step 1"
```
**Verification:**
- Both steps: Exit code 0
- Step 1: `CLAUDE_SESSION_DIR` set; no `--continue` in claude argv
- Step 2: `CLAUDE_SESSION_DIR` set to same path; `--continue` present in claude argv
- `--max-tokens 200000` present in both
**Pass Criteria:** Exit 0 for both steps; session dir consistent across turns; `--continue` only in step 2 argv
**Source:** [session_dir:: parameter](../../params.md#parameter--7-session_dir); [continue:: parameter](../../params.md#parameter--3-continue); CC-3 in `../param_group/environment.md`
