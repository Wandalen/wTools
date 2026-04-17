# Invariant: Result<Report, Report> Contract

### Statement

`run()` returns `Result<Report, Report>`. Both the `Ok` and `Err` variants carry a fully populated `Report` containing the command, working directory, stdout, stderr, and error field. This ensures callers never lose diagnostic context on failure: `Err(report)` contains the same fields as `Ok(report)`, with `report.error` set to the failure cause. Code that matches on `Result` can apply identical logging or display logic regardless of branch.

### Violation Consequence

If a failure path returned only `Error` (not `Report`), callers could not retrieve stdout/stderr context from the failed invocation, making error diagnosis harder and requiring separate state to reconstruct what was executed.

### Measurement

`grep -r "Result<Report," src/` must return only `Result<Report, Report>` occurrences. No `Result<Report, Error>` signatures permitted.
