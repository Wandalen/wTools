# Invariant: Result<Report, Report> Contract

### Scope

- **Purpose**: Guarantee that callers are never left without diagnostic context when a subprocess invocation fails.
- **Responsibility**: Enforces that both the success and failure branches carry a fully populated report.
- **In Scope**: `run()` and `run_with_shell()` return type; field population ordering in `process.rs` before the success/failure branch.
- **Out of Scope**: `Report` field definitions (→ `api/002`); shell selection logic (→ `invariant/002`).

### Invariant Statement

`run()` returns a result where both the success and failure branches carry a fully populated report containing the command, working directory, stdout, stderr, and error field. This ensures callers never lose diagnostic context on failure — the failure branch contains the same fields as the success branch, with the error field set to the failure cause. Code that handles the result can apply identical logging or display logic regardless of branch.

### Enforcement Mechanism

In `process.rs`, `Report` is constructed and populated with `command`, `current_path`, stdout, and stderr before the success/failure branch is evaluated. Every error return path uses the same populated report variable. There is no code path that returns a bare error without a fully populated report.

Verification command:

```bash
grep -n "Result<Report," src/process.rs
# Must show only: Result<Report, Report>
# No Result<Report, Error> or Result<Report, _> with different type
```

### Violation Consequences

If a failure path returned only a bare error (not a full report), callers could not retrieve stdout/stderr context from the failed invocation. Error diagnosis would require separate state to reconstruct what was executed. The uniform return type — a report on both branches — allows a single handler to process both outcomes with identical display code.

### Example

```rust
use process_tools::process::Run;

// Both branches give full context — same display logic works for both
let report = match Run::former()
  .bin_path( "ls" )
  .args( vec![ "/nonexistent".into() ] )
  .current_path( "." )
  .run()
{
  Ok( r ) | Err( r ) => r,
};

// report.command, report.out, report.err are always populated
println!( "{}", report );
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [src/process.rs](../../src/process.rs) | Enforces populate-before-branch ordering for `Report` |
| doc | [api/002_report_api.md](../api/002_report_api.md) | `Report` type that both branches carry |
| doc | [api/001_run_api.md](../api/001_run_api.md) | `run()` and `run_with_shell()` return type |
| doc | [feature/002_output_capture.md](../feature/002_output_capture.md) | Design rationale for always-populated `Report` |
