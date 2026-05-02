# Invariant: Error Propagation

### Scope

- **Purpose**: Guarantee that all failure modes in the build and execution pipeline surface as structured errors to the caller.
- **Responsibility**: Documents the error propagation contract — what can fail, how failures are represented, and what is never silently ignored.
- **In Scope**: Cargo invocation failures; compilation errors; workspace creation failures; manifest generation failures.
- **Out of Scope**: Errors emitted by the target program itself (captured as stderr content and non-zero exit status, not as Rust error values).

### Statement

Every failure in the build and execution pipeline is surfaced to the caller as a structured error value — no failure is silently ignored, swallowed, or converted into empty output.

### Rationale

Silent failures are the most damaging class of defect. If a compilation error were silently converted into empty output that happens to match an expected string, a test would pass precisely when it should catch a regression. Explicit, typed error values force the caller to handle each failure mode intentionally.

### Enforcement

All fallible operations in the runner use the workspace error type. Build failures include Cargo's diagnostic output in the error value, providing actionable context. A non-zero exit code from the target program is not an error — it is a distinct result variant in the captured output type. This distinction allows callers to assert on expected failures without treating them as infrastructure errors.

### Violations

A caller that discards the result value opts out of error propagation at their own choice. This is the caller's prerogative. The invariant guarantees that errors are surfaced — not that they are handled.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/api/002_runner_api.md` | Runner API return types that carry structured errors |
| doc | `docs/feature/001_script_execution.md` | Script execution pipeline where errors originate |
| doc | `docs/feature/002_output_capture.md` | Non-zero exit status as a distinct captured output variant |
| test | `tests/inc/runner_test.rs` | Integration tests verifying non-zero exit and diagnostic output on failures |
