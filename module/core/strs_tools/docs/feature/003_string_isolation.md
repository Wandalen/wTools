# Feature: String Isolation

### Scope

- **Purpose**: Extract a substring from a source string relative to one or two delimiter patterns, without splitting the entire string.
- **Responsibility**: Documents the isolation capability and links to its source, tests, and API contract.
- **In Scope**: Left isolation (content before first delimiter), right isolation (content after last delimiter), between isolation (content between two delimiters).
- **Out of Scope**: Full string splitting (`feature/001`); API operation signatures (`api/002`).

### Design

Isolation operates on a single target relationship rather than decomposing the entire string. Three modes are provided:

Left isolation yields everything before the first occurrence of the delimiter pattern. If the delimiter is absent, the operation returns nothing.

Right isolation yields everything after the last occurrence of the delimiter pattern. If the delimiter is absent, the operation returns nothing.

Between isolation takes two delimiter patterns and yields the content found between the first occurrence of the left delimiter and the first subsequent occurrence of the right delimiter. If either delimiter is absent, the operation returns nothing.

All three modes return optional results: presence indicates the delimiter was found and the substring was extracted; absence indicates the delimiter was not present in the source.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/isolate.rs` | Isolation implementation for all three modes |
| test | `tests/inc/isolate_test.rs` | Isolation correctness and edge case tests |
| doc | `docs/api/002_string_utilities_api.md` | Isolation operation contract |
| doc | `docs/invariant/004_no_std_alloc_contract.md` | No-std compatibility guarantee for core operations |
