# API: Output API

### Scope

- **Purpose**: Provide structured access to captured execution output and assertion methods for test verification.
- **Responsibility**: Documents the output capture type — its fields, assertion methods, predicate methods, and display semantics.
- **In Scope**: Captured output fields (exit status, stdout, stderr); assertion methods; predicate methods; failure message format.
- **Out of Scope**: How output is captured during execution (→ `feature/002`); runner execution (→ `api/002`); CLI output formatting (→ `api/004`).

### Abstract

The captured output type is a plain data container holding the complete result of a single run: the exit status code, the complete standard output, and the complete standard error. Assertion methods on the type enable single-expression test verification — execute a plan and chain assertions directly on the result without intermediate variables.

### Operations

**Fields**:
- `exit_status` — the integer exit code of the completed child process. Zero conventionally indicates success; any non-zero value indicates the process terminated with an error or explicit non-zero return.
- `stdout` — the complete standard output produced by the run, captured as a byte sequence. Decoded to UTF-8 for comparison in assertion and predicate methods; raw bytes available for binary output.
- `stderr` — the complete standard error produced by the run, captured as a byte sequence. Decoded to UTF-8 for comparison in assertion and predicate methods.

**Assertion methods** (panic on failure with a descriptive message showing expected and actual values):
- `assert_exit_ok` — asserts that exit status is zero.
- `assert_stdout_eq(expected)` — asserts that stdout exactly equals the expected string.
- `assert_stderr_eq(expected)` — asserts that stderr exactly equals the expected string.
- `assert_stdout_contains(substring)` — asserts that stdout contains the given substring.
- `assert_stderr_contains(substring)` — asserts that stderr contains the given substring.
- `assert_stdout_empty` — asserts that stdout is empty (zero bytes).
- `assert_stderr_empty` — asserts that stderr is empty (zero bytes).

**Predicate methods** (return bool, never panic):
- `exit_ok` — true if exit status is zero.
- `stdout_eq(expected)` — true if stdout equals the expected string.
- `stderr_eq(expected)` — true if stderr equals the expected string.
- `stdout_contains(substring)` — true if stdout contains the substring.
- `stderr_contains(substring)` — true if stderr contains the substring.

### Error Handling

Assertion methods panic with a descriptive failure message when the assertion does not hold. The message includes the expected value, the actual value, and (for `assert_exit_ok`) the stderr content to aid diagnosis. Predicate methods never panic. No method returns a `Result` — the type is designed for test code where panics are the correct failure mechanism.

### Compatibility Guarantees

Version 0.1.0, marked experimental. The field names and assertion method set are intended to be stable but may gain additional methods. The panic message format is not guaranteed stable.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/output.rs` | CapturedOutput struct and all method implementations |
| test | `tests/inc/runner_test.rs` | Predicate and assertion method tests plus runner integration |
| doc | `docs/api/002_runner_api.md` | Runner execution that produces the captured output value |
| doc | `docs/feature/002_output_capture.md` | Output capture feature that this API exposes |
| doc | `docs/feature/004_programmatic_test_integration.md` | Test patterns using assertion methods directly |
| doc | `docs/invariant/003_output_determinism.md` | Determinism guarantee that assertion methods rely on |
