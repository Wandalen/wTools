# Test Organization for reflect_tools

## Organization Principles

The test suite is organized by **functional domain** using a two-tier directory structure:

- **`fundamental/`** - Tests for the `Fields` trait implementation across standard Rust collection types
- **`group1/`** - Tests for reflection capabilities on primitive types, structs, and collections
- **Root level** - Integration smoke tests and test suite entry point

This domain-based organization ensures:
- Clear separation of concerns between trait testing and reflection testing
- Easy discovery of tests for specific functionality
- Natural grouping that matches the crate's architectural boundaries

## Directory Structure

```
tests/
├── readme.md                           # This file - test organization guide
├── tests.rs                            # Main test suite entry point
├── smoke_test.rs                       # Integration smoke tests (local + published)
└── inc/                                # Implementation test modules
    ├── fundamental/                    # Fields trait tests (8 files)
    │   ├── fields_bmap.rs             # Fields for BTreeMap
    │   ├── fields_bset.rs             # Fields for BTreeSet
    │   ├── fields_deque.rs            # Fields for VecDeque
    │   ├── fields_hmap.rs             # Fields for HashMap
    │   ├── fields_hset.rs             # Fields for HashSet
    │   ├── fields_llist.rs            # Fields for LinkedList
    │   ├── fields_test.rs             # Fields general tests
    │   └── fields_vec.rs              # Fields for Vec
    └── group1/                         # Reflection tests (12 files + subdir)
        ├── array_test.rs              # Array reflection
        ├── common_test.rs             # Comprehensive primitive reflection
        ├── hashmap_test.rs            # HashMap reflection
        ├── hashset_test.rs            # HashSet reflection
        ├── is_ordered_test.rs         # Container ordering invariant
        ├── newtype_experiment.rs      # Newtype pattern experiments
        ├── primitive_test.rs          # Primitive data wrapper
        ├── slice_test.rs              # Slice reflection
        ├── struct_in_struct_manual_test.rs    # Nested struct reflection
        ├── struct_manual_test.rs              # Struct reflection
        ├── struct_with_lifetime_manual_test.rs # Lifetime struct reflection
        ├── vec_test.rs                # Vec reflection
        └── only_test/                 # Compile-only tests (4 files)
            ├── all.rs                 # Module aggregator
            ├── reflect_struct.rs      # Struct reflection compile test
            ├── reflect_struct_in_struct.rs     # Nested struct compile test
            └── reflect_struct_with_lifetime.rs # Lifetime struct compile test
```

## Domain Map

| Domain | Description | Test Files |
|--------|-------------|------------|
| **Fields Trait** | `Fields` trait implementation for standard collections | `fundamental/fields_*.rs` (8 files) |
| **Primitive Reflection** | Reflection on integers, floats, references | `group1/common_test.rs`, `group1/primitive_test.rs` |
| **Collection Reflection** | Reflection on Vec, HashMap, HashSet, arrays, slices | `group1/vec_test.rs`, `group1/array_test.rs`, `group1/slice_test.rs`, `group1/hashmap_test.rs`, `group1/hashset_test.rs` |
| **Container Ordering** | is_ordered() correctness for container types | `group1/is_ordered_test.rs` |
| **Struct Reflection** | Reflection on structs (plain, nested, with lifetimes) | `group1/struct_*.rs` (3 manual test files) + `group1/only_test/reflect_struct*.rs` (3 compile tests) |
| **Newtype Experiments** | Experimental newtype pattern support | `group1/newtype_experiment.rs` |
| **Integration** | Smoke tests for local and published crate | `smoke_test.rs` |
| **Test Entry** | Main test module aggregator | `tests.rs` |

## Scope

### Responsibilities

This test suite validates:
- **Fields trait correctness** - Key-value iteration over all standard Rust collections
- **Reflection API completeness** - Type introspection for primitives, collections, and structs
- **Type safety** - Correct TypeId, type name, and entity representation
- **Container detection** - Accurate is_container() and len() for all types
- **Element iteration** - Working elements() iterator for containers
- **Lifetime handling** - Proper reflection of structs with lifetime parameters

