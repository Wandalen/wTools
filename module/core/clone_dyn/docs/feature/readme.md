# Feature Doc Entity

### Scope

- **Purpose**: Document functional capabilities of the `clone_dyn` facade crate.
- **Responsibility**: Describe the macro-based and manual clone developer interaction models.
- **In Scope**: `#[clone_dyn]` macro usage, manual Clone implementation pattern.
- **Out of Scope**: Proc-macro internals (clone_dyn_meta), runtime trait impls (clone_dyn_types).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macro-Based Cloning](001_macro_usage.md) | `#[clone_dyn]` macro-based cloning pattern | ✅ |
| 002 | [Manual Clone Implementation](002_manual_impl.md) | Manual Clone implementation without macro | ✅ |
