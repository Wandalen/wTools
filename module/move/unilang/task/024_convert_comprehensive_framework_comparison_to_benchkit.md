# Convert comprehensive framework comparison to benchkit

## Description

The comprehensive framework comparison benchmark in `benchmarks/comprehensive_framework_comparison.rs` uses manual timing measurements instead of leveraging benchkit's professional benchmarking infrastructure. The test compares Unilang vs Clap vs Pico-Args across different command counts but lacks statistical rigor and proper measurement methodology.

The test needs to be modernized to use benchkit's `BenchmarkSuite` framework to provide statistically rigorous validation of framework performance characteristics with clear performance metrics and comparative analysis.

Related to audit findings of skipped benchmark tests that need benchkit compliance.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Replace manual timing measurements with benchkit's `BenchmarkSuite` API
-   Convert framework comparison tests to use benchkit algorithms for each framework
-   Implement proper feature gating with `#[cfg(feature = "benchmarks")]` and fallback
-   Provide clear comparative analysis between Unilang, Clap, and Pico-Args performance
-   Display comprehensive benchmark results using benchkit's reporting methods
-   Remove timeout-based approach in favor of proper statistical measurement
-   Include performance validation logic with clear pass/fail criteria
-   Test compiles and runs successfully with proper feature flags
-   Update ignore attribute to reference correct benchmark feature requirements
-   Maintain existing multi-framework comparison capabilities with statistical rigor