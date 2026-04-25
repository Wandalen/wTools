# Async From Tests

### Organization Principles

Tests are organized by functional domain (async conversion traits) rather than by test methodology. This test suite validates that async versions of Rust's standard conversion traits work correctly.

### Directory Structure

```
tests/
├── readme.md                         # This file - test organization guide
├── tests.rs                          # Test harness entry point
├── manual_corner_cases_test.rs       # Comprehensive corner case validation
├── additional_corner_cases_test.rs   # Readme examples and parsing edge cases
├── send_bounds_validation_test.rs    # Send bounds in multi-threaded runtime
└── inc/                              # Test implementation modules
    ├── mod.rs                        # Module declarations
    └── basic_test.rs                 # Integration tests
```

### Scope

#### Responsibilities

Organizes all automated tests for async conversion traits (AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto). Validates trait implementations, blanket implementations, error handling, and async conversion correctness. Targets Rust 1.70+ across all platforms.

#### In Scope

- AsyncFrom trait functionality tests
- AsyncInto blanket implementation tests
- AsyncTryFrom trait functionality tests
- AsyncTryInto blanket implementation tests
- Error path validation for fallible conversions
- Success path validation for infallible conversions

#### Out of Scope

- Performance benchmarking (not needed for trait library)
- Feature flag combination testing (verified at build level via feature flags)
- Manual testing (not applicable to trait definitions)
- Runtime-specific behavior (traits are runtime-agnostic)

#### Responsibility Table

| File | Responsibility |
|------|----------------|
| `tests.rs` | Test harness entry point and module setup |
| `manual_corner_cases_test.rs` | Comprehensive corner case validation (edge cases, boundaries, concurrency) |
| `additional_corner_cases_test.rs` | Additional corner case validation (readme examples, parsing edge cases, format validation) |
| `send_bounds_validation_test.rs` | Validate Send bounds in multi-threaded runtime |
| `inc/mod.rs` | Test module declarations |
| `inc/basic_test.rs` | Integration tests for async conversion traits |

### Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| AsyncFrom/AsyncInto | `inc/basic_test.rs::async_from_test` | Infallible async conversion and blanket implementation |
| AsyncTryFrom/AsyncTryInto | `inc/basic_test.rs::async_try_from_test` | Fallible async conversion and blanket implementation |
| Corner Cases | `manual_corner_cases_test.rs` | Edge cases (empty strings, boundaries, overflow, special chars, concurrency) |
| AsyncFrom/AsyncTryFrom Corner Cases | `additional_corner_cases_test.rs` | Readme examples and parsing edge cases (AF-5–AF-10, ATF-9–ATF-16) |
| Send Bounds | `send_bounds_validation_test.rs` | Thread-safety bounds in multi-threaded runtime for blanket impls |

### Adding New Tests

**Q: Testing new async conversion scenario?**
→ Add to `inc/basic_test.rs` (async conversion domain)

**Q: Testing feature flag combinations?**
→ Out of scope (feature selection is clear and tested at build level)

**Q: Testing new trait or entirely new domain?**
→ Create new test file in `inc/`, update this readme with new domain entry

### Test Matrix

See individual test files for comprehensive test matrices documenting all test cases, edge conditions, and boundary values.

### Manual Testing

Not applicable - this is a library crate providing trait definitions. All functionality is validated through automated integration tests.
