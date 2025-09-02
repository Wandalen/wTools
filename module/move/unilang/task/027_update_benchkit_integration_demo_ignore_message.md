# Update benchkit integration demo ignore message

## Description

The benchkit integration demo test in `benchmarks/throughput_benchmark.rs` is already properly implemented using benchkit but has a generic ignore message that doesn't follow the standardized format for benchkit integration tests. The message should be updated to be consistent with other benchkit tests.

This is a minor but important consistency fix to ensure all benchmark tests follow the same ignore message pattern for clarity and maintainability.

Related to audit findings of skipped benchmark tests that need benchkit compliance.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Update ignore attribute from `"Benchkit integration demo - run explicitly"` to `"Benchkit integration - comprehensive throughput analysis"`
-   Ensure the ignore message follows the consistent format used by other benchkit tests
-   Verify the test functionality remains unchanged
-   Confirm the test still compiles and runs correctly with the updated ignore message
-   Maintain all existing benchkit functionality and statistical analysis
-   Ensure ignore message accurately describes the test's purpose
-   Follow standardized naming convention for benchkit integration tests