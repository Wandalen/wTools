# Convert simple_json_perf_test.rs to use benchkit properly

## Description

The simple JSON performance test in `tests/simple_json_perf_test.rs` currently uses manual timing with `std::time::Instant` instead of following benchkit's statistical benchmarking patterns. This leads to potentially inaccurate and statistically naive measurements that can be skewed by system noise.

The test needs to be converted to use benchkit's `ComparativeAnalysis` API to provide professional-grade statistical analysis and follow the benchkit "toolkit philosophy" for rigorous performance measurement.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Replace manual timing loops with benchkit's `ComparativeAnalysis::new().algorithm().run()` pattern
-   Use proper feature gating with `#[cfg(feature = "benchmarks")]` and fallback implementation
-   Maintain comparison between serde_json and SIMD JSON parsing performance
-   Provide statistical rigor through benchkit's built-in measurement infrastructure
-   Display results using benchkit's `ComparisonReport` methods (`fastest()`, `sorted_by_performance()`)
-   Calculate and display speedup ratios between algorithms
-   Preserve existing SIMD capability detection and reporting
-   Test compiles and runs successfully with `--features benchmarks`
-   Update ignore attribute to reference correct feature flag requirements