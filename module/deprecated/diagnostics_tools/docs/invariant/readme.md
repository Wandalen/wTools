# Invariant Doc Entity

### Scope

- **Purpose**: Document behavioral contracts for diagnostics_tools that must hold regardless of code path.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instances covering one correctness property each — debug noop, pretty diff, zero overhead, alloc dependency.
- **Out of Scope**: Feature design and API interface details — see feature/ and api/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Debug Variants Are No-Ops in Release Builds](001_debug_variants_release_noop.md) | a_dbg_* macros compile out entirely in release builds | ✅ |
| 002 | [Equality Assertions Produce Colored Diff Output](002_pretty_diff_output.md) | a_id and a_not_id always delegate to pretty_assertions | ✅ |
| 003 | [Compile-Time Assertions Introduce No Runtime Overhead](003_compiletime_zero_overhead.md) | cta_* macros never emit binary code | ✅ |
| 004 | [Alloc Feature Requires No-Std](004_alloc_requires_no_std.md) | use_alloc is only meaningful in no_std builds | ✅ |
