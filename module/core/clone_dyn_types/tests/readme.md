# clone_dyn_types Tests

## Organization Principles

Tests organized by functional domain (generic types, slices, trait objects) rather than by methodology (unit, integration). Each test module focuses on specific aspects of CloneDyn trait implementations and helper functions.

## Directory Structure

```
tests/
├── readme.md                        # This file
├── smoke_test.rs                    # Basic smoke tests (no external dependencies)
├── tests.rs                         # Main test suite (6 modules)
├── clone_arrays_test.rs             # Array cloning tests
├── clone_tuples_test.rs             # Tuple cloning tests
├── additional_corner_cases_test.rs  # Iterator edge case tests
└── inc/                             # Test module declarations
    └── mod.rs                       # Module aggregator (external tests disabled)
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate clone() and clone_into_box() without external dependencies |
| `tests.rs` | Validate CloneDyn impls across generic, slice, str, and trait-object categories |
| `clone_arrays_test.rs` | Validate array cloning for sizes 0 through 128 via blanket impl |
| `clone_tuples_test.rs` | Validate tuple cloning for arities 0 through 12 and mixed element types |
| `additional_corner_cases_test.rs` | Validate iterator edge cases: partial, double-ended, large, empty |
| `inc/` | Manage external test integration (disabled: circular dependency) |
| `manual/` | Manual testing procedures for example compilation and helper functions |

### Scope

#### Responsibilities

Validates clone_dyn_types runtime functionality covering CloneDyn trait implementations for standard library types, helper functions (clone, clone_into_box), and trait object cloning patterns. Tests ensure types implementing Clone can be cloned through trait objects and unsized types (slices, str slices) are handled correctly. Targets Rust stable on all platforms.

#### In Scope

- CloneDyn trait implementations for standard library types
- Helper function `clone()` for generic cloning
- Helper function `clone_into_box()` for trait object cloning
- Generic types: primitives (i32, bool, char), String, Vec, custom structs
- Unsized types: slices (&[T]), str slices (&str), including empty and Unicode
- Trait object cloning: Box<dyn Trait> with CloneDyn bound
- Clone independence: ensuring cloned values are independent of originals

#### Out of Scope

- Procedural macro functionality (tested in clone_dyn_meta)
- `#[clone_dyn]` attribute usage (tested in clone_dyn facade crate)
- Performance benchmarks (would be in benches/)
- Thread safety runtime behavior (compile-time markers only)
- External test_tools integration (removed due to circular dependency)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke tests | `smoke_test.rs` | Basic clone() and clone_into_box() functionality |
| Generic types | `tests.rs::clone_generic_types` | Primitives, String, Vec, custom structs |
| Slices | `tests.rs::clone_slices` | Slice cloning (&[T]), empty slices, unsized types |
| String slices | `tests.rs::clone_str_slices` | str cloning (&str), empty strings, Unicode |
| Trait objects | `tests.rs::clone_trait_objects` | Box<dyn Trait> cloning with CloneDyn bound |
| Corner cases | `tests.rs::clone_corner_cases` | Single-element slices, large slices, non-Copy types, ZSTs, Drop types, very long strings |
| Iterator examples | `tests.rs::clone_iterator_from_example` | Iterator trait from example (Some/None cases, independence) |
| Arrays | `clone_arrays_test.rs::clone_arrays` | Array cloning [T; N] for sizes 0, 1, 3, 8, 16, 32, 64, 128 |
| Tuples | `clone_tuples_test.rs::clone_tuples` | Tuple cloning for arities 0–12, mixed types, nested tuples |
| Iterator corner cases | `additional_corner_cases_test.rs` | Iterator edge cases: partial consumption, double-ended, len accuracy, large, empty |

## Adding New Tests

**Q: Testing new standard library type (e.g., HashMap)?**
→ Add to `tests.rs::clone_generic_types` module

**Q: Testing new unsized type (e.g., [T] without reference)?**
→ Add to `tests.rs::clone_slices` module (or create new module if fundamentally different)

**Q: Testing trait object with multiple bounds (e.g., CloneDyn + Send)?**
→ Add to `tests.rs::clone_trait_objects` module

**Q: Testing new array size or element type?**
→ Add to `clone_arrays_test.rs::clone_arrays` module

**Q: Testing new tuple arity or element type combination?**
→ Add to `clone_tuples_test.rs::clone_tuples` module

**Q: Testing iterator edge cases (partial consumption, double-ended, etc.)?**
→ Add to `additional_corner_cases_test.rs`

**Q: Testing helper function edge cases?**
→ Add to `smoke_test.rs` for basic cases, or relevant domain module in `tests.rs`

**Q: Testing entirely new domain (e.g., Clone implementations for custom DST)?**
→ 1. Add new module to `tests.rs` (e.g., `mod clone_custom_dst`)
→ 2. Update this readme.md with new domain entry
→ 3. Add to domain map table above

## File Naming Conventions

- Test files use `snake_case.rs`
- Smoke tests: `smoke_test.rs` (standard pattern)
- Main test suite: `tests.rs` (contains multiple domain modules)
- Domain modules: descriptive names within `tests.rs` (e.g., `clone_generic_types`, `clone_slices`)

## Special Considerations

- All tests use `#[cfg(feature = "enabled")]` for feature-gated testing
- Tests use `the_module` alias in main aggregator for flexibility
- Unsized types (slices, str) require double reference pattern for trait object coercion: `&slice as &dyn CloneDyn`
- Smoke tests avoid external dependencies due to circular dependency with test_tools
- External tests from clone_dyn facade crate are disabled in `inc/mod.rs` (circular dependency)
- All trait object Clone implementations require `#[allow(non_local_definitions)]` attribute
