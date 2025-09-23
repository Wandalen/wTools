# Fix Benchmarks Directory Structure

## Description

**CRITICAL VIOLATION**: The project uses `benchmarks/` directory structure which is explicitly prohibited by benchkit usage.md. Benchkit **actively discourages** using framework-specific directories like `benchmarks/` and requires using standard Rust directories instead.

**Current State**: 
- `/home/user1/pro/lib/wTools_2/module/move/unilang/benchmarks/` contains 8+ benchmark files
- This violates benchkit's "📁 Why Not `benches/`? Standard Directory Integration" mandatory requirement

**Required Fix**: Move ALL benchmark files to standard directories:
- Performance tests → `tests/` directory 
- Demonstration benchmarks → `examples/` directory
- Benchmark executables → `src/bin/` directory

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md Section "📁 Why Not `benches/`? Standard Directory Integration" - MANDATORY COMPLIANCE
-   Benchkit will show runtime warnings when detecting benchmarks/ directory usage

## Acceptance Criteria

-   [x] All files from `benchmarks/` directory moved to appropriate standard directories (`tests/`, `examples/`, `src/bin/`)
-   [x] `benchmarks/` directory completely removed
-   [x] All moved benchmark files compile and execute correctly in their new locations
-   [x] Cargo.toml [[bench]] sections updated to reflect new file locations
-   [x] Documentation updated to reference new benchmark locations
-   [x] No benchkit runtime warnings when executing benchmarks

## Outcomes

**✅ COMPLETED**: Benchmarks directory structure fixed to comply with benchkit standards.

### Key Deliverables

1. **Directory Migration**:
   - **Performance Tests** → `tests/benchmarks/`:
     - `comprehensive_framework_comparison.rs`
     - `throughput_benchmark.rs`
     - `string_interning_benchmark.rs`
   - **Example Benchmarks** → `examples/benchmarks/`:
     - `integrated_string_interning_benchmark.rs`
     - `simd_json_benchmark.rs`
     - `strs_tools_benchmark.rs`
   - **Test Orchestrator** → `tests/`:
     - `run_all_benchmarks.rs`

2. **Cargo.toml Updates**:
   - Updated `[[bench]]` entries to point to `tests/benchmarks/` locations
   - Converted demonstration benchmarks to `[[example]]` entries
   - Updated test configuration for `run_all_benchmarks`

3. **Directory Structure Compliance**:
   - Completely removed prohibited `benchmarks/` directory
   - All benchmark functionality now uses standard Rust directories
   - Follows benchkit's "📁 Why Not `benches/`? Standard Directory Integration" requirement

4. **Verification**:
   - All 279 tests pass with new structure
   - All benchmark files compile successfully in new locations
   - Examples and benchmarks accessible via standard cargo commands

### Commands After Migration

```bash
# Performance benchmarks (tests/benchmarks/)
cargo bench comprehensive_benchmark --features benchmarks
cargo bench throughput_benchmark --features benchmarks
cargo bench string_interning_benchmark --features benchmarks

# Example benchmarks (examples/benchmarks/)
cargo run --example integrated_string_interning_benchmark --features benchmarks
cargo run --example simd_json_benchmark --features benchmarks
cargo run --example strs_tools_benchmark --features benchmarks

# Test orchestrator (tests/)
cargo test run_all_benchmarks --release --features benchmarks -- --ignored --nocapture
```

### Compliance Achievement

The migration eliminates benchkit runtime warnings about prohibited directory usage and aligns with benchkit's philosophy of standard directory integration rather than framework-specific directories.