# Feature Doc Entity

### Scope

- **Purpose**: Document user-facing capabilities of the `impls_index` crate.
- **Responsibility**: Track all implemented features as navigational hubs pointing to source, tests, and design artifacts.
- **In Scope**: User-facing macro capabilities — function indexing, test indexing, function manipulation utilities.
- **Out of Scope**: Internal macro API contracts (→ `api/`), correctness invariants (→ `invariant/`), architectural patterns (→ `pattern/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Function Indexing](001_function_indexing.md) | Wrap functions in named macros for explicit code structure | ✅ |
| 002 | [Test Indexing](002_test_indexing.md) | Index test functions with automatic test attribute injection | ✅ |
| 003 | [Function Utilities](003_function_utilities.md) | Manipulate function definitions as token trees | ✅ |
