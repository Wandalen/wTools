# Implement ergonomic aggregation APIs

## Description

Implement the new ergonomic aggregation APIs that provide simple interfaces for common use cases while preserving complex APIs for advanced scenarios. This involves creating the aggregate_cli! macro for zero-boilerplate static aggregation, CliBuilder for complex scenarios, intelligent mode selection, and conditional module loading. The implementation should work with both compile-time and runtime aggregation while maintaining full backward compatibility. Links to task 052 for test foundation and tasks 048-051 for underlying systems.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- aggregate_cli! procedural macro implemented for simple cases
- CliBuilder implemented for complex scenarios
- Intelligent mode selection and auto-detection
- Backward compatibility maintained with existing APIs
- Conditional module loading with feature flag support
- Error handling and validation for user inputs
- Integration with hybrid registry and multi-YAML build system
- All tests from task 052 pass
- Implementation validated with `ctest1` verification