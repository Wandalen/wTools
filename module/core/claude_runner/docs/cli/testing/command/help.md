# Command: `.help`

- **Purpose:** Print usage information and exit
- **Parameters:** 0
- **Exit Codes:** 0 (success) | 1 (unknown flag before `--help`)

## Test Case Index

| ID | Test Name | Category | Priority |
|----|-----------|----------|----------|
| IT-1 | `--help` flag prints usage to stdout | Integration Tests | P0 |
| IT-2 | `-h` short form prints usage | Integration Tests | P0 |
| IT-3 | `--help` exits with code 0 | Integration Tests | P0 |
| IT-4 | Output contains `USAGE:` section header | Integration Tests | P0 |
| IT-5 | Output contains `--message` flag | Integration Tests | P0 |
| IT-6 | Output contains `--dir` flag | Integration Tests | P0 |
| IT-7 | Output contains `--continue` flag | Integration Tests | P0 |
| IT-8 | Output contains `--dry-run` flag | Integration Tests | P0 |
| IT-9 | Output contains `--skip-permissions` flag | Integration Tests | P0 |
| IT-10 | Output contains `--model` flag | Integration Tests | P1 |
| CSB-1 | Unknown flag before `--help` → exit 1 (error, not help) | Command-Specific Behavior | P0 |

## Test Coverage Summary

| Category | Count | Coverage |
|----------|-------|----------|
| Integration Tests | 10 | 100% |
| Command-Specific Behavior | 1 | 100% |
| **Total** | **11** | **100%** |

**Cross-References:**
- Help flag parsing → `../param/` (no dedicated param file — help is CLI-level)
- Unknown-arg sequential behavior → `../param_group/behavior_flags.md`

---

## Integration Tests

### IT-1: `--help` flag prints usage to stdout

**Goal:** Verify the `--help` flag triggers the help routine, which prints usage information to stdout and then exits without attempting to invoke Claude Code.
**Command:** `claude_runner --help`
**Expected Output:** Usage text printed to stdout (contains at minimum a `USAGE:` header and flag listing)
**Verification:**
- Exit code is 0
- stdout is non-empty
- stderr is empty
- No claude process started
**Pass Criteria:** Exit 0; usage text on stdout; no execution of Claude Code
**Source:** [`.help` command](../../commands.md#command--2-help)

---

### IT-2: `-h` short form prints usage

**Goal:** Verify the `-h` short flag is an alias for `--help` and produces identical output.
**Command:** `claude_runner -h`
**Expected Output:** Same usage text as IT-1
**Verification:**
- Exit code is 0
- stdout output is identical to `claude_runner --help`
- `-h` accepted without error
**Pass Criteria:** Exit 0; `-h` alias produces same help output as `--help`
**Source:** [`.help` command](../../commands.md#command--2-help)

---

### IT-3: `--help` exits with code 0

**Goal:** Verify the help command terminates with exit code 0, signaling successful completion (not an error).
**Command:** `claude_runner --help; echo "exit_code=$?"`
**Expected Output:**
```
<usage text>
exit_code=0
```
**Verification:**
- `echo "exit_code=$?"` prints `exit_code=0`
- Shell `$?` is 0 immediately after the command
**Pass Criteria:** Exit 0; help is a successful operation, not an error path
**Source:** [`.help` command](../../commands.md#command--2-help)

---

### IT-4: Output contains `USAGE:` section header

**Goal:** Verify the help output includes a `USAGE:` section header, establishing the standard CLI help format that users and tooling can rely on.
**Command:** `claude_runner --help | grep -c "USAGE:"`
**Expected Output:** `1` (exactly one match)
**Verification:**
- Exit code of grep is 0 (match found)
- Output is `1` or more
- `USAGE:` appears in the help text
**Pass Criteria:** `USAGE:` header present; custom `print_help()` produces standard format
**Source:** [`.help` command](../../commands.md#command--2-help)

---

### IT-5: Output contains `--message` flag

**Goal:** Verify the help text lists `--message` so users know how to supply a task prompt.
**Command:** `claude_runner --help | grep -c -- "--message"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--message` present in help text
**Pass Criteria:** `--message` flag documented in help output
**Source:** [message:: parameter](../../params.md#parameter--1-message); [`.help` command](../../commands.md#command--2-help)

---

### IT-6: Output contains `--dir` flag

**Goal:** Verify the help text lists `--dir` so users know how to set the working directory.
**Command:** `claude_runner --help | grep -c -- "--dir"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--dir` present in help text
**Pass Criteria:** `--dir` flag documented in help output
**Source:** [dir:: parameter](../../params.md#parameter--2-dir); [`.help` command](../../commands.md#command--2-help)

---

### IT-7: Output contains `--continue` flag

**Goal:** Verify the help text lists `--continue` so users know how to resume a prior conversation.
**Command:** `claude_runner --help | grep -c -- "--continue"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--continue` present in help text
**Pass Criteria:** `--continue` flag documented in help output
**Source:** [continue:: parameter](../../params.md#parameter--3-continue); [`.help` command](../../commands.md#command--2-help)

---

### IT-8: Output contains `--dry-run` flag

**Goal:** Verify the help text lists `--dry-run` so users know how to preview the assembled command without executing it.
**Command:** `claude_runner --help | grep -c -- "--dry-run"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--dry-run` present in help text
**Pass Criteria:** `--dry-run` flag documented in help output
**Source:** [dry:: parameter](../../params.md#parameter--6-dry); [`.help` command](../../commands.md#command--2-help)

---

### IT-9: Output contains `--skip-permissions` flag

**Goal:** Verify the help text lists `--skip-permissions` (the user-facing alias, not the internal `--dangerously-skip-permissions` form) so users know the safe flag name to use.
**Command:** `claude_runner --help | grep -c -- "--skip-permissions"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--skip-permissions` present (user-facing name)
- Help does NOT need to mention `--dangerously-skip-permissions` (internal translation detail)
**Pass Criteria:** `--skip-permissions` documented; adapter abstraction preserved in user-facing help
**Source:** [skip_permissions:: parameter](../../params.md#parameter--4-skip_permissions); [`.help` command](../../commands.md#command--2-help)

---

### IT-10: Output contains `--model` flag

**Goal:** Verify the help text lists `--model` so users know they can specify a Claude model name.
**Command:** `claude_runner --help | grep -c -- "--model"`
**Expected Output:** `1` or more
**Verification:**
- Exit code of grep is 0 (match found)
- `--model` present in help text
**Pass Criteria:** `--model` flag documented in help output
**Source:** [model:: parameter](../../params.md#parameter--8-model); [`.help` command](../../commands.md#command--2-help)

---

## Command-Specific Behavior

### CSB-1: Unknown flag before `--help` → exit 1, not help

**Goal:** Verify that an unrecognized flag appearing before `--help` in argv triggers a parse error (exit 1) rather than printing help, demonstrating that argument parsing is left-to-right and the unknown flag is encountered first.
**Command:** `claude_runner --unknown-flag --help`
**Expected Output (stderr):**
```
Error: unknown argument: --unknown-flag
```
**Verification:**
- Exit code is 1
- stderr contains an error message referencing the unknown flag
- stdout does NOT contain help text (help routine not reached)
- `--help` is not processed when parsing fails on an earlier token
**Pass Criteria:** Exit 1; parse error on unknown flag precedes help processing; help text suppressed
**Source:** [`.help` command](../../commands.md#command--2-help); [`.run` command](../../commands.md#command--1-run)
