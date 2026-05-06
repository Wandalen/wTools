# Feature: Performance Analysis

### Scope

- **Purpose**: Turn raw timing data into actionable optimization signals through comparison, scaling, and regression detection.
- **Responsibility**: Documents comparative analysis, throughput metrics, scaling behaviour, and regression detection workflows.
- **In Scope**: Side-by-side algorithm comparison, ops/sec throughput, input-size scaling curves, regression detection against baselines.
- **Out of Scope**: Raw measurement collection (→ feature/001); report output (→ feature/003).

### Design

Comparative analysis accepts two or more named algorithm variants, runs each under identical conditions, and produces a ranked result set with relative speedup ratios. A builder-style configuration interface makes multi-variant setup ergonomic without requiring boilerplate per variant.

Throughput analysis translates timing measurements into operations-per-second figures, which are more intuitive for data-structure and I/O benchmarks where absolute nanoseconds are less meaningful than capacity under load.

Scaling analysis runs the same benchmark at multiple input sizes and records the time-vs-size relationship, enabling identification of linear, super-linear, or constant-time behaviour. Results are structured for direct integration with markdown report templates.

Regression detection compares a current run against a stored baseline and flags results that exceed a configurable threshold. The baseline is stored as a previous benchmark result, not as a raw timestamp, ensuring comparisons are semantically meaningful.

### Cross-References

| Type   | File                             | Responsibility                                        |
|--------|----------------------------------|-------------------------------------------------------|
| source | `src/analysis.rs`                | Comparative analysis builder and result ranking       |
| source | `src/comparison.rs`              | Side-by-side comparison utilities                     |
| source | `src/scaling.rs`                 | Input-size scaling analysis                           |
| source | `src/throughput.rs`              | Throughput and ops/sec calculations                   |
| source | `src/profiling.rs`               | Profiling instrumentation helpers                     |
| source | `src/statistical.rs`             | Statistical significance and CV calculations          |
| source | `src/diff.rs`                    | Git-style diff view for benchmark result sets         |
| test   | `tests/`                         | Regression detection and comparison accuracy tests    |
| doc    | `docs/api/001_benchkit_api.md`   | Public API surface including analysis operations      |
| doc    | `docs/invariant/006_reliability_nfr.md` | Reproducibility invariant that analysis must satisfy |
| doc    | `docs/pattern/001_toolkit_not_framework.md` | Design principle this feature exemplifies        |
