# Feature: Output Capture and Comparison

### Scope

- **Purpose**: Enable callers to inspect and assert on the standard output and standard error produced by a script execution.
- **Responsibility**: Documents the output capture feature — capture modes, the output value structure, comparison semantics, and assertion integration with test code.
- **In Scope**: Separate stdout and stderr capture; exit status capture; exact equality comparison; substring containment comparison; emptiness assertion.
- **Out of Scope**: Running the script (→ `feature/001`); test convenience forms (→ `feature/004`); CLI output forwarding (→ `api/004`).

### Design

Output capture collects the complete content of the child process's standard output and standard error into separate in-memory buffers. This structured capture is what distinguishes the runner from a simple subprocess invocation — callers receive a typed value with fields and assertion methods, not raw bytes they must interpret themselves.

**Capture model**: Standard output and standard error are captured independently through the subprocess pipe mechanism. The child process runs to completion before any content is returned to the caller. The full content of each stream is available after the run returns. No incremental or streaming access is provided.

**Comparison modes**:

*Exact equality* — the captured content matches an expected string byte-for-byte after UTF-8 decoding. Used when the output format is fully controlled and deterministic.

*Substring containment* — the captured content contains a given substring. Used when output includes variable sections (timestamps, process IDs, generated values) alongside stable expected text.

*Emptiness* — the captured content is zero bytes. Used to assert that a run produces no output noise.

**Exit status integration**: Exit status is part of the captured output value — it is not a separate result dimension. A run that exits with non-zero status still produces a captured output value with the exit status field set accordingly. This allows callers to assert on expected failures: checking that a compilation error produces a non-zero exit and that the error message contains the expected diagnostic text, all in one captured value.

**Test-oriented design**: The assertion methods on the captured output value are designed for direct use in Rust test functions. They panic with descriptive messages rather than returning results, following the workspace test convention that panics are the correct failure mechanism in test code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/output.rs` | CapturedOutput struct definition and method implementations |
| test | `tests/inc/runner_test.rs` | Predicate tests and output assertion integration |
| doc | `docs/api/003_output_api.md` | Output type fields and assertion method signatures |
| doc | `docs/feature/001_script_execution.md` | Script execution that produces the captured output value |
| doc | `docs/feature/004_programmatic_test_integration.md` | Test patterns combining execution and output assertion |
| doc | `docs/invariant/003_output_determinism.md` | Determinism guarantee for captured output |
| doc | `docs/invariant/004_error_propagation.md` | Non-zero exit status as a distinct output variant |
