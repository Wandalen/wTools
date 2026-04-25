# Invariant Doc Entity

### Scope

- **Purpose**: Define behavioral contracts that must always hold for this crate.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instance 001 — proc-macro crate separation; Instance 002 — selective compilation per feature flag.
- **Out of Scope**: Desired behavior and processing logic — see `feature/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Proc-Macro Crate Separation](001_proc_macro_separation.md) | Macro implementations and consumer API must reside in separate crates | ✅ |
| 002 | [Selective Compilation](002_selective_compilation.md) | Each macro must be independently disableable via feature flag | ✅ |
