//! Responsibility Boundary Test: `claude_runner` MUST be ONLY execution point
//!
//! # Test Purpose
//!
//! Enforce single execution point: `Command::new("claude")` MUST appear exactly once
//! in the entire workspace, and that occurrence MUST be in `claude_runner` crate.
//!
//! # Responsibility Split
//!
//! - **`claude_runner`** (THIS crate): Claude Code process execution (ONLY location)
//! - **`claude_session`**: Session storage paths (NO execution)
//! - **`dream_agent`**: Orchestration (uses `claude_runner`, does NOT execute directly)
//!
//! # Verification Method
//!
//! 1. Grep entire module/ directory for `Command::new\s*\(\s*"claude"\s*\)` (extended regex)
//! 2. Count occurrences (must equal 1)
//! 3. Verify that occurrence is in `claude_runner/src/`
//!
//! **Pattern Specificity**: Uses `\s*\(\s*"claude"\s*\)` to match exactly `Command::new("claude")`
//! with optional whitespace, avoiding false positives from similar binaries like
//! `Command::new("claude_storage")` or `Command::new("../path/to/claude_something")`
//!
//! # Test Strategy
//!
//! This is a static analysis test that inspects source files across entire workspace.
//! It ensures duplication factor = 1x (single execution point).
//!
//! # Failure Scenarios
//!
//! Test FAILS if:
//! - `Command::new("claude")` appears 0 times (missing implementation)
//! - `Command::new("claude")` appears 2+ times (duplication violation)
//! - `Command::new("claude")` appears in `claude_session` (boundary violation)
//! - `Command::new("claude")` appears in `dream_agent` (bypass violation)
//!
//! # Bug Prevention
//!
//! Prevents:
//! - Duplicate execution points (measured baseline: 2x → target: 1x)
//! - Scattered execution logic
//! - Difficulty updating execution behavior
//!
//! # Related Tests
//!
//! - `claude_session/tests/responsibility_no_process_execution_test.rs`: Verifies NO execution in `claude_session`
//! - `dream_agent/tests/responsibility_builder_pattern_usage_test.rs`: Verifies `dream_agent` uses builder

use std::path::Path;
use std::process::Command;

