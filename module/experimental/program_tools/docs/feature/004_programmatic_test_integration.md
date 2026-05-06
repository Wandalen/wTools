# Feature: Programmatic Test Integration

### Scope

- **Purpose**: Enable Rust test functions to execute scripts with a single expression, without boilerplate workspace or process management.
- **Responsibility**: Documents the test integration convenience layer — single-expression invocation forms, failure semantics, inline assertion patterns, and fixture crate conventions.
- **In Scope**: Single-expression invocation for file, source string, and project targets; panic-on-failure semantics; inline assertion chains; fixture crate pattern.
- **Out of Scope**: Output capture internals (→ `feature/002`); artifact cache management (→ `feature/003`); CLI invocation (→ `api/004`).

### Design

The primary use case for `program_tools` is Rust test code. Test functions need to compile and run small programs — usually snippets or fixture crates — and assert on their output, without constructing a full plan. The test integration layer provides convenience entry points optimized for this pattern.

**Single-expression invocation**: Each convenience entry point accepts the minimum information required for a complete run. For a source file on disk, the caller passes the file path; for inline source code, the caller passes a string; for a project, the caller passes the directory path. The runner internally constructs the full Plan, executes it, and returns the captured output.

**Failure semantics**: The convenience forms return `Result<CapturedOutput>`. Infrastructure failures (file not found, Cargo binary not available) are propagated as `Err`; callers use `.expect("run failed")` to convert them into test panics. Non-zero exit status is not an infrastructure failure — it is carried in the returned `CapturedOutput` as `exit_status != 0`, enabling callers to assert on expected compilation failures or non-zero program exits using the predicate and assertion methods.

**Inline assertion chains**: The convenience forms return the captured output value, enabling single-statement test patterns: one expression to execute, one chained call to assert. This produces compact, readable test code where each test function covers exactly one execution scenario without intermediate variable binding.

**Fixture crate pattern**: For integration tests requiring complex target programs, callers place fixture binary crates under `tests/asset/` and invoke them by project directory path. The runner compiles and runs the fixture crate as a complete Cargo project. Fixture crates model specific output formats, error conditions, or behaviors that are impractical to express in inline source strings.

**Shared artifact cache for suites**: Test suites executing many fixture crate invocations should configure a shared persistent target directory so that compilation artifacts are reused across test runs. A stable location under the crate's `target/` prevents redundant rebuilds between test suite executions.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/api/002_runner_api.md` | Runner convenience constructors for single-expression invocation |
| doc | `docs/api/003_output_api.md` | Output type with assertion methods for inline chaining |
| doc | `docs/feature/001_script_execution.md` | Script execution underlying the convenience layer |
| doc | `docs/feature/002_output_capture.md` | Output capture enabling test assertions |
