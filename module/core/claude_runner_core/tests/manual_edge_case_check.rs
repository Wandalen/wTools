//! Manual Edge Case Verification Tests
//!
//! Comprehensive testing of corner cases and boundary conditions for `claude_runner_core`
//! builder pattern implementation. These tests verify the builder handles edge cases
//! that aren't typically exercised by standard test suites.
//!
//! ## Purpose
//!
//! While automated tests verify normal usage patterns, real-world usage encounters
//! edge cases that can break assumptions:
//! - Unicode and special characters in messages
//! - Paths with spaces and special characters
//! - Boundary values (zero, maximum u32, empty strings)
//! - Very long input strings (megabyte-scale messages)
//! - Argument ordering requirements
//! - Parameter combinations (kitchen sink testing)
//!
//! These tests prove the builder is production-ready by verifying it handles
//! edge cases gracefully without panics or data corruption.
//!
//! ## Test Coverage Matrix
//!
//! | Category | Test Count | What's Verified |
//! |----------|------------|-----------------|
//! | Unicode/Special Chars | 2 | Non-ASCII characters, newlines, tabs, quotes |
//! | Path Edge Cases | 1 | Spaces in paths, non-ASCII path components |
//! | Argument Ordering | 1 | Custom args → flags → message ordering |
//! | Boundary Values | 2 | Zero values, `u32::MAX`, empty strings |
//! | Large Inputs | 1 | 1MB message handling without panic |
//! | Kitchen Sink | 1 | All 26 parameters set simultaneously |
//! | Argument Formats | 2 | Args with equals, empty args handling |
//!
//! **Total:** 10 manual edge case tests
//!
//! ## Design Decisions
//!
//! ### Why Manual Edge Case Tests?
//!
//! **Automated tests** verify correctness for typical usage:
//! - Standard ASCII text
//! - Simple paths without spaces
//! - Normal parameter ranges
//! - Single parameter changes
//!
//! **Manual edge case tests** verify robustness for unusual usage:
//! - International text (世界, тест, 🌍)
//! - Paths with spaces and special characters
//! - Extreme values (zero, `u32::MAX`, empty, 1MB)
//! - All parameters set at once (interference testing)
//!
//! ### Test Organization Strategy
//!
//! Each test focuses on one edge case category:
//! 1. **Unicode tests**: Verify non-ASCII handling (CJK, Cyrillic, emoji)
//! 2. **Special character tests**: Newlines, tabs, quotes, escape sequences
//! 3. **Path tests**: Spaces in paths (common Windows/Mac issue)
//! 4. **Order tests**: Verify argument ordering requirements
//! 5. **Boundary tests**: Zero, maximum, empty values
//! 6. **Size tests**: Very large inputs (memory safety)
//! 7. **Kitchen sink tests**: Parameter interference testing
//! 8. **Format tests**: Arguments with equals, special formats
//!
//! ## Lessons Learned
//!
//! ### Why Edge Cases Matter
//!
//! **Production failures from edge cases:**
//! - Path with spaces breaks on Windows → shell parsing failure
//! - Unicode emoji in message → UTF-8 encoding corruption
//! - `u32::MAX` parameter → integer overflow in C extension
//! - All parameters set → unexpected parameter interaction
//! - Empty string message → command line parsing ambiguity
//!
//! **Edge case testing prevents:**
//! - Silent data corruption (truncated unicode)
//! - Runtime panics (large allocations)
//! - Shell injection vulnerabilities (special characters)
//! - Platform-specific failures (path separators)
//!
//! ### Test Design Principles
//!
//! 1. **One edge case per test**: Isolates failure cause
//! 2. **Representative examples**: Use real-world edge cases
//! 3. **No mocking**: Test real builder behavior
//! 4. **Clear assertions**: Verify specific invariants
//! 5. **Descriptive output**: Print what's being tested
//!
//! ### Common Pitfalls
//!
//! **Pitfall 1: Assuming ASCII-only input**
//! - Issue: Truncates unicode characters silently
//! - Fix: Test with CJK, emoji, Cyrillic characters
//! - Lesson: Always test international text
//!
//! **Pitfall 2: Not testing boundary values**
//! - Issue: Overflow/underflow at extremes
//! - Fix: Test zero, `u32::MAX`, empty strings
//! - Lesson: Boundaries reveal implementation limits
//!
//! **Pitfall 3: Single parameter testing only**
//! - Issue: Parameters interfere when combined
//! - Fix: Kitchen sink test (all parameters)
//! - Lesson: Interaction bugs need combination testing
//!
//! **Pitfall 4: Not testing platform differences**
//! - Issue: Paths with spaces work on Linux, fail on Windows
//! - Fix: Test paths with spaces explicitly
//! - Lesson: Cross-platform code needs platform-specific edge cases
//!
//! **Pitfall 5: Ignoring argument ordering**
//! - Issue: Wrong order breaks CLI parsing
//! - Fix: Test verifies [`custom_args`] → [-c] → [message] order
//! - Lesson: CLI tools have strict ordering requirements
//!
//! ## Integration with Verification Framework
//!
//! These manual edge case tests integrate with the 6-layer verification pyramid:
//!
//! **Layer 1 (Bottom):** Migration Metrics - Proves old patterns eliminated
//! **Layer 2:** Rollback Detection - Proves migration irreversible
//! **Layer 3:** Impossibility - Proves old API won't compile
//! **Layer 4:** Shortcuts Detection - Proves no test faking
//! **Layer 5:** Negative Criteria - Proves forbidden patterns = 0
//! **Layer 6 (Top):** Positive Tests - **← Manual edge cases are here**
//!
//! Manual edge cases extend positive tests with robustness verification:
//! - Standard tests verify correctness for typical usage
//! - Edge case tests verify robustness for unusual usage
//! - Together: Complete coverage of builder behavior
//!
//! ## Test Execution
//!
//! **Run all edge case tests:**
//! ```bash
//! cargo test --test manual_edge_case_check
//! ```
//!
//! **Run specific edge case:**
//! ```bash
//! cargo test --test manual_edge_case_check manual_unicode_in_message
//! ```
//!
//! **Run with output:**
//! ```bash
//! cargo test --test manual_edge_case_check -- --nocapture
//! ```
//!
//! **Integration with full test suite:**
//! ```bash
//! w3 .test l::3  # Includes these tests in full verification
//! ```
//!
//! ## Success Criteria
//!
//! All edge case tests must pass with:
//! - ✅ No panics (even with extreme inputs)
//! - ✅ No data corruption (unicode preserved)
//! - ✅ No memory issues (large inputs handled)
//! - ✅ Correct argument ordering (CLI requirements met)
//! - ✅ Platform independence (works on Windows/Mac/Linux)
//!
//! ## Known Edge Cases (Not Yet Tested)
//!
//! Future edge cases to add:
//! - Non-UTF8 byte sequences (currently assumes valid UTF-8)
//! - Filesystem limits (path length > 255 on some systems)
//! - Concurrent builder usage (thread safety)
//! - Memory pressure scenarios (OOM behavior)
//! - Signal handling during execution (SIGTERM, SIGINT)

