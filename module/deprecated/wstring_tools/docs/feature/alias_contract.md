# Feature: Alias Contract

### Scope

- **Purpose**: Document the re-export contract between `wstring_tools` and `strs_tools`.
- **Responsibility**: Define feature flag mapping, default activation set, and full activation set for the alias.
- **In Scope**: Feature flag mapping table, default activation set, and full activation set.
- **Out of Scope**: String processing capabilities — see `strs_tools` documentation.

### Design

`wstring_tools` is a named alias over `strs_tools` with no independent implementation.

### Re-export Contract

When the `enabled` feature is active, the crate exposes the full `strs_tools` public API
verbatim. When `enabled` is inactive, the crate compiles to a zero-dependency stub with no
public symbols.

### Feature Mapping

| wstring_tools feature | strs_tools feature activated |
|-----------------------|------------------------------|
| `enabled` | dep activation |
| `indentation` | `strs_tools/string_indentation` |
| `isolate` | `strs_tools/string_isolate` |
| `parse_request` | `strs_tools/string_parse_request` + `split` + `isolate` |
| `parse_number` | `strs_tools/string_parse_number` |
| `split` | `strs_tools/string_split` + `std` |
| `std` | `strs_tools/std` |
| `no_std` | `strs_tools/no_std` |
| `use_alloc` | `strs_tools/use_alloc` |

### Default Feature Set

The default activation set enables `enabled`, `indentation`, and `parse_number`. String
splitting, isolation, and request parsing are opt-in; `split` brings in a `std` dependency
and the others extend the API surface beyond the common-case need.

### Full Feature Set

The full activation set enables all functionality features: `enabled`, `indentation`,
`isolate`, `parse_request`, `split`, and `parse_number`. Environment-configuration features
(`no_std`, `use_alloc`) are excluded from the full set as they are not additive capabilities.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `../../../../core/strs_tools/docs/readme.md` | Underlying crate whose API this alias exposes |
