# Fix clippy float_cmp warnings in diagnostics_tools examples

## Description

Fix 2 clippy::float_cmp warnings in diagnostics_tools/examples/005_debug_variants.rs on lines 130 and 158. These warnings occur due to strict comparison of f32/f64 values, which is an anti-pattern that can fail due to floating-point precision issues.

The examples should demonstrate proper floating-point comparison techniques using epsilon-based comparisons or other appropriate methods.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- All clippy::float_cmp warnings in 005_debug_variants.rs are resolved
- Examples demonstrate proper floating-point comparison techniques (epsilon-based or similar)
- Code compiles without warnings when running `cargo clippy --all-targets --all-features -- -D warnings`
- Examples remain educational and demonstrate best practices for floating-point assertions

## Outcomes

Successfully fixed both clippy::float_cmp warnings in diagnostics_tools/examples/005_debug_variants.rs by replacing direct floating-point equality comparisons with proper epsilon-based comparisons:

1. **Line 130**: Replaced `a_dbg_id!( first_result, 2.0, "..." )` with epsilon-based comparison using `a_dbg_true!( (first_result - expected_first).abs() < epsilon, "..." )` and added diagnostic output showing expected vs actual values and difference.

2. **Line 158**: Replaced `a_dbg_id!( result, step2, "..." )` with epsilon-based comparison using `a_dbg_true!( (result - step2).abs() < epsilon, "..." )` and added diagnostic output for debugging purposes.

**Key improvements:**
- Both clippy::float_cmp warnings eliminated
- Examples now demonstrate proper floating-point comparison best practices using epsilon tolerance (1e-10)
- Added educational value by showing expected vs actual values and differences
- Maintains debug variant functionality with enhanced output
- Examples remain fully functional for educational purposes

**Educational benefits:**
- Teaches proper floating-point comparison techniques
- Demonstrates epsilon-based tolerance for floating-point arithmetic
- Shows how to debug floating-point calculations effectively
- Provides clear output for troubleshooting numerical issues

**Verification:**
- `cargo clippy --example 005_debug_variants --all-features -- -D warnings` passes ✅
- `cargo run --example 005_debug_variants` works correctly ✅
- All unit tests continue to pass ✅
- Debug output shows proper floating-point comparison results ✅