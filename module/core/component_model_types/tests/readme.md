# Component Model Types Tests

## Organization Principles

Tests organized by functional domain (smoke tests vs corner case tests) rather than by methodology (unit, integration). Smoke tests verify basic functionality without circular dependencies. Corner case tests explore edge cases and advanced usage patterns.

## Directory Structure

```
tests/
├── readme.md              # This file
├── smoke_test.rs          # Basic functionality verification
├── corner_cases.rs        # Edge cases and boundary conditions
└── manual/                # Manual testing
    ├── readme.md          # Manual testing procedures
    └── -*.md              # Temporary test plans/results
```

### Scope

**Responsibilities:**

Organizes all automated tests for component_model_types trait system functionality. Covers basic trait implementation verification (Assign, AssignWithType, OptionExt), edge case testing (empty values, type conversions, builder patterns), and integration testing with collection_tools dependency. Targets Rust 1.70+ with no_std support.

**In Scope:**

- Core trait functionality (Assign, AssignWithType, OptionExt)
- Builder pattern support (impute method chaining)
- Type conversion testing (Into trait integration)
- Option extension behavior (None → Some, Some → Some transitions)
- Edge cases (empty strings, boundary values, multiple assignments)
- Feature gating verification (enabled, types_component_assign)
- Dependency loading verification (collection_tools)

**Out of Scope:**

- Derive macro generation (covered in component_model crate)
- Performance/benchmarking (types-only crate, minimal performance concerns)
- End-user integration tests (covered in consumer crates: former, component_model)
- Procedural macro tests (no macros in this crate)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Basic trait functionality | `smoke_test.rs` | Assign, AssignWithType, OptionExt basic behavior |
| Edge cases | `corner_cases.rs` | Builder patterns, empty values, type conversions |
| Manual verification | `manual/` | Examples execution, feature combinations |
| Dependency loading | `smoke_test.rs::smoke_test_crate_loads` | collection_tools integration |

## Adding New Tests

**Q: Testing new trait method?**
→ Add to `smoke_test.rs` if basic functionality, `corner_cases.rs` if edge case

**Q: Testing new type conversion?**
→ Add to `corner_cases.rs::test_type_conversion_*` family

**Q: Testing new Option behavior?**
→ Add to `smoke_test.rs` (basic) or `corner_cases.rs` (edge case like Some→Some)

**Q: Testing feature flag interaction?**
→ Add feature-gated test to appropriate file (use `#[cfg(feature = "...")]`)

**Q: Testing entirely new trait or module?**
→ 1. Create new test file named after module (e.g., `popular_types_test.rs`)
→ 2. Update this readme.md with new domain entry
→ 3. Add to domain map table above

## Test File Naming Conventions

- **`smoke_test.rs`**: Baseline functionality, always passes or build fails
- **`corner_cases.rs`**: Edge cases, boundary conditions, advanced patterns
- **`[module]_test.rs`**: Module-specific tests (if needed in future)

## Running Tests

```bash
# Run all tests with all features
cargo nextest run --all-features

# Run smoke tests only
cargo nextest run smoke_test --all-features

# Run corner case tests only
cargo nextest run corner_cases --all-features

# Run with no features (tests feature gating)
cargo nextest run --no-default-features

# Run documentation tests
cargo test --doc --all-features

# Run full test suite (level 3)
w3 .test l::3
```

## Manual Testing

See `manual/readme.md` for manual testing procedures and comprehensive example testing.

## Test Organization Standards

All tests follow test_organization.rulebook.md standards:

- **Test Matrix**: Each file documents comprehensive test matrix
- **One Aspect Per Test**: Each test function tests single behavior
- **Environmental Independence**: No reliance on external state
- **Feature Gating**: Tests use appropriate `#[cfg(feature = "...")]` guards
- **Documentation**: Module-level and function-level doc comments required
