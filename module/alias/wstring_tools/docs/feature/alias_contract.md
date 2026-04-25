# Feature :: Alias Contract

`wstring_tools` is a named alias over `strs_tools`. It has no independent implementation.

## Re-export Contract

When the `enabled` feature is active, the crate exposes the full `strs_tools` public API
verbatim via `pub use strs_tools::*`. When `enabled` is inactive, the crate compiles to a
zero-dependency stub with no public symbols.

## Feature Mapping

| wstring_tools feature | strs_tools feature activated |
|-----------------------|------------------------------|
| `enabled` | dep activation (`dep:strs_tools`) |
| `indentation` | `strs_tools/string_indentation` |
| `isolate` | `strs_tools/string_isolate` |
| `parse_request` | `strs_tools/string_parse_request` + `split` + `isolate` |
| `parse_number` | `strs_tools/string_parse_number` |
| `split` | `strs_tools/string_split` + `std` |
| `std` | `strs_tools/std` |
| `no_std` | `strs_tools/no_std` |
| `use_alloc` | `strs_tools/use_alloc` |

## Default Feature Set

`default = ["enabled", "indentation", "parse_number"]`

String splitting, isolation, and request parsing are opt-in; they are not enabled by default
because `split` brings in a `std` dependency and the others extend the API surface beyond
the common-case need.

## Full Feature Set

`full = ["enabled", "indentation", "isolate", "parse_request", "split", "parse_number"]`

Activates all functionality features. Does not activate `no_std` or `use_alloc` as those
are environment-configuration features rather than additive capabilities.
