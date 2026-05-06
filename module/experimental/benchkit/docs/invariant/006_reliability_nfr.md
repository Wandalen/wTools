# Invariant: Measurement Reproducibility

### Scope

- **Purpose**: Ensure benchmark results are stable enough to be trusted as the basis for optimization decisions.
- **Responsibility**: States the variance, seeding, and confidence reporting requirements for reproducible benchmarking.
- **In Scope**: Run-to-run variance of timing results; deterministic data generation; confidence interval reporting.
- **Out of Scope**: Controlling host system noise (OS scheduling, CPU frequency scaling — benchkit documents but cannot eliminate these); absolute accuracy of the system clock.

### Invariant Statement

Three reproducibility properties must hold:
1. For a given benchmark on a stable system, repeated runs must produce mean timing values within ±5% of each other across at least 80% of run pairs.
2. Data generators initialized with the same seed must produce byte-identical output across runs, platforms, and compiler versions.
3. Every result set must include a statistical confidence measure (coefficient of variation or confidence interval width) so callers can assess result reliability before acting on it.

### Enforcement Mechanism

The timing loop runs the closure for a configurable number of iterations; more iterations reduce variance by averaging over more samples. The default iteration count is set to produce ±5% variance on typical hardware; callers can increase it for noisy environments.

Data generator seeding uses a deterministic algorithm with no platform-specific entropy sources. Seeded generators produce identical sequences regardless of OS or architecture. This is tested with cross-platform golden output tests.

Every result value includes the coefficient of variation computed from the timing samples. Results where the CV exceeds a configurable threshold are flagged as unreliable, prompting the caller to increase the iteration count or investigate system interference.

### Violation Consequences

Variance exceeding ±5% makes it impossible to distinguish a 3% performance improvement from measurement noise. Developers make incorrect optimization decisions and waste effort on changes with no real benefit — or miss genuine regressions.

Non-deterministic data generation means benchmarks cannot be reproduced by a second developer or in CI, undermining the value of historical comparison.

Absence of confidence measures causes callers to treat unreliable results as trustworthy, compounding the decision-making problem.

### Cross-References

| Type   | File                                     | Responsibility                                             |
|--------|------------------------------------------|------------------------------------------------------------|
| source | `src/measurement.rs`                     | Iteration count and variance tracking in the timing loop   |
| source | `src/statistical.rs`                     | Coefficient of variation and confidence interval computation|
| source | `src/generators.rs`                      | Seeded deterministic data generation                        |
| test   | `tests/`                                 | Reproducibility and cross-platform golden output tests     |
| doc    | `docs/feature/001_measurement_timing.md` | Feature whose reliability this invariant governs            |
| doc    | `docs/feature/004_performance_analysis.md` | Analysis feature that surfaces confidence measures        |
