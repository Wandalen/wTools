# Invariant Doc Entity

### Scope

- **Purpose**: Document correctness properties that must hold unconditionally throughout benchkit's operation.
- **Responsibility**: Collects invariant statements, enforcement mechanisms, and violation consequences.
- **In Scope**: Properties that, if violated, produce incorrect benchmark results or corrupt documentation files.
- **Out of Scope**: Usage conventions and API design (→ feature/, → api/).

### Overview Table

| ID  | Name                                                              | Purpose                                                        | Status |
|-----|-------------------------------------------------------------------|----------------------------------------------------------------|--------|
| 001 | [Benches Directory Mandate](001_benches_directory.md)             | Benchmarks must reside in the benches/ directory               | ✅ |
| 002 | [Exact Section Match](002_exact_section_match.md)                 | Section markers use exact trimmed match, not substring         | ✅ |
| 003 | [Performance Overhead Constraint](003_performance_nfr.md)         | Measurement and reporting overhead must stay within thresholds | ✅ |
| 004 | [Integration Ease Constraint](004_usability_nfr.md)               | First working benchmark achievable in 10 lines of code         | ✅ |
| 005 | [Platform and Environment Compatibility](005_compatibility_nfr.md) | Full feature set on Linux/macOS/Windows; core timing in no_std | ✅ |
| 006 | [Measurement Reproducibility](006_reliability_nfr.md)             | Repeated runs within ±5% variance; seeded generation identical | ✅ |