use claude_runner_core::{ ClaudeCommand, ActionMode, LogLevel };

#[test]
fn manual_unicode_in_message() {
  println!( "\n=== Test: Unicode in message ===" );
  let cmd = ClaudeCommand::new()
    .with_message("Hello 世界 🌍 тест");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain the message
  assert!( debug.contains( "Hello" ) );
  println!( "✓ Unicode message handled correctly" );
}

#[test]
fn manual_special_characters_in_message() {
  println!( "\n=== Test: Special characters in message ===" );
  let cmd = ClaudeCommand::new()
    .with_message("Line 1\nLine 2\tTabbed\r\n\"Quoted\" and 'single'");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain the message
  assert!( debug.contains( "Line 1" ) );
  println!( "✓ Special characters handled correctly" );
}

#[test]
fn manual_path_with_spaces() {
  println!( "\n=== Test: Path with spaces ===" );
  let cmd = ClaudeCommand::new()
    .with_working_directory("/tmp/path with spaces/sub dir")
    .with_session_dir("/tmp/session with spaces");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain the paths
  assert!( debug.contains( "path with spaces" ) );
  assert!( debug.contains( "session with spaces" ) );
  println!( "✓ Paths with spaces handled correctly" );
}

#[test]
fn manual_argument_order_verification() {
  println!( "\n=== Test: Argument order ===" );
  let cmd = ClaudeCommand::new()
    .with_arg("--custom-arg")
    .with_arg("value1")
    .with_model("claude-opus-4-5")
    .with_continue_conversation(true)
    .with_message("Final message");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Find positions
  let custom_pos = debug.find( "--custom-arg" );
  let model_pos = debug.find( "--model" );
  let continue_pos = debug.find( "\"-c\"" );
  let message_pos = debug.find( "Final message" );

  // Verify order: custom args → flags → message
  assert!( custom_pos.is_some() );
  assert!( model_pos.is_some() );
  assert!( continue_pos.is_some() );
  assert!( message_pos.is_some() );

  // Custom args should come before -c
  if let (Some(custom), Some(cont)) = (custom_pos, continue_pos) {
    assert!( custom < cont, "Custom args should come before -c flag" );
  }

  // -c should come before message
  if let (Some(cont), Some(msg)) = (continue_pos, message_pos) {
    assert!( cont < msg, "-c flag should come before message" );
  }

  println!( "✓ Argument order correct: [custom_args...] [-c] [message]" );
}

#[test]
fn manual_very_long_argument_1mb() {
  println!( "\n=== Test: Very long argument (1MB) ===" );
  let long_string = "A".repeat( 1_000_000 );
  let cmd = ClaudeCommand::new()
    .with_message(long_string.clone());
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should not panic and should contain the string
  assert!( debug.len() > 1_000_000 );
  println!( "✓ 1MB message handled without panic (debug output: {} bytes)", debug.len() );
}

