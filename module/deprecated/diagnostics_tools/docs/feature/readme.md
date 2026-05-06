# Feature Doc Entity

### Scope

- **Purpose**: Document capabilities provided by diagnostics_tools for contributors and consumers.
- **Responsibility**: Master index for all feature doc instances in this crate.
- **In Scope**: Instances covering one assertion subsystem or environmental concern each — runtime, compile-time, memory layout, and no-std.
- **Out of Scope**: API contracts and behavioral invariants — see api/ and invariant/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Runtime Assertions](001_runtime_assertions.md) | Unified macro set for runtime condition checks with colored diff output | ✅ |
| 002 | [Compile-Time Assertions](002_compiletime_assertions.md) | Build-time cfg condition validation with zero runtime overhead | ✅ |
| 003 | [Memory Layout Assertions](003_memory_layout_assertions.md) | Compile-time size and alignment verification for types and values | ✅ |
| 004 | [No-Std Support](004_no_std_support.md) | Feature flags for embedded and constrained environments | ✅ |
