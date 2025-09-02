# Convert run all benchmarks suite to benchkit

## Description

The run all benchmarks test in `benchmarks/run_all_benchmarks.rs` is a meta-test that runs other benchmarks using manual timing and timeout-based approaches instead of proper benchkit orchestration. It creates circular dependencies by calling other test functions and lacks proper statistical analysis of results.

The test needs to be converted to use benchkit's `BenchmarkSuite` to orchestrate comprehensive performance testing with statistical rigor, removing circular dependencies and providing meaningful performance validation for the entire unilang framework.

Related to audit findings of skipped benchmark tests that need benchkit compliance.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Replace timeout-based meta-benchmark with proper benchkit `BenchmarkSuite`
-   Remove circular dependencies by implementing direct benchmarks instead of calling test functions
-   Add core unilang performance benchmarks (command registry, parsing, YAML loading)
-   Implement proper feature gating with `#[cfg(feature = "benchmarks")]`
-   Provide comprehensive performance validation with clear pass/fail criteria
-   Display unified benchmark results using benchkit's reporting methods
-   Include performance thresholds and validation logic for all core operations
-   Test compiles and runs successfully with benchmarks feature enabled
-   Update ignore attribute to reference correct benchmark feature requirements
-   Provide actionable performance insights and recommendations