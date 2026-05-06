# API: benchkit Public API

### Scope

- **Purpose**: Provide a composable, zero-setup interface for timing benchmarks, generating test data, and updating markdown documentation.
- **Responsibility**: Documents all public operations available through the crate prelude and feature-gated modules.
- **In Scope**: All operations re-exported via the prelude; feature flag gates for optional capabilities.
- **Out of Scope**: Internal module implementation (→ src/); criterion compatibility shim internals (→ src/validation.rs).

### Abstract

benchkit exposes a layered public API organized around four capabilities: measurement and timing (always available), data generation (data_generators feature), markdown report updating (markdown_reports feature), and comparative analysis (enabled feature). All capabilities are opt-in through Cargo feature flags; the prelude re-exports all operations enabled at compile time.

### Operations

**Measurement and timing** (feature: enabled):
- Construct a named result collector, run a closure for N iterations, and retrieve aggregate statistics (mean, min, max, standard deviation, ops/sec)
- Attach named custom metrics to a result for application-specific values alongside timing data
- Group multiple benchmarks into a suite for batch execution

**Markdown report updating** (feature: markdown_reports):
- Construct a section updater with a target file path and section name; the section name is validated at construction
- Update a named section: replace the section body with new markdown content using exact-match section identification
- Build update chains to update multiple sections in one logical operation
- Render pre-built report templates into markdown strings for common patterns

**Data generation** (feature: data_generators):
- Generate fixed-size collections of random numbers, strings, and byte sequences with an optional seed for reproducibility
- Generate structured parser inputs (comma-separated values, nested structures) at configurable sizes

**Comparative analysis** (feature: enabled):
- Add named algorithm variants using a builder chain
- Run all variants under identical conditions and receive a ranked result set with relative speedup ratios
- Run scaling analysis across a sequence of input sizes and collect the time-vs-size relationship

### Error Handling

Section updater construction returns an error for: empty section name, section name exceeding 100 characters, section name containing newline characters, and section names where one is a strict substring of another (section conflict). File update operations return IO errors for missing files, permission failures, and disk write failures. All other operations are infallible; they return values directly.

### Compatibility Guarantees

benchkit follows semantic versioning. The public API surface (all operations accessible via the prelude or direct module path) is stable within a major version. Feature flag names are stable; new flags are additive. The `enabled` feature is the minimum viable surface; all other features compose additively on top of it. Internal module structure may change in minor versions.

The following feature flags are defined (in addition to `default`, `full`, and `enabled`):

| Feature flag          | Capability unlocked                                               |
|-----------------------|-------------------------------------------------------------------|
| `markdown_reports`    | Markdown section updating and report template rendering           |
| `data_generators`     | Deterministic data generation for benchmarks and parser tests     |
| `criterion_compat`    | Compatibility shim for criterion-style benchmark harnesses        |
| `html_reports`        | HTML output format for benchmark results                          |
| `json_reports`        | JSON output format for benchmark results                          |
| `statistical_analysis`| Extended statistical measures (confidence intervals, variance CV) |
| `comparative_analysis`| Multi-algorithm comparison with ranked speedup ratios             |
| `diff_analysis`       | Diff-based regression detection between benchmark runs            |
| `visualization`       | Chart and graph generation from result data                       |
| `optimization_hints`  | Automated suggestions derived from performance analysis results   |

### Cross-References

| Type   | File                                       | Responsibility                                        |
|--------|--------------------------------------------|-------------------------------------------------------|
| source | `src/lib.rs`                               | Feature-gated module declarations and prelude         |
| source | `src/measurement.rs`                       | Result collector and timing primitives                |
| source | `src/reporting.rs`                         | Section updater and reporting error types             |
| source | `src/analysis.rs`                          | Comparative analysis builder and result types         |
| source | `src/suite.rs`                             | Suite grouped execution                               |
| config | `Cargo.toml`                               | Feature flag definitions and dependency gates         |
| doc    | `docs/feature/001_measurement_timing.md`   | Measurement and timing feature scope                  |
| doc    | `docs/feature/002_data_generation.md`      | Data generation feature scope                         |
| doc    | `docs/feature/003_markdown_reports.md`     | Markdown reports feature scope                        |
| doc    | `docs/feature/004_performance_analysis.md` | Performance analysis feature scope                    |
| doc    | `docs/pattern/002_markdown_first_reporting.md` | Architectural pattern behind the markdown API     |
