# Feature Doc Entity

### Scope

- **Purpose**: Document the user-facing capabilities of `meta_tools` that consumers select via feature flags.
- **Responsibility**: Define the scope, design decisions, and cross-references for each compile-time meta-programming feature the crate provides.
- **In Scope**: The four feature-gated capabilities (`meta_for_each`, `meta_impls_index`, `mod_interface`, `meta_idents_concat`) and their usage scope.
- **Out of Scope**: Internal module organization, dependency namespace details, and API signatures (see `api/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Macro Iteration](001_macro_iteration.md) | Apply a macro to each element in a comma-delimited list | ✅ |
| 002 | [Trait Implementation Generation](002_trait_impl_generation.md) | Generate trait implementations for tuple types at multiple arities | ✅ |
| 003 | [Module Interface Pattern](003_module_interface.md) | Organize module namespaces via declarative layer declarations | ✅ |
| 004 | [Identifier Concatenation](004_identifier_concatenation.md) | Paste tokens together to construct identifiers at compile time | ✅ |
