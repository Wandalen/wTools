# API Doc Entity

### Scope

- **Purpose**: Document the public interface of the `#[derive(VariadicFrom)]` proc-macro.
- **Responsibility**: Catalog of api/ doc instances specifying what the macro accepts and what implementations it generates.
- **In Scope**: Supported struct forms, generated impl contract per field count, generic propagation.
- **Out of Scope**: Code generation algorithm internals → [`variadic_from/docs/algorithm/001_variadic_from_derive.md`](../../../variadic_from/docs/algorithm/001_variadic_from_derive.md); trait definitions → [`variadic_from/docs/api/001_from_n_traits.md`](../../../variadic_from/docs/api/001_from_n_traits.md).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [VariadicFrom Derive Macro](001_variadic_from_derive.md) | Proc-macro API: accepted struct forms and generated impl contract | ✅ |
