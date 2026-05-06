# derive_tools_meta Tests

## Organization Principles

Tests organized by functionality: smoke tests for basic crate health, derive-specific tests for individual macro behavior.

Currently minimal coverage with only smoke tests. Future expansion should add comprehensive derive-specific tests.

## Directory Structure

```
tests/
├── readme.md                      # This file
├── smoke_test.rs                  # Basic smoke tests (local + published)
├── deref_derive_test.rs           # Comprehensive Deref derive behavior tests
├── deref_mut_derive_test.rs       # Comprehensive DerefMut derive behavior tests
├── new_derive_test.rs             # Comprehensive New derive behavior tests
├── from_derive_test.rs            # Comprehensive From derive behavior tests
├── index_derive_test.rs           # Comprehensive Index derive behavior tests
├── index_mut_derive_test.rs       # Comprehensive IndexMut derive behavior tests
├── as_ref_derive_test.rs          # Comprehensive AsRef derive behavior tests
├── as_mut_derive_test.rs          # Comprehensive AsMut derive behavior tests
├── not_derive_test.rs             # Comprehensive Not derive behavior tests
├── inner_from_derive_test.rs      # Comprehensive InnerFrom derive behavior tests
├── variadic_from_derive_test.rs   # Comprehensive VariadicFrom derive behavior tests
├── derive_integration_test.rs     # Integration tests for multiple derives working together
└── doc_example_syntax_validation_test.rs  # Validates correct attribute syntax in doc examples
```

### Scope

**Responsibilities:**

Validates procedural macro functionality for `derive_tools_meta` crate. Currently provides basic smoke tests ensuring crate compiles and integrates correctly in both local and published contexts. Future expansion to include comprehensive derive macro behavior tests.

**In Scope:**
- Smoke tests (crate compilation and basic functionality)
- Derive macro behavior tests (future: per-macro test files)
- Edge case and corner case tests for macro expansion (future)
- Error message quality tests for invalid inputs (future)

**Out of Scope:**
- Performance benchmarks (see `benches/` if created)
- Integration tests with `derive_tools` facade crate (tested in parent crate)
- Macro_tools functionality tests (covered in `macro_tools` crate)
- General derive trait behavior (covered by Rust std tests)

## Domain Map

| Domain | Test Location | What It Tests |
|--------|---------------|---------------|
| Basic health | `smoke_test.rs` | Crate compiles, local + published modes work |
| Derive: Deref | `deref_derive_test.rs` | Deref derive macro: single/multi-field, generics, lifetimes |
| Derive: DerefMut | `deref_mut_derive_test.rs` | DerefMut derive macro: mutable dereference for single/multi-field structs |
| Derive: New | `new_derive_test.rs` | New derive macro constructor generation (named structs only) |
| Derive: From | `from_derive_test.rs` | From derive macro: single-field auto-conversion, multi-field tuple with marker |
| Derive: Index | `index_derive_test.rs` | Index derive macro: field access syntactic sugar (index param ignored) |
| Derive: IndexMut | `index_mut_derive_test.rs` | IndexMut derive macro: generates both Index and IndexMut implementations |
| Derive: AsRef | `as_ref_derive_test.rs` | AsRef derive macro: reference conversion for single/multi-field, generics |
| Derive: AsMut | `as_mut_derive_test.rs` | AsMut derive macro: mutable reference conversion with marker support |
| Derive: Not | `not_derive_test.rs` | Not derive macro: logical negation for unit/single-field structs with bool |
| Derive: InnerFrom | `inner_from_derive_test.rs` | InnerFrom derive macro: inherent method for single-field conversion |
| Derive: VariadicFrom | `variadic_from_derive_test.rs` | VariadicFrom derive macro: inherent method for structs and enum variants |
| Integration | `derive_integration_test.rs` | Multiple derives working together: combinations, std derives, nested types |
| Doc examples | `doc_example_syntax_validation_test.rs` | AsMut/AsRef/Deref/DerefMut: correct attribute syntax (no parameter) |

## Adding New Tests

**Current State:** Comprehensive test coverage for all 11 active derives plus integration tests.

**Future Expansion Guide:**

**Q: Testing new derive macro behavior?**
→ Create `{macro_name}_derive_test.rs` or add to appropriate domain file

**Q: Testing error messages for invalid derive usage?**
→ Add to derive-specific test file with comprehensive test matrix

**Q: Testing edge cases (empty structs, generics, lifetimes)?**
→ Add to derive-specific test file covering all corner cases

**Q: Testing interaction between multiple derives?**
→ Create `derive_interactions_test.rs` or add to relevant derive file

**Reorganization Trigger:**
When adding first derive-specific test file, update this readme.md to document new organization.

## Test Matrix Requirements

Each derive-specific test file MUST document comprehensive test matrix covering:
- Happy path (valid usage)
- Edge cases (empty structs, unit structs, enums)
- Boundary conditions (maximum field count, complex generics)
- Error conditions (invalid attributes, conflicting derives)
- Special cases (lifetimes, where clauses, PhantomData)

See test_organization.rulebook.md § Test Matrix Documentation for detailed requirements.

## Known Issues Fixed During Testing

