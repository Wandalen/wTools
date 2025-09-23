# Write Tests for PHF Map Generation System

## Description

Write comprehensive tests for the Perfect Hash Function (PHF) map generation system that will be integrated into `build.rs`. This system must parse YAML command definitions and generate Rust code containing static PHF maps for zero-overhead command lookup. Tests should cover YAML parsing, PHF codegen, and the generated code structure.

Links to related tasks: Depends on task 057 (static data structures), leads to task 059 (PHF implementation).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Tests must be located in the `tests/` directory as per design rules
-   Tests must verify YAML command definition parsing from multiple files
-   Tests must validate PHF codegen output format and structure
-   Tests must verify generated code compiles and creates valid `StaticCommandMap`
-   Tests must check STATIC_COMMANDS constant generation
-   Tests must validate build.rs integration points
-   All tests must use 2-space indentation following codestyle rules
-   All tests must pass with `cargo test`
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`

## Outcomes

Successfully completed comprehensive test suite for PHF map generation system:

- **Test File Created**: Created `/home/user1/pro/lib/wTools/module/move/unilang/tests/phf_map_generation_system_test.rs` with 14 comprehensive tests
- **YAML Parsing Tests**: Tests verify parsing of empty YAML, missing files, simple commands, namespaced commands, complex arguments, and multiple commands
- **Build Integration Tests**: Tests validate that build.rs properly responds to YAML file changes and environment variables
- **Error Handling Tests**: Tests validate graceful handling of various YAML edge cases and malformed input
- **PHF Generation Tests**: Tests verify that generated PHF maps compile without warnings and create valid StaticCommandMap structures
- **Argument Kind Coverage**: Tests validate all supported argument kinds (String, Integer, Float, Boolean, Path, File, Directory, Url, DateTime, Pattern, JsonString, Object)
- **Special Character Handling**: Tests verify proper escaping of quotes, backslashes, and special characters in generated code
- **Code Generation Validation**: Tests verify the structure and compilation success of generated static commands
- **Build System Integration**: Tests validate default YAML file handling and environment variable configuration
- **Command Key Generation**: Tests verify correct key generation for both global (.name) and namespaced (namespace.name) commands
- **Test Coverage**: 14 tests covering all major functionality areas:
  1. Empty YAML handling
  2. Missing file handling
  3. Simple command parsing
  4. Namespaced command parsing
  5. Complex arguments parsing
  6. Multiple commands parsing
  7. YAML validation and error handling
  8. All argument kinds parsing
  9. Special character escaping
  10. Build regeneration on changes
  11. Generated code structure validation
  12. Command key generation
  13. PHF map compilation with warnings as errors
  14. Default YAML file behavior

All tests pass with ctest3 validation (nextest + doctests + clippy) and comprehensive coverage of the build.rs PHF generation system.