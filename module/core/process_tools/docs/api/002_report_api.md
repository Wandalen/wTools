# API: Report

### Scope

- **Purpose**: Define the `Report` struct that every subprocess invocation returns on both success and failure branches.
- **Responsibility**: Documents all public fields, trait implementations (`Display`, `Clone`, `Default`), and population guarantees.
- **In Scope**: All `Report` fields, `Display::fmt` output format, `Clone::clone` stringification behavior, and `Default::default` values.
- **Out of Scope**: How `Report` is produced (→ `api/001`); exit status construction (→ `api/003`).

### Abstract

`Report` is the universal return type for all subprocess invocations. It bundles the command string, working directory, stdout, stderr, and result into one struct so callers never lose context on either success or failure. It implements `Display` for CLI-friendly human-readable output and `Clone` via manual implementation (because `Error` is not `Clone`).

### Operations

| Symbol | Kind | Notes |
|--------|------|-------|
| `Report::command` | field | Command as executed (binary + args joined) |
| `Report::current_path` | field | Working directory used for the invocation |
| `Report::out` | field | Captured stdout (empty string if none) |
| `Report::err` | field | Captured stderr; empty when `joining_streams = true` |
| `Report::error` | field | Success on zero exit code; error cause on failure |
| `Display::fmt()` | trait impl | Renders command, path, and indented output blocks |
| `Clone::clone()` | trait impl | Stringifies error to preserve message across clone |
| `Default::default()` | trait impl | All fields empty/default; error field starts as success |

### Error Handling

`Report` itself does not fail. All failure information is carried inside the `error` field. Callers inspect the error field to distinguish success from failure — a non-success value means the process exited with an error or could not be started. The `run()` function returns the report wrapped in a failure result when the error field is set. Callers that receive a failed report can safely read all output fields — they are always populated before the error path is taken.

### Compatibility Guarantees

- **Field visibility:** all fields are `pub`. Field names are stable since 0.1.0.
- **`Clone`:** available but error is stringified — the clone loses the original error type, preserving only the message. Clones should not be used for programmatic error-type inspection.
- **`Display` format:** stable. The `>` prefix on the command line and `@` prefix on the path line will not change without a semver bump.
- **`joining_streams` effect:** when `joining_streams = true`, `report.err` is always empty; stdout and stderr are both in `report.out`.

### Example

```rust
use process_tools::process::Run;

let report = Run::former()
  .bin_path( "echo" )
  .args( vec![ "world".into() ] )
  .current_path( "." )
  .run()
  .unwrap();

println!( "stdout: {}", report.out );
println!( "Display output:\n{}", report );
// > echo world
//   @ .
//   world
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | `Report` struct, `Display` impl, and manual `Clone` impl |
| doc | [feature/002_output_capture.md](../feature/002_output_capture.md) | Design rationale for why `Report` is always fully populated |
| doc | [api/001_run_api.md](001_run_api.md) | `RunFormer::run()` produces `Report` values |
| doc | [invariant/001_result_contract.md](../invariant/001_result_contract.md) | Guarantees `Report` fields are populated on both `Ok` and `Err` branches |
