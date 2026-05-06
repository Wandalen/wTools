# Invariant: Performance Overhead Constraint

### Scope

- **Purpose**: Ensure benchkit's own operation does not meaningfully distort the measurements it produces.
- **Responsibility**: States the measurable overhead thresholds for measurement, data generation, and report generation.
- **In Scope**: Timing overhead of the measurement loop; cost of data generation relative to measured operation; duration of report generation.
- **Out of Scope**: Performance of the user's benchmarked code (benchkit does not govern that); statistical analysis accuracy (→ invariant/006).

### Invariant Statement

Three thresholds must hold simultaneously:
1. The measurement loop overhead must be less than 1% of the measured operation's own execution time, for any operation whose mean duration exceeds 1 millisecond.
2. Test data generation must complete before the timed region begins; generation cost must not appear in timing results.
3. Report generation — including markdown rendering and file update — must complete within 10 seconds for a typical benchmark suite of up to 20 benchmarks.

### Enforcement Mechanism

The timing loop records wall-clock duration of the user closure only — setup and teardown (including data generation) execute outside the timed region. The separation is enforced structurally: data is prepared before the closure is passed to the timing function.

Report generation is single-threaded and synchronous; its duration can be observed by the caller and instrumented in integration tests. The 10-second threshold is verified by benchkit's own performance tests against representative suite sizes.

Overhead below the 1% threshold is validated by comparing measured durations of a no-op closure against non-trivial closures; if the no-op cost exceeds 1% of a 1ms operation in CI, the measurement loop implementation must be optimised before release.

### Violation Consequences

Measurement loop overhead exceeding 1% causes benchmark comparisons to be misleading: two operations that differ by 5% in real performance may appear equivalent or reversed when the harness adds 3% noise to both. Optimization decisions made from such data drive the wrong changes.

Report generation exceeding 10 seconds stalls the developer's benchmark workflow and discourages running benchmarks frequently — the opposite of the living-documentation goal.

### Cross-References

| Type   | File                                       | Responsibility                                         |
|--------|--------------------------------------------|--------------------------------------------------------|
| source | `src/measurement.rs`                       | Timing loop that must respect the overhead threshold   |
| source | `src/reporting.rs`                         | Report generation that must complete within 10 seconds |
| test   | `tests/`                                   | Overhead validation and report generation timing tests |
| doc    | `docs/feature/001_measurement_timing.md`   | Feature whose implementation this invariant constrains |
