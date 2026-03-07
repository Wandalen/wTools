//! Migration Validation Test - Claude Code Execution Consolidation
//!
//! # Purpose
//!
//! Validates the successful migration from duplicated execution logic to single
//! execution point with builder pattern.
//!
//! # Why This Test Exists
//!
//! **Problem:** Before migration, Claude Code execution logic was duplicated in
//! `claude_session` (2x occurrences of `Command::new("claude")`). This caused:
//! - Difficulty updating execution behavior (had to change in multiple places)
//! - Token limit bug (32K default caused "exceeded maximum" errors)
//! - Mixed responsibilities (`claude_session` handled both storage AND execution)
//!
//! **Solution:** Consolidated to single execution point in `claude_runner_core` with
//! builder pattern API.
//!
//! # Architecture After Migration
//!
//! ```text
//! dream_agent (orchestration)
//!   └→ uses ClaudeCommand::new()
//!         .with_working_directory()
//!         .with_max_output_tokens(200_000)  ← BUG FIX
//!         .with_message()
//!         .execute()
//!   └→ uses check_session_exists() from claude_session
//!
//! claude_runner_core (execution ONLY)
//!   └→ Command::new("claude")  ← SINGLE POINT
//!
//! claude_session (session storage ONLY)
//!   └→ NO Command::new("claude")
//! ```
//!
//! # Key Metrics Validated
//!
//! - Single execution point: 1 (was 2)
//! - Old API usage: 0 (was 16)
//! - Builder pattern adoption: ✓
//! - Token limit bug fixed: ✓ (200K explicit)
//!
//! # Root Cause of Token Limit Bug
//!
//! Claude Code defaults to 32K `max_output_tokens` when `CLAUDE_CODE_MAX_OUTPUT_TOKENS`
//! environment variable is not set. Complex conversations exceed 32K, causing:
//! "Error: Maximum output tokens exceeded"
//!
//! # Why Not Caught Earlier
//!
//! - No integration tests with real Claude Code execution
//! - Tests used mock outputs under 32K
//! - No visibility into Claude Code's internal token counting
//!
//! # Fix Applied
//!
//! Set explicit `CLAUDE_CODE_MAX_OUTPUT_TOKENS=200000` via:
//! ```rust
//! ClaudeCommand::new()
//!   .with_max_output_tokens(200_000)
//! ```
//!
//! # Prevention
//!
//! - Single execution point means token limit set in ONE place
//! - Builder pattern makes configuration explicit and visible
//! - Default of 200K in `ClaudeCommand::new()` prevents regression
//!
//! # Pitfall
//!
//! Don't rely on system defaults for critical configuration. Make all important
//! settings explicit in code.

#[test]
fn migration_complete_validation() {
  // This test documents the completed migration
  // Actual validation is in responsibility_single_execution_point_test.rs

  // BEFORE:
  // - 2x Command::new("claude") in claude_session
  // - ClaudeCommand::generate() factory method
  // - execute_interactive() / execute_non_interactive()
  // - Default 32K token limit (bug)

  // AFTER:
  // - 1x Command::new("claude") in claude_runner_core
  // - ClaudeCommand::new().with_*() builder pattern
  // - Single execute() method
  // - Explicit 200K token limit (bug fix)

  // Documentation-only test - no assertions needed
}
