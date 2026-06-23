# Feature: String Isolation

### Scope

- **Purpose**: Extract a substring from a source string relative to one or two delimiter patterns, without splitting the entire string.
- **Responsibility**: Documents the isolation capability and links to its source, tests, and API contract.
- **In Scope**: Left isolation (content before first delimiter), right isolation (content after last delimiter). Between isolation (content between two different delimiters) is a planned extension not yet implemented.
- **Out of Scope**: Full string splitting (`feature/001`); API operation signatures (`api/002`).

### Design

Isolation operates on a single target relationship rather than decomposing the entire string. Two modes are provided:

Left isolation yields everything before the first occurrence of the delimiter pattern. If the delimiter is absent, the operation returns nothing.

Right isolation yields everything after the last occurrence of the delimiter pattern. If the delimiter is absent, the operation returns nothing.

Both modes return optional results: presence indicates the delimiter was found and the substring was extracted; absence indicates the delimiter was not present in the source.

Between isolation (extracting content between two different delimiter patterns) is a planned extension not yet implemented. The current API supports a single delimiter per operation; callers requiring between-style extraction can chain two isolate calls.

### Sources

- [src/string/isolate.rs](../../src/string/isolate.rs) — Isolation implementation for left and right modes

### Tests

- [tests/inc/isolate_test.rs](../../tests/inc/isolate_test.rs) — Isolation correctness and edge case tests

### APIs

- [002_string_utilities_api.md](../api/002_string_utilities_api.md) — Isolation operation contract

### Invariants

- [004_no_std_alloc_contract.md](../invariant/004_no_std_alloc_contract.md) — No-std compatibility guarantee for core operations
