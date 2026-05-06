# API Doc Entity

### Scope

- **Purpose**: Define the public programmatic interface contracts for `reflect_tools_meta`.
- **Responsibility**: Document proc-macro operations, accepted inputs, and error conditions.
- **In Scope**: Proc-macro API surface — accepted struct forms, optional attributes, compile-time error behavior.
- **Out of Scope**: Feature flag configuration (→ `Cargo.toml`); internal derive logic (→ `src/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Reflect Derive](001_reflect_derive.md) | Derive macro invocation contract | ✅ |
