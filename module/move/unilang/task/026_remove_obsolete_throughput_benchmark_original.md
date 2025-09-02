# Remove obsolete throughput benchmark original

## Description

The original throughput benchmark in `benchmarks/throughput_benchmark_original.rs` uses legacy manual timing implementation and is superseded by the modernized benchkit version in `throughput_benchmark.rs`. This creates redundancy and confusion in the benchmark suite.

The legacy file should be removed since the benchkit-compliant version already provides superior statistical analysis and performance validation. Keeping both versions creates maintenance overhead and potential inconsistencies.

Related to audit findings of skipped benchmark tests that need benchkit compliance.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Remove the entire `benchmarks/throughput_benchmark_original.rs` file
-   Verify that `benchmarks/throughput_benchmark.rs` provides equivalent functionality with benchkit
-   Ensure no other files reference the obsolete throughput benchmark original
-   Update any documentation or comments that may reference the removed file
-   Verify all tests still pass after removal
-   Confirm that the benchkit version covers all use cases from the original
-   Remove any imports or dependencies that were specific to the original version
-   Ensure clean build with no dangling references