**Issue #2: New Derive Generated Invalid Code (CRITICAL - FIXED)**
- **Root Cause:** New derive implementation generated `impl crate::New` for non-existent trait
- **Why Not Caught:** No tests existed for New derive; all derive_tools tests were commented out
- **Fix Applied:** Changed implementation to generate inherent `impl` with `pub fn new()` method instead of trait implementation (src/derive/new.rs:54-85, 87-140)
- **Prevention:** Added comprehensive test suite (new_derive_test.rs) with 4 test cases covering unit structs, single-field, multi-field, and generics
- **Pitfall:** Proc macros that generate trait implementations must verify the trait exists and is in scope. Inherent implementations are safer for utility methods like constructors.

**Issue #3: InnerFrom Derive Generated Code for Non-Existent Trait (CRITICAL - FIXED)**
- **Root Cause:** InnerFrom derive implementation generated `impl crate::InnerFrom<T>` but no InnerFrom trait exists in the crate (src/derive/inner_from.rs:88)
- **Why Not Caught:** All InnerFrom tests in parent derive_tools crate were commented out with note "InnerFrom derive not available" (tests/inc/inner_from_only_test.rs:5)
- **Fix Applied:** Changed implementation to generate inherent `impl` with `pub fn inner_from()` method instead of trait implementation (src/derive/inner_from.rs:88)
- **Prevention:** Added comprehensive test suite (inner_from_derive_test.rs) with 3 test cases covering tuple/named structs and generics
- **Pitfall:** Proc macros that generate trait implementations must verify the trait exists and is in scope. Inherent implementations are safer for utility methods like constructors.

**Issue #4: VariadicFrom Derive Generated Code for Non-Existent Trait (CRITICAL - FIXED)**
- **Root Cause:** VariadicFrom derive implementation generated `impl crate::VariadicFrom<T>` but no VariadicFrom trait exists (src/derive/variadic_from.rs:107, 208)
- **Why Not Caught:** No comprehensive test coverage for VariadicFrom derive in derive_tools_meta
- **Fix Applied:** Changed implementation to generate inherent `impl` with `pub fn variadic_from()` method instead of trait implementation (src/derive/variadic_from.rs:107, 208)
- **Prevention:** Added comprehensive test suite (variadic_from_derive_test.rs) with 4 test cases covering tuple/named structs, generics, and enum variants
- **Pitfall:** Variadic conversion patterns require custom traits. Must define trait before implementing it, or use inherent methods for utility constructors.

**Issue #5: DerefMut Generic Type Support Broken (HIGH - FIXED)**
- **Root Cause:** DerefMut derive used `generic_params::decompose()` which returns Punctuated types incompatible with quote! macro, causing "expected one of..." compilation errors with generic types (src/derive/deref_mut.rs:16)
- **Why Not Caught:** No generic type test existed for DerefMut. Deref derive had passing generic test, but DerefMut was not tested.
- **Fix Applied:** Changed to use `split_for_impl()` instead of `decompose()`, matching Deref implementation pattern (src/derive/deref_mut.rs:9-11, 101-103)
- **Prevention:** Added generic type test to deref_mut_derive_test.rs validating both String and Vec<i32> generic wrappers
- **Pitfall:** Always use `split_for_impl()` for trait implementations, not `decompose()`. `split_for_impl()` returns properly formatted ImplGenerics and TypeGenerics that work correctly with quote! macro.

## Current Test Coverage

**Test Statistics:** 49 tests total (39 derive-specific + 8 integration + 2 smoke)

**Derives Tested (11 of 11 active - 100% coverage):**
- ✅ Deref - 5 tests (single-field tuple/named, multi-field with marker, generics, lifetimes)
- ✅ DerefMut - 4 tests (single-field tuple/named, multi-field with marker, generics)
- ✅ New - 4 tests (unit struct, single-field, multi-field, generics)
- ✅ From - 4 tests (single-field tuple/named, multi-field tuple with marker, generics)
- ✅ Index - 4 tests (single-field tuple/named, multi-field with marker, generics)
- ✅ IndexMut - 3 tests (single-field tuple/named, multi-field with marker)
- ✅ AsRef - 3 tests (single-field tuple/named, generics)
- ✅ AsMut - 3 tests (single-field tuple/named, multi-field with marker)
- ✅ Not - 3 tests (unit struct, single-field tuple/named with bool)
- ✅ InnerFrom - 3 tests (single-field tuple/named, generics)
- ✅ VariadicFrom - 3 tests (single-field tuple/named, generics - structs only)

**Integration Tests (8):**
- ✅ Deref + DerefMut together
- ✅ AsRef + AsMut together
- ✅ IndexMut provides both Index and IndexMut
- ✅ Deref + From + New (newtype pattern)
- ✅ Integration with std derives (Clone, Debug, PartialEq)
- ✅ Nested derived types (wrapper of wrapper)
- ✅ Multi-field with multiple marker attributes
- ✅ Generic types with multiple derives

**Implementation Limitations Found:**
- New derive: Only supports named structs, NOT tuple structs
- From derive: Multi-field named structs NOT supported (implementation generates invalid code)
- Not derive: Generic types need manual where clause (doesn't add `where T: Not`)
- Index/IndexMut semantic: Index parameter is ignored, provides field access syntax only
- VariadicFrom derive: Enum support NOT functional (inherent methods cannot support multiple variants with same method name)

**Recommended Future Work:**
1. Add `VariadicFrom` trait definition to enable enum support (currently structs only)
2. Add negative tests for error cases (invalid derive usage)
3. Test error message quality for invalid inputs
4. Test interaction with more complex generics, lifetimes, and where clauses
