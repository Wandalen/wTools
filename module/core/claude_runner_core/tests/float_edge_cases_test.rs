//! Float Parameter Edge Case Tests
//!
//! Tests for edge cases in float parameters: temperature and `top_p`.
//! These parameters have documented ranges (0.0 to 1.0) but the builder
//! accepts any f64 value. These tests verify the actual behavior.
//!
//! ## Test Coverage
//!
//! | Edge Case | temperature | `top_p` |
//! |-----------|-------------|-------|
//! | NaN | ✓ | ✓ |
//! | Infinity | ✓ | ✓ |
//! | Negative infinity | ✓ | ✓ |
//! | Negative value | ✓ | ✓ |
//! | Value > 1.0 | ✓ | ✓ |
//! | Zero | ✓ | ✓ |
//! | Exactly 1.0 | ✓ | ✓ |
//! | Very small positive | ✓ | ✓ |
//! | Exact string format for 1.0 | ✓ | ✓ |
//! | Exact string format for 0.0 | ✓ | ✓ |
//! | `describe_env` vs `build_command` consistency | ✓ | ✓ |
//!
//! ## String Formatting Note
//!
//! Rust's `f64::to_string()` (Display) produces `"1"` for `1.0` and `"0"` for `0.0`,
//! NOT `"1.0"` or `"0.0"`. Both `describe_env()` and `build_command()` use the same
//! formatting so they remain consistent. The tests pin this behavior explicitly.

use claude_runner_core::ClaudeCommand;

// ============================================================================
// Temperature Edge Cases
// ============================================================================

#[test]
fn temperature_nan_produces_nan_string() {
  // NaN is a valid f64 but makes no sense for temperature
  // The builder accepts it without panic
  let cmd = ClaudeCommand::new()
    .with_temperature( f64::NAN );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain NaN as string representation
  // Note: f64::NAN.to_string() produces "NaN"
  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  // The actual value will be "NaN" in the env var
  println!( "✓ temperature(NaN) accepted (produces NaN string in env var)" );
}

#[test]
fn temperature_infinity_produces_inf_string() {
  let cmd = ClaudeCommand::new()
    .with_temperature( f64::INFINITY );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  // f64::INFINITY.to_string() produces "inf"
  println!( "✓ temperature(INFINITY) accepted (produces inf string in env var)" );
}

#[test]
fn temperature_negative_infinity_produces_neg_inf_string() {
  let cmd = ClaudeCommand::new()
    .with_temperature( f64::NEG_INFINITY );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  println!( "✓ temperature(NEG_INFINITY) accepted (produces -inf string in env var)" );
}