### In Scope

- Fields trait for: Vec, VecDeque, LinkedList, HashMap, HashSet, BTreeMap, BTreeSet
- Reflection for: all primitive integers (i8-i64, u8-u64), floats (f32, f64), and references
- Reflection for: Vec, arrays, slices, HashMap, HashSet
- Reflection for: plain structs, nested structs, structs with lifetimes
- Type introspection: type_id(), type_name(), is_container(), len(), elements()
- Compile-time validation (only_test/ directory)
- Integration smoke tests (local + published crate)

### Out of Scope

- Performance benchmarks (no benchmark tests in this suite)
- Reflection for enums (not currently tested)
- Reflection for tuples (not currently tested)
- Reflection for Option/Result (not currently tested)
- Fields trait for custom user types (library crate responsibility)
- Thread safety or concurrent reflection (not tested)
- Reflection for function pointers or closures (not tested)

## Adding New Tests

### Decision Tree

Use this guide to determine where to add new tests:

1. **Are you testing the `Fields` trait implementation?**
   - YES → Add to `inc/fundamental/`
     - New collection type? → Create `fields_<collection>.rs`
     - Extending existing collection tests? → Edit corresponding `fields_*.rs`

2. **Are you testing reflection on a specific type?**
   - YES → Add to `inc/group1/`
     - Primitive type? → Edit `common_test.rs` or `primitive_test.rs`
     - Collection (Vec, array, slice, map, set)? → Edit corresponding `*_test.rs`
     - Struct (plain, nested, or with lifetime)? → Edit corresponding `struct_*_manual_test.rs`
     - New type category? → Create `<type>_test.rs`

3. **Are you adding a compile-only test (no runtime execution)?**
   - YES → Add to `inc/group1/only_test/`
     - Create `reflect_<feature>.rs`
     - Add module to `all.rs`

4. **Are you testing integration or basic crate functionality?**
   - YES → Add to root-level `smoke_test.rs` or create new root-level test file

### File Naming Conventions

- **Fields tests**: `fields_<collection>.rs` (lowercase abbreviation)
  - Example: `fields_vec.rs`, `fields_hmap.rs`, `fields_bset.rs`
- **Reflection tests**: `<type>_test.rs` (descriptive type name)
  - Example: `array_test.rs`, `slice_test.rs`, `hashmap_test.rs`
- **Manual tests**: `<feature>_manual_test.rs` (for tests requiring manual setup)
  - Example: `struct_manual_test.rs`, `struct_with_lifetime_manual_test.rs`
- **Compile-only tests**: `reflect_<feature>.rs` (in only_test/ directory)
  - Example: `reflect_struct.rs`, `reflect_struct_with_lifetime.rs`

## Special Considerations

### Collection Abbreviations

The `fundamental/` directory uses standard abbreviations for collection names:
- `bmap` - BTreeMap
- `bset` - BTreeSet
- `hmap` - HashMap
- `hset` - HashSet
- `llist` - LinkedList
- `deque` - VecDeque
- `vec` - Vec

### Manual vs Automated Tests

Tests with `_manual_` suffix require manual test setup or validation. These are not isolated unit tests but integration-style tests validating complex scenarios.

### Compile-Only Tests (only_test/)

The `only_test/` directory contains tests that validate compile-time behavior. These tests should compile successfully but may not have runtime assertions.

### Common Test Pattern

Most reflection tests follow this pattern:
1. Create test data (primitive, collection, or struct)
2. Call `reflect()` to get Entity
3. Validate: `type_name()`, `type_id()`, `is_container()`, `len()`
4. For containers: validate `elements()` iterator

### Task Markers

- `xxx:` - Implementation needed for additional collections (see `fundamental/fields_vec.rs:10`)
- `qqq:` - Known issues requiring investigation (see `group1/common_test.rs:13`)
