# Write Tests for CliBuilder API

## Description

Write comprehensive tests for the `CliBuilder` fluent API that enables ergonomic CLI aggregation. This builder pattern allows combining multiple CLI modules with prefixes, conflict detection, and namespace isolation. Tests should cover the builder pattern, module aggregation, and conflict detection functionality.

Links to related tasks: Independent of static registry tasks, leads to task 066 (CliBuilder implementation).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Tests must be located in the `tests/` directory as per design rules
-   Tests must verify fluent API builder pattern functionality
-   Tests must validate `static_module_with_prefix()` method behavior
-   Tests must check conflict detection system for duplicate prefixes
-   Tests must verify namespace isolation between modules
-   Tests must validate `build_static()` method creating unified registry
-   All tests must use 2-space indentation following codestyle rules
-   All tests must pass with `cargo test`
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`