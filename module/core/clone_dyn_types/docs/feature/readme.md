# Feature Doc Entity

### Scope

- **Purpose**: Document behavioral capabilities of the `clone_dyn_types` crate.
- **Responsibility**: Describe each feature's scope, design, and cross-references.
- **In Scope**: No-std support, DST cloning capabilities, supported type coverage.
- **Out of Scope**: Internal algorithm steps (`algorithm/`), public API contracts (`api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | No-Std Support | `#![no_std] + alloc` compatibility guarantee | ✅ |
| 002 | DST Cloning | Boxed trait object and DST clone capability | ✅ |
| 003 | Type Coverage | Supported concrete types for CloneDyn | ✅ |
