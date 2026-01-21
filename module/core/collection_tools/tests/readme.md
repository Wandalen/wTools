# Collection Tools Tests

Test suite for collection_tools crate.

## Test Files

| File | Responsibility |
|------|----------------|
| `heap_macro_availability_test.rs` | Verifies heap macro public API accessibility (bug reproducer for issue-1) |
| `manual_corner_cases_test.rs` | Comprehensive collection macro corner case tests (empty collections, heap ordering, heterogeneous types, complex types, move semantics) |
| `smoke_test.rs` | Smoke testing (disabled due to circular dependency) |
| `tests.rs` | Test aggregation entry point |
| `inc/` | Subdirectory containing modular collection tests |

## Test Organization

### Bug Reproducer Tests
- `heap_macro_availability_test.rs`: Permanent regression test for Issue #1 (heap macro accessibility bug)

### Corner Case Tests
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

### Modular Tests (`inc/`)
The `inc/` subdirectory contains modular tests for each collection type:
- `bmap.rs` - BTreeMap constructor tests
- `bset.rs` - BTreeSet constructor tests
- `deque.rs` - VecDeque constructor tests
- `heap.rs` - BinaryHeap constructor tests
- `hmap.rs` - HashMap constructor tests
- `hset.rs` - HashSet constructor tests
- `llist.rs` - LinkedList constructor tests
- `vec.rs` - Vec constructor tests
- `namespace_test.rs` - Namespace exposure verification
- `components.rs` - Component-based testing utilities
- `mod.rs` - Module aggregation

## Test Coverage

- **Functional Tests**: 95+ tests across all collection constructors (8 empty + 38 new corner cases + existing tests)
- **Doc Tests**: 60 documentation examples
- **Edge Cases**: Empty collections, single elements, trailing commas, capacity, duplicates, large collections, heap ordering, heterogeneous types, nested types, move semantics
