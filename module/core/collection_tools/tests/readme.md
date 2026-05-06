# Collection Tools Tests

### Scope

Test suite for the `collection_tools` crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `compile_fail_test.rs` | trybuild runner for into/FT-02 (type annotation required) |
| `feature_gate_compile_fail_test.rs` | Subprocess cargo check for FT-07, into/FT-06, AP-09 (feature gate isolation) |
| `heap_macro_availability_test.rs` | Verifies heap macro public API accessibility (bug reproducer for issue-1) |
| `manual_corner_cases_test.rs` | Comprehensive collection macro corner case tests |
| `no_std_alloc_test.rs` | Invariant tests for no_std allocation selection (cfg-gated on use_alloc feature) |
| `tests.rs` | Test aggregation entry point |
| `compile_fail/` | trybuild compile-fail fixtures (.rs + .stderr golden files) |
| `docs/` | Test spec surface for each doc entity instance |
| `inc/` | Modular per-collection-type constructor tests |
| `manual/` | Manual testing plan and execution records |

### Test Organization

#### Bug Reproducer Tests
- `heap_macro_availability_test.rs`: Permanent regression test for Issue #1 (heap macro accessibility bug)

#### Corner Case Tests
- `manual_corner_cases_test.rs`: Comprehensive edge case coverage
  - Empty collections (all 8 collection types)
  - Single element collections
  - Trailing comma support
  - Capacity pre-allocation verification
  - Duplicate key behavior (HashMap, BTreeMap)
  - Large collections (10-20 elements)
  - Type inference scenarios
  - BinaryHeap max-heap ordering verification
  - Into macros with heterogeneous types (mixing &str and String)
  - Complex nested types (Vec<Vec<T>>, HashMap<K, Vec<V>>, tuples, Options, Results)
  - Non-Copy types and move semantics (String, Box, custom structs)

#### Modular Tests (`inc/`)
See `inc/readme.md` for the full responsibility table.

### Test Coverage

- **Functional Tests**: Coverage across all collection constructors — empty, single-element, trailing-comma, capacity, duplicate-key, large collections, heap ordering, heterogeneous types, nested types, move semantics
- **Doc Tests**: Documentation examples in readme.md and source doc comments
- **Invariant Tests**: no_std allocation selection (use_alloc feature), capacity pre-allocation
