# Feature: String Aggregation

### Scope

- **Purpose**: Provide string manipulation utilities through the wtools string module alias.
- **Responsibility**: Document the string category feature flags, the aliased module, and its granular sub-features.
- **In Scope**: Feature flags in the string namespace, exposed module alias, five granular string-processing sub-features.
- **Out of Scope**: String utility implementation details (see strs_tools docs/).

### Design

The string category re-exports strs_tools under the short alias `string`. It offers five granular sub-features for specific string-processing capabilities. Note the alias difference: the source crate is `strs_tools` but the aggregated module is named `string` for clarity.

| Flag | Enables |
|------|---------|
| `string` | Base sub-crate inclusion |
| `string_default` | indentation, isolate, parse_request, parse_number, split |
| `string_full` | All string sub-features (same as default) |
| `string_no_std` | no_std support |
| `string_use_alloc` | Allocator support in no_std mode |
| `string_indentation` | Indentation manipulation utilities |
| `string_isolate` | String isolation and extraction |
| `string_parse_request` | Request string parsing (depends on string_isolate) |
| `string_parse_number` | Number parsing from strings |
| `string_split` | String splitting utilities (activates parse_request internally) |

The string_parse_request flag implicitly activates string_isolate. The string_split flag implicitly activates parse_request in the source crate.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| config | `../../Cargo.toml` | Feature flag definitions (lines 143-172) |
| source | `../../src/lib.rs` | Feature-gated re-export and namespace wiring |
| doc | `../api/001_namespace_hierarchy.md` | Namespace surface contract |
| doc | `../pattern/002_feature_flag_composition.md` | Feature flag design pattern |
