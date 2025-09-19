# Implement Benchmark Configuration System

## Description

Implement the benchmark configuration system in `src/benchmark_config.rs` that provides environment detection, configuration loading, and performance target management. This system must detect hardware capabilities and provide consistent benchmark parameters across different execution environments.

Links to related tasks: Depends on task 072 (tests), parallel with other benchmarking infrastructure.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Must implement `BenchmarkConfig` with Serde derive for serialization
-   Must provide `detect_environment()` for automatic hardware detection
-   Must implement `load_from_file()` for configuration file loading
-   Must implement `EnvironmentConfig` with CPU, memory, and OS detection
-   Must provide `PerformanceTargets` for benchmark validation
-   Must use 2-space indentation following codestyle rules
-   All tests from task 072 must pass after implementation
-   Must integrate with system information crates for hardware detection
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`