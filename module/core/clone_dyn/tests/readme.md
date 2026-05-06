# clone_dyn Tests

## Organization Principles

Tests organized by functional domain (basic cloning, parametrized traits, manual implementations) rather than by methodology (unit, integration). Each test file focuses on specific aspect of trait object cloning functionality.

## Directory Structure

```
tests/
├── readme.md              # This file
├── smoke_test.rs          # Basic smoke tests (local + published)
├── tests.rs               # Main test module aggregator
└── inc/                   # Test implementations
    ├── mod.rs             # Module declarations
    ├── basic.rs           # Non-generic trait cloning tests
    ├── parametrized.rs    # Generic trait cloning tests
    ├── basic_manual.rs    # Manual Clone implementation tests
    └── only_test/         # Shared test logic
        └── basic.rs       # clone_into_box and clone function tests
```

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validates basic crate loading and compilation for both local and published crate variants |
| `tests.rs` | Aggregates all functional test modules and provides unified test entry point |
| `inc/` | Contains domain-specific test implementations organized by trait cloning patterns (basic, parametrized, manual) |
| `manual/` | Manual testing procedures for example compilation and feature flag combinations |

### Scope

**Responsibilities:**
Validates clone_dyn facade crate functionality covering procedural macro-generated Clone implementations for trait objects, manual Clone implementations, and helper functions. Tests ensure trait objects (Box<dyn Trait>) can be cloned for non-generic traits, generic traits with type parameters, and various underlying types (primitives, String, slices). Targets Rust stable on all platforms.

**In Scope:**
- `#[clone_dyn]` procedural macro functionality
- Clone implementation for Box<dyn Trait> (all four variants: base, +Send, +Sync, +Send+Sync)
- Non-generic trait cloning (basic.rs)
- Generic trait cloning with type parameters (parametrized.rs)
- Manual Clone implementations without macro (basic_manual.rs)
- Helper functions: `clone()` and `clone_into_box()`
- Various underlying types: i32, i64, String, &str, &[T]
- Integration with clone_dyn_types::CloneDyn trait

**Out of Scope:**
- Procedural macro implementation details (tested in clone_dyn_meta)
- CloneDyn trait implementation (tested in clone_dyn_types)
- Performance benchmarks (would be in benches/)
- Thread safety runtime behavior (compile-time markers only)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Smoke tests | `smoke_test.rs` | Basic crate loading (local + published) |
| Non-generic traits | `inc/basic.rs` | Clone for Box<dyn Trait> with simple trait |
| Generic traits | `inc/parametrized.rs` | Clone for Box<dyn Trait<T1, T2>> with generics |
| Manual implementations | `inc/basic_manual.rs` | Manual Clone without `#[clone_dyn]` macro |
| Helper functions | `inc/only_test/basic.rs` | `clone()` and `clone_into_box()` functions |

## Adding New Tests

**Q: Testing new underlying type (e.g., HashMap)?**
→ Add to `inc/basic.rs` or `inc/parametrized.rs` depending on trait complexity

**Q: Testing Send/Sync variants?**
→ Add to `inc/basic.rs` (compile-time verification)

**Q: Testing macro expansion edge cases?**
→ Add to clone_dyn_meta tests (this is facade crate)

**Q: Testing new helper function?**
→ Add to `inc/only_test/basic.rs`

**Q: Testing entirely new domain (e.g., async trait objects)?**
→ 1. Create new file `inc/async.rs`
→ 2. Update this readme.md with new domain entry
→ 3. Add to domain map table above
→ 4. Declare in `inc/mod.rs`

## File Naming Conventions

- Test files use `snake_case.rs`
- Smoke tests: `smoke_test.rs` (standard pattern)
- Domain tests: descriptive names (e.g., `basic.rs`, `parametrized.rs`)
- Manual variant: suffix `_manual` (e.g., `basic_manual.rs`)

## Special Considerations

- All tests in `inc/` use `use super::*;` to import common test infrastructure
- Tests use `the_module` alias for flexibility in testing different feature combinations
- Parametrized tests demonstrate generic trait support with where clauses
- Manual tests demonstrate usage without procedural macro (for users who prefer explicit control)