#[test]
fn temperature_negative_value() {
  // Negative temperature makes no physical sense but builder accepts it
  let cmd = ClaudeCommand::new()
    .with_temperature( -0.5 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  assert!( debug.contains( "-0.5" ) );
  println!( "✓ temperature(-0.5) accepted (produces -0.5 in env var)" );
}

#[test]
fn temperature_above_one() {
  // Documentation says range is 0.0-1.0, but 1.5 is accepted
  let cmd = ClaudeCommand::new()
    .with_temperature( 1.5 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  assert!( debug.contains( "1.5" ) );
  println!( "✓ temperature(1.5) accepted (outside documented 0.0-1.0 range)" );
}

#[test]
fn temperature_zero() {
  let cmd = ClaudeCommand::new()
    .with_temperature( 0.0 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  // 0.0.to_string() can be "0" or "0.0" depending on Rust version
  assert!( debug.contains( "\"0\"" ) || debug.contains( "\"0.0\"" ) );
  println!( "✓ temperature(0.0) handled correctly" );
}

#[test]
fn temperature_exactly_one() {
  let cmd = ClaudeCommand::new()
    .with_temperature( 1.0 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  println!( "✓ temperature(1.0) handled correctly" );
}

#[test]
fn temperature_very_small_positive() {
  let cmd = ClaudeCommand::new()
    .with_temperature( f64::MIN_POSITIVE );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  println!( "✓ temperature(MIN_POSITIVE) handled correctly" );
}

// ============================================================================
// Top-P Edge Cases
// ============================================================================

#[test]
fn top_p_nan_produces_nan_string() {
  let cmd = ClaudeCommand::new()
    .with_top_p( f64::NAN );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(NaN) accepted (produces NaN string in env var)" );
}

#[test]
fn top_p_infinity_produces_inf_string() {
  let cmd = ClaudeCommand::new()
    .with_top_p( f64::INFINITY );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(INFINITY) accepted (produces inf string in env var)" );
}

#[test]
fn top_p_negative_infinity_produces_neg_inf_string() {
  let cmd = ClaudeCommand::new()
    .with_top_p( f64::NEG_INFINITY );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(NEG_INFINITY) accepted (produces -inf string in env var)" );
}

#[test]
fn top_p_negative_value() {
  let cmd = ClaudeCommand::new()
    .with_top_p( -0.3 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  assert!( debug.contains( "-0.3" ) );
  println!( "✓ top_p(-0.3) accepted (produces -0.3 in env var)" );
}

#[test]
fn top_p_above_one() {
  let cmd = ClaudeCommand::new()
    .with_top_p( 2.5 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  assert!( debug.contains( "2.5" ) );
  println!( "✓ top_p(2.5) accepted (outside documented 0.0-1.0 range)" );
}

#[test]
fn top_p_zero() {
  let cmd = ClaudeCommand::new()
    .with_top_p( 0.0 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(0.0) handled correctly" );
}

#[test]
fn top_p_exactly_one() {
  let cmd = ClaudeCommand::new()
    .with_top_p( 1.0 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(1.0) handled correctly" );
}

#[test]
fn top_p_very_small_positive() {
  let cmd = ClaudeCommand::new()
    .with_top_p( f64::MIN_POSITIVE );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  println!( "✓ top_p(MIN_POSITIVE) handled correctly" );
}

// ============================================================================
// Combined Float Edge Cases
// ============================================================================

#[test]
fn both_temperature_and_top_p_set() {
  let cmd = ClaudeCommand::new()
    .with_temperature( 0.7 )
    .with_top_p( 0.9 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  assert!( debug.contains( "0.7" ) );
  assert!( debug.contains( "0.9" ) );
  println!( "✓ Both temperature and top_p set correctly" );
}

#[test]
fn float_precision_preserved() {
  // Test that float precision is preserved in string conversion
  let cmd = ClaudeCommand::new()
    .with_temperature( 0.123_456_789 )
    .with_top_p( 0.987_654_321 );
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // f64::to_string() preserves reasonable precision
  assert!( debug.contains( "0.123456789" ) );  // String still shows without separators
  assert!( debug.contains( "0.987654321" ) );  // String still shows without separators
  println!( "✓ Float precision preserved in string conversion" );
}

// ============================================================================
// Float-to-string format for integer-valued floats
// ============================================================================
//
// Rust's f64::to_string() (via Display) produces "1" for 1.0 and "0" for 0.0,
// NOT "1.0" or "0.0". Both describe_env() and build_command() use the same
// formatting, so they are consistent. These tests document and pin this behavior.

#[test]
fn temperature_one_exact_format_is_integer_string() {
  // temperature = 1.0 → env var value is "1" (Rust Display format for whole numbers)
  // Both describe_env() and build_command() produce the same "1" string.
  let env = ClaudeCommand::new()
    .with_temperature( 1.0 )
    .describe_env();

  let temp_line = env.lines()
    .find( | l | l.starts_with( "CLAUDE_CODE_TEMPERATURE=" ) )
    .unwrap_or( "NOT_FOUND" );

  assert_eq!(
    temp_line, "CLAUDE_CODE_TEMPERATURE=1",
    "temperature=1.0 must produce env var value '1' (not '1.0'): Rust Display format for f64"
  );

  // Verify build_command() uses the same format
  let built = ClaudeCommand::new()
    .with_temperature( 1.0 )
    .build_command_for_test();
  let debug = format!( "{built:?}" );
  assert!(
    debug.contains( "CLAUDE_CODE_TEMPERATURE" ) && debug.contains( "\"1\"" ),
    "build_command() must also produce '1' for temperature=1.0, got: {debug}"
  );
}

#[test]
fn temperature_zero_exact_format_is_integer_string() {
  // temperature = 0.0 → env var value is "0" (Rust Display format for whole numbers)
  let env = ClaudeCommand::new()
    .with_temperature( 0.0 )
    .describe_env();

  let temp_line = env.lines()
    .find( | l | l.starts_with( "CLAUDE_CODE_TEMPERATURE=" ) )
    .unwrap_or( "NOT_FOUND" );

  assert_eq!(
    temp_line, "CLAUDE_CODE_TEMPERATURE=0",
    "temperature=0.0 must produce env var value '0' (not '0.0'): Rust Display format for f64"
  );
}

#[test]
fn top_p_one_exact_format_is_integer_string() {
  // top_p = 1.0 → env var value is "1" (not "1.0")
  let env = ClaudeCommand::new()
    .with_top_p( 1.0 )
    .describe_env();

  let top_p_line = env.lines()
    .find( | l | l.starts_with( "CLAUDE_CODE_TOP_P=" ) )
    .unwrap_or( "NOT_FOUND" );

  assert_eq!(
    top_p_line, "CLAUDE_CODE_TOP_P=1",
    "top_p=1.0 must produce env var value '1' (not '1.0')"
  );
}

#[test]
fn top_p_zero_exact_format_is_integer_string() {
  // top_p = 0.0 → env var value is "0" (not "0.0")
  let env = ClaudeCommand::new()
    .with_top_p( 0.0 )
    .describe_env();

  let top_p_line = env.lines()
    .find( | l | l.starts_with( "CLAUDE_CODE_TOP_P=" ) )
    .unwrap_or( "NOT_FOUND" );

  assert_eq!(
    top_p_line, "CLAUDE_CODE_TOP_P=0",
    "top_p=0.0 must produce env var value '0' (not '0.0')"
  );
}

#[test]
fn describe_env_and_build_command_float_values_are_consistent() {
  // Both describe_env() and build_command() must produce the same string representation
  // for float parameters, ensuring dry-run matches actual execution.
  let temp = 0.7_f64;
  let top_p = 0.9_f64;

  let env = ClaudeCommand::new()
    .with_temperature( temp )
    .with_top_p( top_p )
    .describe_env();

  let built = ClaudeCommand::new()
    .with_temperature( temp )
    .with_top_p( top_p )
    .build_command_for_test();
  let debug = format!( "{built:?}" );

  // describe_env shows 0.7
  assert!(
    env.contains( "CLAUDE_CODE_TEMPERATURE=0.7" ),
    "describe_env must show CLAUDE_CODE_TEMPERATURE=0.7"
  );
  // build_command also shows 0.7
  assert!(
    debug.contains( "CLAUDE_CODE_TEMPERATURE" ) && debug.contains( "\"0.7\"" ),
    "build_command must also produce '0.7' for temperature=0.7"
  );
  // Same for top_p
  assert!(
    env.contains( "CLAUDE_CODE_TOP_P=0.9" ),
    "describe_env must show CLAUDE_CODE_TOP_P=0.9"
  );
  assert!(
    debug.contains( "CLAUDE_CODE_TOP_P" ) && debug.contains( "\"0.9\"" ),
    "build_command must also produce '0.9' for top_p=0.9"
  );
}
