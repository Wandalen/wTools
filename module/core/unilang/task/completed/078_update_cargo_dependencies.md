# Update Cargo Dependencies

## Description

Update `Cargo.toml` with the required dependencies for static command registry, CLI aggregation, and advanced benchmarking infrastructure. This includes adding PHF crates, YAML processing, hardware detection, and configuring proper feature flags for the new functionality.

Links to related tasks: Support task for all implementations, should be done early in the process.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Must add `phf` and `phf_codegen` dependencies for static command registry
-   Must add `walkdir` for YAML file discovery
-   Must add system information crates for hardware detection
-   Must configure feature flags: `static_commands`, `multi_yaml`, `advanced_benchmarks`
-   Must update `[build-dependencies]` section for build.rs requirements
-   Must maintain existing dependency versions where possible
-   All dependencies must compile successfully
-   Feature flags must enable/disable functionality correctly
-   No version conflicts or dependency resolution issues

## Outcomes

Successfully updated Cargo.toml with required dependencies:

- **Dependencies Added**: Added `walkdir = "2.4"` and `sysinfo = "0.30"` as optional dependencies for multi-YAML file discovery and hardware detection
- **Build Dependencies Added**: Added `phf_codegen = "0.11"` to build-dependencies for static command registry generation
- **Feature Flags Configured**:
  - `static_commands = []` - enables PHF-based static command registry (phf already available)
  - `multi_yaml = ["walkdir"]` - enables YAML file discovery and processing
  - `advanced_benchmarks = ["benchmarks", "sysinfo", "static_commands", "multi_yaml"]` - enables comprehensive benchmarking with hardware detection
- **Full Feature Integration**: Updated `full` feature to include all new functionality
- **Compilation Verified**: All dependencies compile successfully with zero warnings
- **Tests Passing**: All 277 tests pass with new dependencies enabled