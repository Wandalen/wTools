# Enable Static Command Examples

## Description

Enable the static command examples that were disabled during the test-clean process. This includes `static_01_basic_compile_time.rs`, `static_02_yaml_build_integration.rs`, and `static_03_performance_comparison.rs`. These examples demonstrate the zero-overhead PHF-based static command system and validate the performance requirements.

Links to related tasks: Depends on task 063 (registry integration), leads to CLI builder tasks.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   All static command examples must compile without errors or warnings
-   Examples must demonstrate actual PHF-based zero-overhead lookup
-   Performance examples must validate <1ms p99 latency requirement
-   Examples must use 2-space indentation following codestyle rules
-   Must rename `.disabled` files back to `.rs` extension
-   All examples must run successfully with `cargo run --example <name>`
-   Examples must demonstrate compile-time command registration workflow
-   No clippy warnings when running `cargo clippy --examples --all-features -- -D warnings`