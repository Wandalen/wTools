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

-   [ ] All files from `benchmarks/` directory moved to appropriate standard directories (`tests/`, `examples/`, `src/bin/`)
-   [ ] `benchmarks/` directory completely removed
-   [ ] All moved benchmark files compile and execute correctly in their new locations
-   [ ] Cargo.toml [[bench]] sections updated to reflect new file locations
-   [ ] Documentation updated to reference new benchmark locations
-   [ ] No benchkit runtime warnings when executing benchmarks