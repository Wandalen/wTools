# Feature: Output Capture

### Scope

- **Purpose**: Ensure callers always have full diagnostic context after any subprocess invocation, regardless of exit code.
- **Responsibility**: Owns the `Report` struct that carries command, working directory, stdout, stderr, and error result in one value.
- **In Scope**: `Report` field layout, `Display` formatting for CLI output, manual `Clone` implementation, and population-before-branch ordering inside `run()`.
- **Out of Scope**: How the process is spawned (→ `feature/001`); exit code interpretation semantics (→ `feature/004`).

### Status

- **Version introduced:** 0.1.0
- **Stability:** stable
- **Module path:** `process_tools::process::Report`

### Design

`Report` is unconditionally populated before the exit-code check. In `run()`, `command`, `current_path`, stdout, and stderr are stored into the report struct before the success/failure branch. The divergence happens only at the very end — after all fields are filled. This means the failure path never carries a partially-filled report, so callers can apply identical display or logging logic regardless of branch.

The `Display` impl prefixes the command with `>` and the working directory with `@`, then indents stdout/stderr with two spaces (replacing `\n` with `\n  `). Whitespace-only output blocks are suppressed. This format is designed for inline subprocess tracing in CLI tool output.

`Clone` cannot be derived because the error field type is not cloneable. The manual impl preserves the failure message across the clone boundary by converting the error to its string representation, accepting the loss of the original error type.

### Example

```rust
use process_tools::process::Run;

// Both Ok and Err carry a fully populated Report
let result = Run::former()
  .bin_path( "cat" )
  .args( vec![ "/nonexistent".into() ] )
  .current_path( "." )
  .run();

let report = result.unwrap_err();
assert!( !report.command.is_empty() );
assert!( report.error.is_err() );

// Display renders cleanly for CLI output
println!( "{}", report );
// > cat /nonexistent
//   @ .
//   cat: /nonexistent: No such file or directory
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | `Report` struct, `Display` impl, `Clone` impl |
| test | [tests/inc/process_run.rs](../../tests/inc/process_run.rs) | `Report` field population and display tests |
| doc | [api/002_report_api.md](../api/002_report_api.md) | Defines fields and method surface of `Report` |
| doc | [invariant/001_result_contract.md](../invariant/001_result_contract.md) | Guarantees `Report` is always fully populated on both branches |
| doc | [feature/001_process_execution.md](001_process_execution.md) | Execution layer that produces `Report` values |
