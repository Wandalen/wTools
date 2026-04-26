# Feature: Measurement and Timing

### Scope

- **Purpose**: Provide low-overhead, accurate timing primitives as the foundation for all benchmarking.
- **Responsibility**: Documents the measurement loop, result aggregation, and custom metrics collection.
- **In Scope**: Single-operation timing, multi-iteration measurement, mean/min/max/stddev statistics, custom metric attachment.
- **Out of Scope**: Statistical significance testing (→ feature/004); report output (→ feature/003).

### Design

Benchmarks are measured by executing a closure repeatedly for a configurable number of iterations. Each iteration's duration is recorded individually; the result set is then aggregated into summary statistics (mean, median, minimum, maximum, standard deviation).

Custom metrics allow callers to attach named floating-point values alongside timing data, enabling throughput, ops/sec, or application-specific measurements to travel with the result.

The measurement loop deliberately stays minimal — no warm-up phase, no outlier removal — to keep overhead predictable. Callers who need statistical rigor can compose with the analysis feature.

### Cross-References

| Type   | File                                      | Responsibility                                          |
|--------|-------------------------------------------|---------------------------------------------------------|
| source | `src/measurement.rs`                      | Result collector and timing loop implementation         |
| source | `src/suite.rs`                            | Suite type for grouped measurement runs                 |
| test   | `tests/`                                  | Integration tests for measurement accuracy              |
| doc    | `docs/api/001_benchkit_api.md`            | Public API surface including measurement operations     |
| doc    | `docs/invariant/001_benches_directory.md` | Directory mandate that governs where benchmarks run     |
| doc    | `docs/invariant/003_performance_nfr.md`   | Overhead budget that measurement must stay within       |
| doc    | `docs/invariant/004_usability_nfr.md`     | Integration ease constraint on the measurement API      |
| doc    | `docs/invariant/005_compatibility_nfr.md` | Platform and environment compatibility requirements      |
| doc    | `docs/invariant/006_reliability_nfr.md`   | Reproducibility requirements for repeated measurements  |
| doc    | `docs/pattern/001_toolkit_not_framework.md` | Design principle this feature exemplifies             |
