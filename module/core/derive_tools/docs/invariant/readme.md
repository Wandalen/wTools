# Invariant Doc Entity

### Scope

- **Purpose**: Define behavioral contracts that must always hold for this crate.
- **Responsibility**: Master index for all invariant doc instances in this crate.
- **In Scope**: Instance 001 — pure facade constraint preventing proc-macro implementation.
- **Out of Scope**: Desired behavior and processing logic — see `feature/` instances.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Pure Facade](001_pure_facade.md) | This crate must not implement any derive macros itself | ✅ |