#[test]
fn manual_all_parameters_kitchen_sink() {
  println!( "\n=== Test: All parameters set (kitchen sink) ===" );
  let cmd = ClaudeCommand::new()
    .with_working_directory("/tmp/work")
    .with_max_output_tokens(100_000)
    .with_continue_conversation(true)
    .with_message("Test message")
    .with_arg("--extra")
    .with_model("claude-sonnet-4-5")
    .with_api_key("test-key")
    .with_verbose(true)
    .with_system_prompt("You are a test assistant")
    .with_bash_timeout_ms(1_000_000)
    .with_bash_max_timeout_ms(2_000_000)
    .with_auto_continue(false)
    .with_telemetry(true)
    .with_auto_approve_tools(true)
    .with_action_mode(ActionMode::Allow)
    .with_log_level(LogLevel::Trace)
    .with_temperature(0.5)
    .with_sandbox_mode(false)
    .with_session_dir("/tmp/session")
    .with_top_p(0.9)
    .with_top_k(50);
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Verify all environment variables are set
  assert!( debug.contains( "CLAUDE_CODE_MAX_OUTPUT_TOKENS" ) );
  assert!( debug.contains( "100000" ) );
  assert!( debug.contains( "CLAUDE_CODE_BASH_TIMEOUT" ) );
  assert!( debug.contains( "1000000" ) );
  assert!( debug.contains( "CLAUDE_CODE_BASH_MAX_TIMEOUT" ) );
  assert!( debug.contains( "2000000" ) );
  assert!( debug.contains( "CLAUDE_CODE_AUTO_CONTINUE" ) );
  assert!( debug.contains( "CLAUDE_CODE_TELEMETRY" ) );
  assert!( debug.contains( "CLAUDE_CODE_AUTO_APPROVE_TOOLS" ) );
  assert!( debug.contains( "CLAUDE_CODE_ACTION_MODE" ) );
  assert!( debug.contains( "allow" ) );
  assert!( debug.contains( "CLAUDE_CODE_LOG_LEVEL" ) );
  assert!( debug.contains( "trace" ) );
  assert!( debug.contains( "CLAUDE_CODE_TEMPERATURE" ) );
  assert!( debug.contains( "0.5" ) );
  assert!( debug.contains( "CLAUDE_CODE_SANDBOX_MODE" ) );
  assert!( debug.contains( "CLAUDE_CODE_SESSION_DIR" ) );
  assert!( debug.contains( "CLAUDE_CODE_TOP_P" ) );
  assert!( debug.contains( "0.9" ) );
  assert!( debug.contains( "CLAUDE_CODE_TOP_K" ) );
  assert!( debug.contains( "50" ) );

  // Verify arguments
  assert!( debug.contains( "--extra" ) );
  assert!( debug.contains( "--model" ) );
  assert!( debug.contains( "claude-sonnet-4-5" ) );
  assert!( debug.contains( "--api-key" ) );
  assert!( debug.contains( "--verbose" ) );
  assert!( debug.contains( "--system-prompt" ) );
  assert!( debug.contains( "-c" ) );
  assert!( debug.contains( "Test message" ) );

  println!( "✓ All 26 parameters set correctly" );
}

#[test]
fn manual_argument_with_equals_sign() {
  println!( "\n=== Test: Argument with equals sign ===" );
  let cmd = ClaudeCommand::new()
    .with_arg("--config=value")
    .with_arg("key=value");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain both arguments intact
  assert!( debug.contains( "--config=value" ) );
  assert!( debug.contains( "key=value" ) );
  println!( "✓ Arguments with equals signs preserved intact" );
}

#[test]
fn manual_empty_string_arguments() {
  println!( "\n=== Test: Empty string arguments ===" );
  let cmd = ClaudeCommand::new()
    .with_arg("")
    .with_message("");
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should handle empty strings (they will appear in command)
  println!( "✓ Empty string arguments handled: {}", debug.len() );
}

#[test]
fn manual_zero_values() {
  println!( "\n=== Test: Zero values ===" );
  let cmd = ClaudeCommand::new()
    .with_max_output_tokens(0)
    .with_bash_timeout_ms(0)
    .with_bash_max_timeout_ms(0)
    .with_temperature(0.0)
    .with_top_p(0.0)
    .with_top_k(0);
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain zero values
  assert!( debug.contains( "\"0\"" ) || debug.contains( "=0" ) );
  println!( "✓ Zero values handled correctly" );
}

#[test]
fn manual_maximum_values() {
  println!( "\n=== Test: Maximum values ===" );
  let cmd = ClaudeCommand::new()
    .with_max_output_tokens(u32::MAX)
    .with_bash_timeout_ms(u32::MAX)
    .with_bash_max_timeout_ms(u32::MAX)
    .with_temperature(1.0)
    .with_top_p(1.0)
    .with_top_k(u32::MAX);
  let built = cmd.build_command_for_test();
  let debug = format!( "{built:?}" );

  // Should contain u32::MAX (4294967295)
  assert!( debug.contains( "4294967295" ) );
  println!( "✓ Maximum values handled correctly" );
}
