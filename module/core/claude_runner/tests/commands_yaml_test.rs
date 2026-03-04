//! Static verification: `claude_runner` YAML defines `.claude` and `.claude.help`.
//!
//! # Test Purpose
//!
//! Enforce command boundaries at the source level: the YAML command definitions
//! that `claude_runner` provides must define only the direct AI commands.
//!
//! # Architecture Note
//!
//! `.plan.claude` has moved to wplan's runner plugin system. Planning orchestration
//! (multi-dir execution, `work_dir` expansion) is wplan's responsibility. `claude_runner`
//! provides only the direct AI interaction commands.
//!
//! # Verification Method
//!
//! 1. `COMMANDS_YAML` constant must point to an existing file.
//! 2. YAML content must contain `name: .claude` (primary command).
//! 3. YAML content must NOT contain `name: .please` (deleted command).
//! 4. YAML content must NOT contain `name: .plan.claude` (moved to wplan runner plugin).
//!
//! # Failure Scenarios
//!
//! Tests FAIL if:
//! - `COMMANDS_YAML` path resolves to a missing file (broken constant)
//! - `.please` command name reappears in the YAML (regression)
//! - `.claude` command name disappears from the YAML (accidental deletion)
//! - `.plan.claude` reappears in the YAML (moved to wplan runner plugin, must not be here)

#![ allow( unused_crate_dependencies ) ]

use std::path::Path;

#[ test ]
fn commands_yaml_file_exists()
{
  let path = Path::new( claude_runner::COMMANDS_YAML );
  assert!(
    path.exists(),
    "COMMANDS_YAML points to non-existent file: {}",
    claude_runner::COMMANDS_YAML
  );
}

#[ test ]
fn commands_yaml_defines_claude_primary()
{
  let content = std::fs::read_to_string( claude_runner::COMMANDS_YAML )
    .expect( "Failed to read claude.commands.yaml" );
  // Top-level commands start with `- name:` (sequence entry)
  assert!(
    content.contains( "- name: \".claude\"" ),
    "REGRESSION: `.claude` command missing from claude.commands.yaml\n\
     File: {}\n\
     Fix: Add `- name: \".claude\"` command block to the YAML",
    claude_runner::COMMANDS_YAML
  );
}

#[ test ]
fn commands_yaml_no_please_command()
{
  let content = std::fs::read_to_string( claude_runner::COMMANDS_YAML )
    .expect( "Failed to read claude.commands.yaml" );
  let please_lines : Vec< &str > = content
    .lines()
    .filter( | l | l.contains( "- name: \".please" ) )
    .collect();
  assert!(
    please_lines.is_empty(),
    "RENAME REGRESSION: `.please` command found in claude.commands.yaml\n\
     File: {}\n\
     Offending lines:\n{}\n\
     Fix: Remove `.please` entries — `.claude` is the sole primary command",
    claude_runner::COMMANDS_YAML,
    please_lines.join( "\n" )
  );
}

#[ test ]
fn commands_yaml_no_plan_claude_command()
{
  // `.plan.claude` moved to wplan runner plugin system (`.plan runner::claude`).
  // It must not appear here — claude_runner handles direct AI commands only.
  let content = std::fs::read_to_string( claude_runner::COMMANDS_YAML )
    .expect( "Failed to read claude.commands.yaml" );
  assert!(
    !content.contains( "- name: \".plan.claude\"" ),
    "ARCHITECTURE VIOLATION: `.plan.claude` found in claude.commands.yaml\n\
     File: {}\n\
     Planning orchestration has moved to wplan runner plugin system.\n\
     Fix: Remove `.plan.claude` entry — use `.plan runner::claude` via wplan instead",
    claude_runner::COMMANDS_YAML
  );
}
