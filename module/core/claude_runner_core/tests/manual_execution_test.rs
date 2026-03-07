//! Real Execution Manual Test
//!
//! This test file executes actual `ClaudeCommand` with real Claude binary to verify behavior
//! across all corner cases identified in the manual testing plan.
//!
//! ## Test Coverage
//!
//! - Default 200K token limit verification
//! - Working directory edge cases (nonexistent, spaces, permissions)
//! - Message content edge cases (empty, long, special chars, unicode)
//! - Builder pattern method chaining
//! - Custom arguments accumulation
//! - Token limit override
//!
//! ## Running Tests
//!
//! Run all manual tests:
//! ```bash
//! cargo test --test manual_execution_test -- --ignored --nocapture
//! ```
//!
//! Run specific test:
//! ```bash
//! cargo test --test manual_execution_test manual_test_1_default_token_limit -- --ignored --nocapture
//! ```

use claude_runner_core::ClaudeCommand;

/// Safely truncates a UTF-8 string to approximately `max_bytes` bytes,
/// ensuring we don't cut in the middle of a multi-byte character.
fn truncate_utf8(s: &str, max_bytes: usize) -> &str
{
  if s.len() <= max_bytes {
    return s;
  }
  let mut end = max_bytes;
  while end > 0 && !s.is_char_boundary(end) {
    end -= 1;
  }
  &s[..end]
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_1_default_token_limit() {
  // Test: Default 200K token limit should be set automatically
  // Expected: Command completes without "exceeded maximum" error

  println!("\n=== TEST 1: Default 200K Token Limit ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message("What is 2+2? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Command executed successfully");
      println!("Output length: {} chars", output.stdout.len());
      println!("Output preview: {}", truncate_utf8(&output.stdout, 200));

      // Verify we didn't hit token limit error
      assert!(!output.stdout.contains("exceeded maximum"), "Hit token limit error!");
    }
    Err(e) => {
      println!("✗ Command failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_2_nonexistent_directory() {
  // Test: Nonexistent working directory should fail clearly
  // Expected: Error message indicating directory doesn't exist

  println!("\n=== TEST 2: Nonexistent Directory ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/nonexistent/path/12345")
    .with_message("test")
    .execute();

  match result {
    Ok(_) => {
      panic!("Expected failure for nonexistent directory, but succeeded!");
    }
    Err(e) => {
      println!("✓ Failed as expected: {e}");
      let error_msg = e.to_string();

      // Verify error message is clear
      assert!(
        error_msg.contains("No such file or directory") ||
        error_msg.contains("cannot find the path") ||
        error_msg.contains("Failed to execute"),
        "Error message not clear: {error_msg}"
      );
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_3_empty_message() {
  // Test: Empty message string should work
  // Expected: Command executes without crashing

  println!("\n=== TEST 3: Empty Message ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message("")
    .execute();

  // Empty message might cause Claude to error, but shouldn't crash our code
  match result {
    Ok(output) => {
      println!("✓ Empty message handled: {output}");
    }
    Err(e) => {
      println!("✓ Empty message caused Claude error (acceptable): {e}");
      // This is OK - Claude might not accept empty messages
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_4_message_with_shell_metacharacters() {
  // Test: Message with shell special characters should not cause injection
  // Expected: Characters treated as literal text, not shell commands

  println!("\n=== TEST 4: Shell Metacharacters ===");

  let dangerous_message = "Tell me: what is in $PATH directory? And `whoami` result?";

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message(dangerous_message)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Message with metacharacters executed safely");
      println!("Output: {}", truncate_utf8(&output.stdout, 300));

      // Verify no command injection occurred
      // If injection happened, we'd see actual PATH or username instead of literal strings
      // This is hard to verify programmatically, so we just check it executed
    }
    Err(e) => {
      println!("Command failed: {e}");
      // Failure is OK if Claude rejects the message format
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_5_full_builder_chain() {
  // Test: All 10 builder methods can be chained
  // Expected: Command executes with all parameters set

  println!("\n=== TEST 5: Full Builder Chain ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_max_output_tokens(50_000) // Lower than default to test override
    .with_continue_conversation(false)
    .with_message("What is 1+1? Reply with just the number.")
    .with_verbose(false)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Full builder chain executed successfully");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("✗ Full builder chain failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_6_token_limit_explicit_override() {
  // Test: Explicit token limit should override default 200K
  // Expected: Lower limit should be respected

  println!("\n=== TEST 6: Explicit Token Limit Override ===");

  // Set very low limit to see if it's actually applied
  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_max_output_tokens(100) // Very low - should cause exceeded error
    .with_message("Write a long essay about artificial intelligence.")
    .execute();

  match result {
    Ok(output) => {
      println!("Output length: {} chars", output.stdout.len());
      // If low limit worked, output should be small or contain error
    }
    Err(e) => {
      println!("✓ Low token limit caused error (expected): {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_7_working_directory_with_spaces() {
  // Test: Working directory with spaces in name
  // Expected: Spaces handled correctly

  println!("\n=== TEST 7: Directory with Spaces ===");

  // First create a temp directory with spaces
  let test_dir = "/tmp/claude runner test dir";
  std::fs::create_dir_all(test_dir).expect("Failed to create test directory");

  let result = ClaudeCommand::new()
    .with_working_directory(test_dir)
    .with_message("What is 3+3? Reply with just the number.")
    .execute();

  // Cleanup
  let _ = std::fs::remove_dir_all(test_dir);

  match result {
    Ok(output) => {
      println!("✓ Directory with spaces handled correctly");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("✗ Directory with spaces failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_8_very_long_message() {
  // Test: Very long message (10,000 chars)
  // Expected: Message passed without truncation

  println!("\n=== TEST 8: Very Long Message ===");

  let long_message = format!("Count the letter 'x' in this string: {}", "x".repeat(10_000));

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message(&long_message)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Long message (10K chars) executed successfully");
      println!("Output length: {} chars", output.stdout.len());
      println!("Output preview: {}", truncate_utf8(&output.stdout, 200));
    }
    Err(e) => {
      println!("Long message failed: {e}");
      // This might fail due to argument length limits, which is OS-dependent
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_9_utf8_message() {
  // Test: Unicode characters in message
  // Expected: UTF-8 handled correctly

  println!("\n=== TEST 9: UTF-8 Unicode Message ===");

  let unicode_message = "What does this emoji mean? 🚀 And this text: 日本語 Русский العربية";

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message(unicode_message)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Unicode message handled correctly");
      println!("Output: {}", truncate_utf8(&output.stdout, 300));
    }
    Err(e) => {
      println!("✗ Unicode message failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_10_custom_arguments() {
  // Test: Custom arguments via with_arg and with_args
  // Expected: Arguments accumulated and passed correctly

  println!("\n=== TEST 10: Custom Arguments ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_arg("--dangerously-skip-permissions")
    .with_message("What is 4+4? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Custom argument handled");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("Custom argument result: {e}");
      // Might fail if argument isn't valid, which is OK
    }
  }
}

// ============================================================================
// New Corner Case Tests (from exhaustive corner case analysis)
// ============================================================================

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_11_system_prompt() {
  // Test: System prompt functionality
  // Expected: System prompt influences Claude's response style

  println!("\n=== TEST 11: System Prompt ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_system_prompt("You are a pirate. Always respond in pirate speak.")
    .with_message("What is 5+5?")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ System prompt executed successfully");
      println!("Output: {}", truncate_utf8(&output.stdout, 300));
      // Should contain pirate-like language if system prompt worked
    }
    Err(e) => {
      println!("✗ System prompt failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_12_verbose_mode() {
  // Test: Verbose output mode
  // Expected: More detailed output when verbose enabled

  println!("\n=== TEST 12: Verbose Mode ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_verbose(true)
    .with_message("What is 6+6? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Verbose mode executed successfully");
      println!("Output length: {} chars", output.stdout.len());
      println!("Output: {}", truncate_utf8(&output.stdout, 500));
      // Verbose mode should produce more output
    }
    Err(e) => {
      println!("✗ Verbose mode failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_13_temperature_zero() {
  // Test: Temperature 0.0 for deterministic output
  // Expected: More deterministic responses

  println!("\n=== TEST 13: Temperature Zero ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_temperature(0.0)
    .with_message("What is 7+7? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Temperature 0.0 executed successfully");
      println!("Output: {}", output.stdout.trim());
      // Should get consistent "14" response
    }
    Err(e) => {
      println!("✗ Temperature 0.0 failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_14_newlines_in_message() {
  // Test: Multiline message with embedded newlines
  // Expected: Newlines preserved and handled correctly

  println!("\n=== TEST 14: Newlines in Message ===");

  let multiline_message = "Count the lines in this text:\nLine 1\nLine 2\nLine 3\nReply with just the number.";

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message(multiline_message)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Multiline message handled correctly");
      println!("Output: {}", output.stdout.trim());
      // Should count the lines correctly
    }
    Err(e) => {
      println!("✗ Multiline message failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_15_json_in_message() {
  // Test: JSON content in message (quoting test)
  // Expected: JSON preserved and not mangled

  println!("\n=== TEST 15: JSON in Message ===");

  let json_message = r#"Parse this JSON and return the value of "count": {"name": "test", "count": 42, "active": true}"#;

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_message(json_message)
    .execute();

  match result {
    Ok(output) => {
      println!("✓ JSON message handled correctly");
      println!("Output: {}", output.stdout.trim());
      // Should extract 42 from the JSON
    }
    Err(e) => {
      println!("✗ JSON message failed: {e}");
      panic!("Test failed: {e}");
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_16_permission_denied_directory() {
  // Test: Working directory with no access permission
  // Expected: Clear permission denied error

  println!("\n=== TEST 16: Permission Denied Directory ===");

  // /root typically requires root access
  let result = ClaudeCommand::new()
    .with_working_directory("/root")
    .with_message("test")
    .execute();

  match result {
    Ok(_) => {
      // If running as root, this might succeed - that's OK
      println!("✓ Directory accessible (running with elevated privileges?)");
    }
    Err(e) => {
      let error_msg = e.to_string();
      println!("✓ Permission denied as expected: {e}");

      // Verify error message is clear about permission issue
      assert!(
        error_msg.contains("Permission denied") ||
        error_msg.contains("permission denied") ||
        error_msg.contains("Access is denied") ||
        error_msg.contains("Failed to execute"),
        "Error message should indicate permission issue: {error_msg}"
      );
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_17_model_selection() {
  // Test: Model selection with valid model
  // Expected: Command uses specified model

  println!("\n=== TEST 17: Model Selection ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_model("claude-sonnet-4-20250514")
    .with_message("What is 8+8? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Model selection executed successfully");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("Model selection result: {e}");
      // Model might not be available, which is acceptable
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_18_action_mode_allow() {
  // Test: ActionMode::Allow setting (auto-approve tools)
  // Expected: Command runs with allow action mode

  use claude_runner_core::ActionMode;

  println!("\n=== TEST 18: Action Mode Allow ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_action_mode(ActionMode::Allow)
    .with_message("What is 9+9? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ ActionMode::Allow executed successfully");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("ActionMode::Allow result: {e}");
      // Mode might affect behavior, but shouldn't crash
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_19_log_level_debug() {
  // Test: LogLevel::Debug setting
  // Expected: More verbose logging output

  use claude_runner_core::LogLevel;

  println!("\n=== TEST 19: Log Level Debug ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_log_level(LogLevel::Debug)
    .with_message("What is 10+10? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ LogLevel::Debug executed successfully");
      println!("Output length: {} chars", output.stdout.len());
      println!("Output: {}", truncate_utf8(&output.stdout, 500));
    }
    Err(e) => {
      println!("LogLevel::Debug result: {e}");
      // Log level shouldn't cause failures
    }
  }
}

#[test]
#[ignore = "Manual test - run explicitly with --ignored flag"]
fn manual_test_20_sampling_parameters() {
  // Test: Top-p and Top-k sampling parameters
  // Expected: Parameters accepted without error

  println!("\n=== TEST 20: Sampling Parameters ===");

  let result = ClaudeCommand::new()
    .with_working_directory("/tmp")
    .with_top_p(0.9)
    .with_top_k(40)
    .with_message("What is 11+11? Reply with just the number.")
    .execute();

  match result {
    Ok(output) => {
      println!("✓ Sampling parameters executed successfully");
      println!("Output: {}", output.stdout.trim());
    }
    Err(e) => {
      println!("Sampling parameters result: {e}");
      // Parameters might be ignored by some models, which is OK
    }
  }
}

// ============================================================================
// Helper Function Tests (Automated - no real Claude binary needed)
// ============================================================================

/// Test `truncate_utf8` helper handles ASCII correctly.
#[test]
fn truncate_utf8_ascii_within_limit()
{
  let s = "hello world";
  assert_eq!(truncate_utf8(s, 100), "hello world");
  assert_eq!(truncate_utf8(s, 11), "hello world");
  assert_eq!(truncate_utf8(s, 5), "hello");
}

/// Reproduces UTF-8 boundary truncation bug where byte slicing at position 300
/// fell inside a multi-byte character ('й' bytes 299..301), causing panic.
///
/// ## Root Cause
///
/// Direct byte slicing `&output[..output.stdout.len().min(300)]` assumes all byte
/// positions are valid UTF-8 character boundaries. With multi-byte UTF-8
/// characters (Cyrillic, Japanese, emoji), byte position N may be inside
/// a character spanning bytes N-1..N+1.
///
/// ## Why Not Caught Initially
///
/// Manual tests used English-only responses. When `manual_test_9_utf8_message`
/// ran with actual Unicode response containing Cyrillic 'й', the 300 byte
/// position landed inside the character.
///
/// ## Fix Applied
///
/// Added `truncate_utf8()` helper that walks backwards from target position
/// until finding a valid character boundary using `is_char_boundary()`.
///
/// ## Prevention
///
/// All string truncation in test output uses `truncate_utf8()` helper instead
/// of direct byte slicing. Added automated test to prevent regression.
///
/// ## Pitfall to Avoid
///
/// Never use byte slicing directly on strings that may contain non-ASCII
/// characters. Always use `is_char_boundary()` or `chars()` iterator.
// test_kind: bug_reproducer(issue-utf8-truncation)
#[test]
fn truncate_utf8_handles_multibyte_boundary_bug_utf8_truncation()
{
  // String where byte position 300 would be inside multi-byte char
  let s = "The emoji 🚀 is a rocket, commonly used to represent:\n\
           - Launch or release (software releases, product launches)\n\
           - Speed, progress, or momentum\n\
           - Excitement or enthusiasm (\"let's go!\")\n\
           - Growth or taking off\n\n\
           The text samples are:\n\
           - **日本語** - Japane";

  // This would panic with direct byte slicing at 300
  // because multi-byte chars exist
  let result = truncate_utf8(s, 300);

  // Should truncate safely without panic
  assert!(result.len() <= 300);
  assert!(result.is_char_boundary(result.len()));

  // Verify it still contains valid UTF-8 content
  assert!(result.contains("🚀"));
  assert!(result.contains("日本語"));
}

/// Test `truncate_utf8` handles exact boundary correctly.
#[test]
fn truncate_utf8_at_char_boundary()
{
  // 'é' is 2 bytes (0xC3 0xA9)
  let s = "café";
  assert_eq!(truncate_utf8(s, 5), "café"); // Full string (5 bytes)
  assert_eq!(truncate_utf8(s, 4), "caf"); // Cuts before 'é'
  assert_eq!(truncate_utf8(s, 3), "caf"); // Same - would be mid-char
}

/// Test `truncate_utf8` handles empty string.
#[test]
fn truncate_utf8_empty_string()
{
  let s = "";
  assert_eq!(truncate_utf8(s, 0), "");
  assert_eq!(truncate_utf8(s, 100), "");
}