#[test]
fn single_execution_point_in_workspace() {
  // Verify: Command::new("claude") appears exactly ONCE in entire workspace
  // Rationale: Single execution point prevents duplication and centralizes logic

  let module_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("Failed to get parent directory");

  let output = Command::new("grep")
    .args([
      "-rE",
      "--include=*.rs",
      r#"Command::new\s*\(\s*"claude"\s*\)"#,
      module_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);
  let match_lines: Vec<&str> = matches
    .lines()
    .filter(|line| {
      // Filter out comments and test code
      !line.contains("//") && !line.contains("/*") && !line.contains("test")
    })
    .collect();

  let match_count = match_lines.len();

  assert_eq!(
    match_count, 1,
    "SINGLE EXECUTION POINT VIOLATION\n\
     \n\
     Expected: Exactly 1 occurrence of Command::new(\"claude\")\n\
     Found: {match_count} occurrence(s)\n\
     \n\
     Matches:\n{}\n\
     \n\
     Migration Metrics:\n\
     - Baseline: 2x duplication\n\
     - Target: 1x duplication\n\
     - Current: {match_count}x duplication\n\
     \n\
     Single Execution Point Rule:\n\
     - Command::new(\"claude\") MUST appear exactly once\n\
     - That occurrence MUST be in claude_runner/src/executor.rs or similar\n\
     \n\
     Fix:\n\
     - If 0 occurrences: Implement execute() method in claude_runner\n\
     - If 2+ occurrences: Remove duplicate calls, use claude_runner::execute() instead",
    match_lines.join("\n")
  );
}

#[test]
fn execution_point_in_claude_runner_only() {
  // Verify: The single Command::new("claude") is in claude_runner crate
  // Rationale: Execution belongs in claude_runner, not claude_session or dream_agent

  let module_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("Failed to get parent directory");

  let output = Command::new("grep")
    .args([
      "-rE",
      "--include=*.rs",
      r#"Command::new\s*\(\s*"claude"\s*\)"#,
      module_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);
  let match_lines: Vec<&str> = matches
    .lines()
    .filter(|line| !line.contains("//") && !line.contains("/*"))
    .collect();

  if match_lines.is_empty() {
    // No implementation yet - this is expected during migration RED phase
    return;
  }

  // Verify all matches are in claude_runner
  for line in &match_lines {
    assert!(
      line.contains("claude_runner/"),
      "BOUNDARY VIOLATION: Command::new(\"claude\") found outside claude_runner\n\
       \n\
       Found: {line}\n\
       \n\
       Responsibility Boundary:\n\
       - claude_runner: Process execution (ONLY location for Command::new)\n\
       - claude_session: Session storage (NO Command::new)\n\
       - dream_agent: Orchestration (NO Command::new)\n\
       \n\
       Fix: Move execution to claude_runner, use builder pattern from other crates"
    );
  }
}

#[test]
fn no_deprecated_factory_methods() {
  // Verify: Old factory method ClaudeCommand::generate does NOT exist
  // Rationale: Builder pattern replaces factory methods

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  if !src_dir.exists() {
    // No implementation yet - this is expected during migration RED phase
    return;
  }

  let output = Command::new("grep")
    .args([
      "-r",
      "ClaudeCommand::generate",
      src_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);

  // Filter out documentation comments and compile_fail examples
  // These document what NOT to do, which is acceptable
  let match_count = matches
    .lines()
    .filter(|line| {
      !line.contains("//!") &&   // Module-level doc comments
      !line.contains("///") &&   // Item-level doc comments
      !line.contains("//")       // Regular comments
    })
    .count();

  assert_eq!(
    match_count, 0,
    "DEPRECATED API VIOLATION: ClaudeCommand::generate found\n\
     Found {match_count} occurrence(s):\n{matches}\n\
     \n\
     Migration:\n\
     - Old API: ClaudeCommand::generate() [DEPRECATED]\n\
     - New API: ClaudeCommand::new().with_*().execute() [CORRECT]\n\
     \n\
     Fix: Remove generate() factory method, use builder pattern only"
  );
}

#[test]
fn no_deprecated_execution_methods() {
  // Verify: Old execute_non_interactive method does NOT exist
  // Rationale: Only execute() and execute_interactive() are valid execution methods

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  if !src_dir.exists() {
    // No implementation yet - this is expected during migration RED phase
    return;
  }

  // Only check for execute_non_interactive (deprecated)
  // execute_interactive() is a VALID method for TTY-attached interactive sessions
  let pattern = "execute_non_interactive";

  let output = Command::new("grep")
    .args([
      "-r",
      pattern,
      src_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);
  let match_count = matches.lines().count();

  assert_eq!(
    match_count, 0,
    "DEPRECATED API VIOLATION: {pattern} found\n\
     Found {match_count} occurrence(s):\n{matches}\n\
     \n\
     Valid Execution Methods:\n\
     - execute() - Non-interactive mode (captures output)\n\
     - execute_interactive() - Interactive mode (TTY attached)\n\
     \n\
     Deprecated Methods:\n\
     - execute_non_interactive() [REMOVED]\n\
     \n\
     Fix: Remove execute_non_interactive(), use execute() instead"
  );
}

#[test]
fn builder_pattern_api_documented() {
  // Verify: claude_runner/readme.md documents builder pattern usage
  // Rationale: Documentation must show correct API usage

  let readme_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("readme.md");

  assert!(
    readme_path.exists(),
    "DOCUMENTATION MISSING: claude_runner/readme.md not found\n\
     \n\
     Spec Before Code Rule:\n\
     - readme.md must exist BEFORE implementation\n\
     - Must document builder pattern usage\n\
     - Must show ClaudeCommand::new().with_*().execute() examples\n\
     \n\
     Fix: Create readme.md with builder pattern documentation"
  );

  let readme_content = std::fs::read_to_string(&readme_path)
    .expect("Failed to read readme.md");

  assert!(
    readme_content.contains("ClaudeCommand::new()"),
    "DOCUMENTATION VIOLATION: readme.md must document ClaudeCommand::new() builder\n\
     \n\
     Required documentation:\n\
     - ClaudeCommand::new() entry point\n\
     - with_*() method examples\n\
     - execute() terminal method\n\
     \n\
     Fix: Add builder pattern usage examples to readme.md"
  );

  assert!(
    readme_content.contains("with_") || readme_content.contains(".with_"),
    "DOCUMENTATION VIOLATION: readme.md must show with_*() method usage\n\
     \n\
     Builder pattern requires documenting fluent API methods\n\
     \n\
     Fix: Add examples showing method chaining with with_*()"
  );
}

#[test]
fn builder_methods_comprehensive_coverage() {
  // Verify: At least 10 builder methods exist
  //
  // Rationale: Comprehensive builder pattern API should support all major Claude CLI parameters
  // to prevent users from falling back to raw args. Migration plan (Phase 3) targets ≥10 methods.
  //
  // Why 10+ methods:
  // - Covers essential Claude CLI parameters (working_dir, tokens, model, api_key, etc.)
  // - Prevents raw string argument usage (.with_arg() as escape hatch only)
  // - Encourages type-safe configuration over string manipulation
  //
  // Root Cause of Previous Gap:
  // Phase 3 was marked complete with only 6 methods because tests passed (functionality worked).
  // Quantitative target (≥10) was overlooked until ultrathink verification revealed gap.
  //
  // Pitfall: Functional correctness != Plan compliance. Always verify BOTH:
  // - Qualitative success (tests pass, features work)
  // - Quantitative success (numeric targets met per plan)

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  if !src_dir.exists() {
    // No implementation yet - acceptable during RED phase
    return;
  }

  let output = Command::new("grep")
    .args([
      "-c",
      "pub fn with_",
      src_dir.join("command.rs").to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let count_str = String::from_utf8_lossy(&output.stdout);
  let count: usize = count_str.trim().parse().unwrap_or(0);

  assert!(
    count >= 10,
    "BUILDER PATTERN INCOMPLETE: Need ≥10 builder methods, found {count}\n\
     \n\
     Migration Plan Phase 3 Target: ≥10 builder methods\n\
     Current: {count} methods\n\
     Gap: {} methods needed\n\
     \n\
     Essential builder methods:\n\
     1. with_working_directory() - Set working directory\n\
     2. with_max_output_tokens() - Token limit (fixes 32K→200K bug)\n\
     3. with_continue_conversation() - Continuation flag\n\
     4. with_message() - Message content\n\
     5. with_arg() - Single custom argument\n\
     6. with_args() - Multiple custom arguments\n\
     7. with_model() - Model selection\n\
     8. with_api_key() - API authentication\n\
     9. with_verbose() - Verbose output\n\
     10. with_system_prompt() - System prompt customization\n\
     \n\
     Why this matters:\n\
     - Type-safe API prevents string manipulation errors\n\
     - Comprehensive coverage reduces .with_arg() escape hatch usage\n\
     - Plan compliance requires meeting quantitative targets\n\
     \n\
     Fix: Implement missing builder methods in src/command.rs",
    10 - count
  );
}
