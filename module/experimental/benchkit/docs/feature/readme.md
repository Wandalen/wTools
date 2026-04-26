# Feature Doc Entity

### Scope

- **Purpose**: Navigational hubs for each implemented user-facing benchkit capability.
- **Responsibility**: Collects all source, test, and documentation artifacts for each feature in one place.
- **In Scope**: Implemented benchkit features with committed tasks; Design section covering modes and key decisions.
- **Out of Scope**: Implementation details and Rust-specific types (→ source code); NFR constraints (→ invariant/).

### Overview Table

| ID  | Name                                              | Purpose                                              | Status |
|-----|---------------------------------------------------|------------------------------------------------------|--------|
| 001 | [Measurement and Timing](001_measurement_timing.md) | Core timing and metrics collection                  | ✅ |
| 002 | [Data Generation](002_data_generation.md)         | Deterministic test data generation for benchmarks    | ✅ |
| 003 | [Markdown Reports](003_markdown_reports.md)       | Documentation-first report generation and updates    | ✅ |
| 004 | [Performance Analysis](004_performance_analysis.md) | Comparative analysis and regression detection        | ✅ |
