# derive_tools Tests

Comprehensive test suite for the `derive_tools` crate, covering smoke tests, derive macro functionality, and bug reproducers.

## File Responsibilities

| File | Responsibility |
|------|---------------|
| smoke_test.rs | Verify crate compilation in local and published contexts |
| tests.rs | Main test suite entry point and module organization |
| example_negative_number_parsing_bug.rs | Reproduce negative number parsing separator conflict bug |

## Test Organization

- **Smoke Tests** (`smoke_test.rs`): Basic compilation and linking verification
- **Comprehensive Derive Tests** (`tests.rs` → `inc/`): Full derive macro behavior coverage
  - From, Deref, DerefMut, AsRef, AsMut, Index, IndexMut
  - Phantom types, generics, lifetimes, bounds
  - Manual implementations vs derived implementations
- **Bug Reproducers** (`example_negative_number_parsing_bug.rs`): Issue regression tests

## Coverage

- 57 tests total
- All derive macros tested with multiple scenarios
- Bug reproducers with comprehensive documentation (Root Cause, Why Not Caught, Fix Applied, Prevention, Pitfall)
