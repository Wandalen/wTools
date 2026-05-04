# Feature Doc Entity

### Scope

- **Purpose**: Document every aggregated sub-crate category so developers understand what wtools exposes and how feature flags gate each module.
- **Responsibility**: Define scope, feature flag structure, and cross-references for each of the 10 aggregated categories.
- **In Scope**: Per-category feature flags, module aliases, sub-feature granularity, and cross-reference tables linking source, config, and related docs.
- **Out of Scope**: Constituent crate internals (see each sub-crate's own docs/), namespace hierarchy (see api/), aggregation rationale (see pattern/).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Iter Aggregation](001_iter_aggregation.md) | Iterator extension utilities via the iter module | ✅ |
| 002 | [Meta Aggregation](002_meta_aggregation.md) | Metaprogramming macros via the meta module | ✅ |
| 003 | [Mem Aggregation](003_mem_aggregation.md) | Memory manipulation utilities via the mem module | ✅ |
| 004 | [Typing Aggregation](004_typing_aggregation.md) | Type inspection and manipulation via the typing module | ✅ |
| 005 | [Time Aggregation](005_time_aggregation.md) | Time and duration utilities via the time module | ✅ |
| 006 | [String Aggregation](006_string_aggregation.md) | String manipulation utilities via the string module | ✅ |
| 007 | [Error Aggregation](007_error_aggregation.md) | Error handling utilities via the error module | ✅ |
| 008 | [Derive Aggregation](008_derive_aggregation.md) | Derive macro collection via the derive module | ✅ |
| 009 | [Dt Aggregation](009_dt_aggregation.md) | Data type utilities via the dt module | ✅ |
| 010 | [Diagnostics Aggregation](010_diagnostics_aggregation.md) | Diagnostic utilities via the diagnostics module | ✅ |
