# Convert performance stress test to benchkit compliance

## Description

The performance stress test in `tests/inc/phase4/performance_stress_test.rs` (function `test_performance_stress_full`) currently uses manual timing and lacks the statistical rigor expected from benchkit's professional benchmarking framework. This intensive test is designed to validate unilang's performance characteristics under stress conditions.

The test needs to be converted to use benchkit's `BenchmarkSuite` and analysis capabilities to provide comprehensive performance validation with proper statistical analysis and clear performance thresholds.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Replace manual timing with benchkit's `BenchmarkSuite` for comprehensive analysis
-   Convert registry initialization and command registration tests to benchkit patterns
-   Implement proper feature gating with `#[cfg(feature = "benchmarks")]` and fallback
-   Define performance thresholds and validation logic for stress test results
-   Use benchkit's reporting methods to display comprehensive performance metrics
-   Maintain intensive nature of stress testing with appropriate workload generation
-   Provide clear PASS/FAIL validation based on performance thresholds
-   Include performance recommendations and optimization insights
-   Test compiles and runs successfully with benchmarks feature enabled
-   Preserve existing stress test coverage while adding statistical rigor