# Write tests for multi-YAML build system

## Description

Write comprehensive tests for the enhanced build system that processes multiple YAML files and combines them at compile-time. This includes testing multi-YAML processing, prefix application during build, conflict detection across modules, Cargo.toml metadata support, and environment variable configuration. The tests should validate the zero runtime overhead aggregation while supporting both dynamic and static scenarios. Links to tasks 048-049 for registry integration.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- Tests for MultiYamlAggregator processing multiple YAML files
- Tests for prefix application during compilation (e.g., .add -> .math.add)
- Tests for conflict detection across modules
- Tests for Cargo.toml metadata parsing and validation
- Tests for environment variable override support
- Tests for PHF map generation with aggregated commands
- Integration tests with hybrid registry from tasks 048-049
- All tests must pass with `ctest1` verification