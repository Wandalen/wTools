# Tests Organization

## Scope

### Responsibilities

Document test organization strategy for winterval crate, including smoke tests validating basic compilation/execution and test propagation from core interval_adapter crate.

### In Scope

- Smoke tests validating crate compiles and executes
- Test propagation from core interval_adapter crate via #[path] inclusion
- Public API surface verification via the_module alias pattern
- Basic compilation and linking validation

### Out of Scope

- Comprehensive functionality tests (delegated to interval_adapter/tests/)
- Performance benchmarks (not applicable to zero-cost re-export)
- Manual testing (simple alias crate requires no manual procedures)
- Feature-specific testing (features delegate to interval_adapter)

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `smoke_test.rs` | Validate crate compiles, links, and basic API is accessible |
| `interval_tests.rs` | Propagate core crate tests to verify re-export completeness |

## Test Strategy

This re-export crate uses **test propagation pattern** to avoid duplication while ensuring correctness:

1. **Minimal Smoke Tests** (`smoke_test.rs`):
   - Verify facade crate compiles successfully
   - Validate basic public API accessibility
   - Confirm linking works correctly

2. **Test Propagation** (`interval_tests.rs`):
   - Uses `#[path]` to include core interval_adapter tests
   - Verifies re-exports are complete via `the_module` alias
   - Tests integration between facade and core crate
   - Comprehensive functionality tests located in `interval_adapter/tests/`

3. **Rationale**:
   - Avoids duplicating interval_adapter's comprehensive test suite
   - Focuses facade tests on facade-specific concerns (compilation, re-exports)
   - Follows Anti-Duplication Principle (principles.rulebook.md)
   - Validates the re-export mechanism itself works correctly
