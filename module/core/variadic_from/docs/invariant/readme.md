# Invariant Doc Entity

### Scope

- **Purpose**: Capture correctness properties that must always hold for the variadic_from system.
- **Responsibility**: Lists all invariant doc instances, each defining one correctness property and its enforcement.
- **In Scope**: Field count boundary; compile-time argument count limit.
- **Out of Scope**: Algorithm details for how properties are maintained → `algorithm/`; API reference → `api/`.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Field Count Boundary](001_field_count_boundary.md) | 0 or >3 fields generate no code | ✅ |
| 002 | [Compile-Time Arg Count](002_compile_time_arg_count.md) | from! with >3 args produces compile error | ✅ |
