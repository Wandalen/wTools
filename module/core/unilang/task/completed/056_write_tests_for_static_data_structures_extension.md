# Write Tests for Static Data Structures Extension

## Description

Write comprehensive tests for extending the existing `src/static_data.rs` module with new static command data structures. This includes `StaticCommandDefinition`, `StaticArgumentDefinition`, and the `StaticCommandMap` type alias. These structures are the foundation for the PHF-based static command registry system that will enable zero-overhead command lookup.

Links to related tasks: This is the first task in the static command registry implementation sequence, followed by tasks 057 and 058.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Tests must be located in the `tests/` directory as per design rules
-   All tests must use 2-space indentation following codestyle rules
-   Tests must cover conversion between dynamic `CommandDefinition` and static `StaticCommandDefinition`
-   Tests must verify PHF map type compatibility with `phf::Map<&'static str, &'static StaticCommandDefinition>`
-   Tests must validate serialization/deserialization for build.rs code generation
-   All tests must pass with `cargo test`
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`

## Outcomes

Successfully completed comprehensive test suite for static data structures extension:

- **Test File Created**: Created `/home/user1/pro/lib/wTools/module/move/unilang/tests/static_data_structures_extension_test.rs` with 15 comprehensive tests
- **Static Structure Tests**: Implemented tests for `StaticCommandDefinition`, `StaticArgumentDefinition`, `StaticKind`, and `StaticValidationRule` creation and validation
- **PHF Map Compatibility Tests**: Verified that static structures work correctly with PHF maps for zero-overhead lookups
- **Conversion Tests**: Comprehensive testing of conversion between static and dynamic data structures
- **Type Safety Validation**: Tests ensure proper type conversion for all StaticKind variants (String, Integer, Float, Boolean, Path, File, Directory, Enum, Url, DateTime, Pattern, List, Map, JsonString, Object)
- **Validation Rules Testing**: Complete coverage of all StaticValidationRule variants (Min, Max, MinLength, MaxLength, Pattern, MinItems)
- **Serialization Roundtrip Tests**: Verification that static structures can be converted to dynamic and maintain data integrity
- **Complex Arguments Testing**: Tests with nested arguments, lists, maps, and complex validation rules
- **Code Quality**: All tests pass with strict Rust warnings (-D warnings), no clippy violations, and proper documentation
- **Test Coverage**: 15 tests covering all major functionality areas:
  1. Basic static structure creation
  2. All StaticKind variants
  3. All StaticValidationRule variants
  4. Static to dynamic conversions
  5. Complex type conversions (Enum, List, Map)
  6. PHF map compatibility
  7. Commands with arguments
  8. Serialization roundtrip testing
  9. Type alias validation

All tests compile and run successfully with ctest3 validation (nextest + doctests + clippy).