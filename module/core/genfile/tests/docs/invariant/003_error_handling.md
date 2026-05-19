# Invariant Spec: Error Handling

### Scope

- **Element:** `invariant/003_error_handling`
- **Source:** `docs/invariant/003_error_handling.md`
- **Prefix:** `IN-`
- **Minimum cases:** 2

### Case Index

| ID | Name | Category | Status |
|----|------|----------|--------|
| IN-01 | error_message_uses_bracketed_format | nominal | ✅ |
| IN-02 | failed_command_exits_nonzero | nominal | ✅ |
| IN-03 | path_traversal_sequence_rejected | security | ✅ |

---

### IN-01: error messages use [ERROR][CONTEXT]: format

- **Given:** Any command that produces an error is run (e.g., load a nonexistent file)
- **When:** The error output is examined
- **Then:** Error message matches `[ERROR] [CONTEXT]: message` pattern; no bare unformatted errors
- **Tests:** `tests/invariant_test.rs`

### IN-02: failed command exits with nonzero exit code

- **Given:** A command that will fail is run (e.g., `.archive.load path::/nonexistent.json`)
- **When:** Exit code is captured
- **Then:** Exit code is 1 (runtime error) or 2 (usage error); never 0 on failure
- **Behavioral Divergence:** Invalid path → exit 1; success → exit 0
- **Tests:** `tests/invariant_test.rs`, `tests/repl_exit_code_bug_test.rs`

### IN-03: path traversal sequences are rejected

- **Given:** An archive is loaded
- **When:** `.materialize destination::/tmp/../../etc` is run
- **Then:** Exit code 1; error output indicates path validation failure; no files written
- **Tests:** `tests/materialization_test.rs`
