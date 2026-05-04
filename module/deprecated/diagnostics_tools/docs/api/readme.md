# API Doc Entity

### Scope

- **Purpose**: Document the public interface exposed by diagnostics_tools for contributors reviewing API contracts.
- **Responsibility**: Master index for all api doc instances in this crate.
- **In Scope**: Instances covering one logical grouping of public macros each — runtime, compile-time, and memory layout.
- **Out of Scope**: Behavioral rationale and implementation design — see feature/ and invariant/ instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Runtime Assertion Macros](001_runtime_assertion_macros.md) | Always-active and debug-only macro set for runtime checks | ✅ |
| 002 | [Compile-Time Assertion Macros](002_compiletime_assertion_macros.md) | Compile-time cfg predicate assertion via cta_true | ✅ |
| 003 | [Memory Layout Assertion Macros](003_memory_layout_macros.md) | Type-level and value-level size and alignment checks | ✅ |
