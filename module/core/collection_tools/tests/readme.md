# Collection Tools Tests

Test suite for collection_tools crate.

## Test Files

| File | Responsibility |
|------|----------------|
| `heap_macro_availability_test.rs` | Verifies heap macro public API accessibility (bug reproducer for issue-1) |
| `manual_corner_cases_test.rs` | Tests collection macro edge cases and corner conditions |
| `smoke_test.rs` | Smoke testing (disabled due to circular dependency) |
| `tests.rs` | Test aggregation entry point |
| `inc/` | Subdirectory containing modular collection tests |

## Test Organization

### Bug Reproducer Tests
- `heap_macro_availability_test.rs`: Permanent regression test for Issue #1 (heap macro accessibility bug)

### Corner Case Tests
- `manual_corner_cases_test.rs`: Comprehensive edge case coverage (single elements, trailing commas, capacity, duplicates, large collections)

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

- **Functional Tests**: 57 tests across all collection constructors
- **Doc Tests**: 60 documentation examples
- **Edge Cases**: Single elements, trailing commas, capacity, duplicates, large collections
