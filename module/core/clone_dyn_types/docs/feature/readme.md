# Feature Doc Entity

### Scope

- **Purpose**: Document behavioral capabilities of the `clone_dyn_types` crate.
- **Responsibility**: Describe each feature's scope, design, and cross-references.
- **In Scope**: No-std support, DST cloning capabilities, supported type coverage.
- **Out of Scope**: Internal algorithm steps (`algorithm/`), public API contracts (`api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No-Std Support](001_no_std_support.md) | `#![no_std] + alloc` compatibility guarantee | ✅ |
| 002 | [DST Cloning](002_dst_cloning.md) | Boxed trait object and DST clone capability | ✅ |
| 003 | [Type Coverage](003_type_coverage.md) | Supported concrete types for CloneDyn | ✅ |
