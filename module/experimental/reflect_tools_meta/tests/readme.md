# reflect_tools_meta Tests

## Organization Principles

Tests organized by functionality: smoke tests for basic crate health, derive compilation tests for Reflect macro infrastructure.

The Reflect derive implementation is a stub (returns empty `TokenStream`), so tests focus on compilation success and macro infrastructure rather than runtime behavior.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `readme.md` | Document test organization and adding new tests guidance |
| `smoke_test.rs` | Validate crate compilation in local and published contexts |
| `reflect_derive_test.rs` | Test Reflect derive compilation for common struct variations |
| `corner_cases_test.rs` | Test Reflect derive compilation for edge cases and boundary conditions |

## Directory Structure

```
tests/
├── readme.md              # This file
├── smoke_test.rs          # Basic smoke tests (local + published)
├── reflect_derive_test.rs # Reflect derive compilation tests
└── corner_cases_test.rs   # Corner case and edge case compilation tests
```

### Scope

**Responsibilities:**

Validates procedural macro functionality for `reflect_tools_meta` crate. Provides smoke tests ensuring crate compiles and integrates correctly, plus compilation tests verifying the Reflect derive macro infrastructure works for various struct types.

**In Scope:**
- Smoke tests (crate compilation and basic functionality)
- Derive compilation tests (macro accepts valid struct definitions)
- Edge cases (unit structs, tuple structs, generics, lifetimes)
- Debug attribute handling tests
- Future: Runtime behavior tests when implementation is complete

**Out of Scope:**
- Performance benchmarks (see `benches/` if created)
- Integration tests with `reflect_tools` facade crate (tested in parent crate)
- macro_tools functionality tests (covered in `macro_tools` crate)
- Runtime reflection behavior (covered in `reflect_tools` crate)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Basic health | `smoke_test.rs` | Crate compiles, local + published modes work |
| Derive compilation | `reflect_derive_test.rs` | Reflect macro compiles for common struct types |
| Edge cases | `corner_cases_test.rs` | Reflect macro handles boundary conditions and unusual syntax |
| Runtime behavior | (future) | Entity trait implementations when stub replaced |

## Adding New Tests

**Current State:** Smoke tests + compilation tests. Implementation is stub returning empty `TokenStream`.

**Expansion Guide:**

**Q: Testing new struct variants or edge cases?**
→ Add to `reflect_derive_test.rs` following the test matrix pattern

**Q: Testing error messages for invalid derive usage?**
→ Create `reflect_derive_errors_test.rs` when implementation handles errors

**Q: Testing runtime Entity trait behavior?**
→ Add runtime behavior tests to `reflect_derive_test.rs` when implementation complete

**Q: Testing enum support?**
→ Create `reflect_enum_test.rs` when enum support is implemented

## Test Matrix Requirements

Future derive-specific test files MUST document comprehensive test matrix covering:
- Happy path (valid usage)
- Edge cases (empty structs, unit structs, enums)
- Boundary conditions (maximum field count, complex generics)
- Error conditions (invalid attributes, conflicting derives)
- Special cases (lifetimes, where clauses, PhantomData)

See test_organization.rulebook.md § Test Matrix Documentation for detailed requirements.

## Known Issues

None yet. Implementation is currently a stub returning empty TokenStream.
