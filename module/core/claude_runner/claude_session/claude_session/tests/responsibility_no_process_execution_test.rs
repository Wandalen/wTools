//! Responsibility Boundary Test: `claude_session` MUST NOT execute processes
//!
//! # Test Purpose
//!
//! Enforce architectural boundary: `claude_session` provides session storage paths ONLY.
//! It MUST NOT import or use `std::process::Command` for process execution.
//!
//! # Responsibility Split
//!
//! - **`claude_session`** (THIS crate): Session storage paths, continuation detection
//! - **`claude_runner`**: Claude Code process execution, `Command::new("claude")`
//!
//! # Verification Method
//!
//! 1. Grep source code for `use std::process::Command`
//! 2. Grep source code for `Command::new`
//! 3. Both must return zero matches
//!
//! # Test Strategy
//!
//! This is a static analysis test that inspects source files rather than runtime behavior.
//! It runs as a standard integration test, not a compile-fail test.
//!
//! # Failure Scenarios
//!
//! Test FAILS if:
//! - Any file in `claude_session/src/` imports `std::process::Command`
//! - Any file in `claude_session/src/` calls `Command::new()`
//! - Process execution logic leaks into session storage crate
//!
//! # Bug Prevention
//!
//! Prevents:
//! - Mixed responsibilities (storage + execution in same crate)
//! - Duplicate execution points
//! - Boundary violations
//!
//! # Related Tests
//!
//! - `claude_runner/tests/responsibility_single_execution_point_test.rs`: Verifies ONLY `claude_runner` executes
//! - `dream_agent/tests/responsibility_builder_pattern_usage_test.rs`: Verifies `dream_agent` uses builder

use std::path::Path;
use std::process::Command;

#[test]
fn no_std_process_command_import() {
  // Verify: claude_session MUST NOT import std::process::Command
  // Rationale: Session storage crate should NOT execute processes

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  let output = Command::new("grep")
    .args([
      "-r",
      "use std::process::Command",
      src_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);
  let match_count = matches.lines().count();

  assert_eq!(
    match_count, 0,
    "RESPONSIBILITY VIOLATION: claude_session MUST NOT import std::process::Command\n\
     Found {match_count} occurrence(s):\n{matches}\n\
     \n\
     Responsibility Boundary:\n\
     - claude_session: Session storage paths ONLY\n\
     - claude_runner: Process execution ONLY\n\
     \n\
     Fix: Remove std::process::Command import and delegate to claude_runner"
  );
}

#[test]
fn no_command_new_calls() {
  // Verify: claude_session MUST NOT call Command::new()
  // Rationale: Process spawning belongs in claude_runner

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  let output = Command::new("grep")
    .args([
      "-r",
      "Command::new",
      src_dir.to_str().unwrap(),
    ])
    .output()
    .expect("Failed to run grep");

  let matches = String::from_utf8_lossy(&output.stdout);
  let match_count = matches.lines().count();

  assert_eq!(
    match_count, 0,
    "RESPONSIBILITY VIOLATION: claude_session MUST NOT call process Command\n\
     Found {match_count} occurrence(s):\n{matches}\n\
     \n\
     Single Execution Point Rule:\n\
     - Process spawning MUST appear exactly 1x in entire workspace\n\
     - That single occurrence MUST be in claude_runner::execute()\n\
     \n\
     Fix: Remove process calls and use claude_runner crate"
  );
}

#[test]
fn no_process_spawning_logic() {
  // Verify: claude_session MUST NOT contain process spawning logic
  // Rationale: Execution logic belongs in claude_runner

  let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

  // Check for common process spawning patterns
  let patterns = [
    "spawn()",
    "output()",
    "status()",
    ".wait()",
    "ExitStatus",
  ];

  for pattern in patterns {
    let output = Command::new("grep")
      .args([
        "-r",
        pattern,
        src_dir.to_str().unwrap(),
      ])
      .output()
      .expect("Failed to run grep");

    let matches = String::from_utf8_lossy(&output.stdout);
    // Filter out false positives (comments, strings in doc comments)
    let real_matches: Vec<&str> = matches
      .lines()
      .filter(|line| {
        // Ignore doc comments
        !line.contains("//") && !line.contains("/*") && !line.contains("*/")
      })
      .collect();

    let count = real_matches.len();
    let joined = real_matches.join("\n");
    assert!(
      real_matches.is_empty(),
      "RESPONSIBILITY VIOLATION: claude_session contains process spawning pattern '{pattern}'\n\
       Found {count} occurrence(s):\n{joined}\n\
       \n\
       Responsibility Boundary:\n\
       - claude_session: Path resolution, continuation detection\n\
       - claude_runner: Process spawning, output capture, exit codes\n\
       \n\
       Fix: Remove process spawning logic and delegate to claude_runner"
    );
  }
}

#[test]
fn responsibility_documented_in_readme() {
  // Verify: claude_session/readme.md clearly states execution is out of scope
  // Rationale: Documentation must match architectural boundaries

  let readme_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("readme.md");
  let readme_content = std::fs::read_to_string(&readme_path)
    .expect("Failed to read readme.md");

  assert!(
    readme_content.contains("claude_runner"),
    "DOCUMENTATION VIOLATION: readme.md must reference claude_runner for execution\n\
     \n\
     Responsibility Table must show:\n\
     Out of Scope: ❌ Claude Code execution → claude_runner\n\
     \n\
     Fix: Update readme.md with proper responsibility table"
  );

  assert!(
    readme_content.contains("Out of Scope") || readme_content.contains("Out of Scope"),
    "DOCUMENTATION VIOLATION: readme.md must have Out of Scope section\n\
     \n\
     Fix: Add Out of Scope section listing what claude_session does NOT do"
  );
}
