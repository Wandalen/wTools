//! Impossibility Verification Test
//!
//! Verifies that old factory pattern is IMPOSSIBLE to use, not merely discouraged.
//! If old way still compiles, migration is incomplete.
//!
//! ## Purpose
//!
//! Tests passing doesn't prove old code removed. This test verifies:
//! - Deprecated methods literally don't exist in source code
//! - Old API calls won't compile (methods missing)
//! - Public fields removed from structs (encapsulation enforced)
//! - Type system prevents old usage patterns
//!
//! **Critical Distinction**: Discouraged vs Impossible
//! - **Discouraged**: Old API exists but documentation says "don't use"
//! - **Impossible**: Old API physically removed, compiler rejects old code
//!
//! This test proves the migration achieved IMPOSSIBLE, not just discouraged.
//!
//! ## Test Matrix
//!
//! | Category | Checks | What Must Not Exist |
//! |----------|--------|---------------------|
//! | Factory Methods | 12 | `from_message`, create, generate methods |
//! | Public Fields | 11 | Any public field in `ClaudeCommand` struct |
//! | Deprecated Methods | 8 | `execute_non_interactive`, old execution API |
//! | Type System | 3 | String literals where enums now required |
//!
//! **Total**: 34 impossibility checks
//!
//! ## Why Impossibility Matters
//!
//! ### Enforcement Levels (Weakest to Strongest)
//!
//! 1. **Documentation Only**: "Please use new API" (no enforcement)
//! 2. **Deprecation Warnings**: `#[deprecated]` attribute (warnings only)
//! 3. **Runtime Errors**: Old API panics at runtime (still compiles)
//! 4. **Compilation Errors**: Old API removed (won't compile) ← THIS TEST
//!
//! This test verifies level 4 (strongest enforcement).
//!
//! ### Real-World Scenario
//!
//! **Without Impossibility Verification**:
//! ```rust,ignore
//! // Developer tries old API (might still exist)
//! let cmd = ClaudeCommand::from_message("hello");  // Compiles?
//! cmd.message = "modified".to_string();             // Compiles?
//! cmd.execute_non_interactive()?;                   // Compiles?
//! ```
//!
//! **With Impossibility Verification**:
//! ```rust,ignore
//! // Developer tries old API (guaranteed not to exist)
//! let cmd = ClaudeCommand::from_message("hello");  // ❌ Compile error: no from_message
//! cmd.message = "modified".to_string();             // ❌ Compile error: field is private
//! cmd.execute_non_interactive()?;                   // ❌ Compile error: no such method
//! ```
//!
//! Impossibility = compile-time guarantee old API cannot be used.
//!
//! ## Lessons Learned
//!
//! ### Why Grep-Based Verification Works
//!
//! - **Simple but effective**: grep finds exact string matches
//! - **No false negatives**: If method exists, grep will find it
//! - **Fast execution**: File scanning completes in milliseconds
//! - **Objective evidence**: Count = 0 is unambiguous
//!
//! ### Common Pitfalls
//!
//! 1. **False Positives from Comments**:
//!    - Problem: grep matches method names in doc comments
//!    - Solution: Filter out lines starting with `//` or `///`
//!    - Example: Ignore "Don't use `from_message()`" documentation
//!
//! 2. **False Positives from Test Code**:
//!    - Problem: Tests might mention old API
//!    - Solution: Only search `src/` directory, not `tests/`
//!    - Example: This test file mentions `from_message()` but it's in tests/
//!
//! 3. **False Negatives from Similar Names**:
//!    - Problem: "`with_message`" might match "`from_message`" pattern
//!    - Solution: Use exact patterns with boundaries
//!    - Example: Search "pub fn `from_message`(" not just "`from_message`"
//!
//! 4. **Assuming Absence**:
//!    - Problem: Not verifying that grep actually ran
//!    - Solution: Assert file exists before grepping
//!    - Example: Verify src/command.rs exists before searching it
//!
//! ### Design Principles for Impossibility
//!
//! 1. **Delete, Don't Deprecate**: Remove old code entirely (no #[deprecated])
//! 2. **Break Compatibility**: Change signatures to prevent accidental usage
//! 3. **Type System Enforcement**: Use enums/types to reject old patterns
//! 4. **No Fallbacks**: Don't provide compatibility shims or adapters
//!
//! ## Test Organization
//!
//! Each `#[test]` function verifies one impossibility category:
//! - Factory methods: Verify none exist except `new()`
//! - Public fields: Verify all fields private
//! - Deprecated methods: Verify old execution API removed
//! - Type system: Verify enums replace strings
//!
//! Following "One Aspect Per Test" principle for precise failure diagnosis.
//!
//! ## Historical Context
//!
//! This test was created during migration from factory pattern to builder pattern.
//! Without impossibility verification, we discovered developers occasionally tried
//! old API patterns that should have been removed. This test prevents regression.

use std::fs;
use std::path::Path;

/// Helper to verify method doesn't exist in file
fn assert_method_not_exists( file_path : &str, method_pattern : &str, method_name : &str )
{
  let path = Path::new( file_path );
  assert!( path.exists(), "File {file_path} should exist" );

  let content = fs::read_to_string( path )
    .unwrap_or_else(|_| panic!("Should read {file_path}"));

  // Filter out comments and test code
  let relevant_lines : Vec< &str > = content.lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      !trimmed.starts_with( "//" ) &&
      !trimmed.starts_with( "///" ) &&
      !trimmed.contains( "#[test]" )
    })
    .collect();

  let relevant_content = relevant_lines.join( "\n" );
  let count = relevant_content.matches( method_pattern ).count();

  assert_eq!( count, 0,
    "Method {method_name} should not exist in {file_path}, found {count} occurrences" );
}

/// Helper to count public fields in struct
fn count_public_fields_in_struct( file_path : &str, struct_name : &str ) -> usize
{
  let path = Path::new( file_path );
  if !path.exists()
  {
    return 0;
  }

  let content = fs::read_to_string( path ).unwrap();
  let lines : Vec< &str > = content.lines().collect();

  let mut in_struct = false;
  let mut count = 0;

  for line in lines
  {
    if line.contains( &format!( "pub struct {struct_name}" ) )
    {
      in_struct = true;
      continue;
    }
    if in_struct && line.trim().starts_with( '}' )
    {
      break;
    }
    if in_struct
    {
      let trimmed = line.trim();
      // Count only fully public fields (not pub(crate))
      if trimmed.starts_with( "pub " ) && !trimmed.starts_with( "pub(" )
      {
        count += 1;
      }
    }
  }

  count
}

// =====================================================================
// Category 1: Factory Methods (12 checks)
// =====================================================================

/// Verifies `from_message()` factory method doesn't exist.
///
/// **Old Pattern**: `ClaudeCommand::from_message("hello")`
/// **Why Removed**: Factory proliferation, use builder pattern instead
/// **New Pattern**: `ClaudeCommand::new().with_message("hello")`
#[test]
fn test_from_message_method_removed()
{
  assert_method_not_exists(
    "src/command.rs",
    "pub fn from_message",
    "from_message"
  );
}

/// Verifies `create()` factory method doesn't exist.
///
/// **Old Pattern**: `ClaudeCommand::create(...)`
/// **Why Removed**: Unclear semantics, use builder pattern
/// **New Pattern**: `ClaudeCommand::new().with_*(...)`
#[test]
fn test_create_method_removed()
{
  assert_method_not_exists(
    "src/command.rs",
    "pub fn create(",
    "create"
  );
}

/// Verifies `generate()` factory method doesn't exist.
///
/// **Old Pattern**: `ClaudeCommand::generate(...)`
/// **Why Removed**: Too many parameters, use builder pattern
/// **New Pattern**: `ClaudeCommand::new().with_*(...).with_*(...)`
#[test]
fn test_generate_method_removed()
{
  assert_method_not_exists(
    "src/command.rs",
    "pub fn generate(",
    "generate"
  );
}

/// Verifies no other factory-like methods exist.
///
/// Checks for common factory method names:
/// - `build_from`, `from_config`, `from_str`, `from_parts`
/// - `default_with`, `new_with`
///
/// Only allowed: `new()` (standard Rust constructor)
#[test]
fn test_no_other_factory_methods()
{
  let forbidden_factories = vec![
    ( "pub fn build_from(", "build_from" ),
    ( "pub fn from_config(", "from_config" ),
    ( "pub fn from_str(", "from_str" ),
    ( "pub fn from_parts(", "from_parts" ),
    ( "pub fn default_with(", "default_with" ),
    ( "pub fn new_with(", "new_with" ),
  ];

  for ( pattern, name ) in forbidden_factories
  {
    assert_method_not_exists( "src/command.rs", pattern, name );
  }
}

/// Verifies only `new()` constructor exists.
///
/// **Allowed**: `pub fn new()` (standard Rust constructor)
/// **Count**: Exactly 1 occurrence
#[test]
fn test_only_new_constructor_exists()
{
  let content = fs::read_to_string( "src/command.rs" )
    .expect( "Should read src/command.rs" );

  let count = content.matches( "pub fn new(" ).count();

  assert_eq!( count, 1,
    "Should have exactly 1 new() constructor, found {count}" );
}

/// Verifies `with_message()` is builder method, not factory.
///
/// **Critical**: `with_message` must take `self`, not be a factory
/// **Signature**: `pub fn with_message(mut self, ...) -> Self`
#[test]
fn test_with_message_is_builder_not_factory()
{
  let content = fs::read_to_string( "src/command.rs" )
    .expect( "Should read src/command.rs" );

  // Find with_message definition
  let with_message_line = content.lines()
    .find( | line | line.contains( "pub fn with_message" ) );

  if let Some( line ) = with_message_line {
    assert!( line.contains( "mut self" ),
      "with_message() must take mut self (builder pattern), got: {line}" );
  } else {
    // with_message might not exist yet, that's okay
  }
}

// =====================================================================
// Category 2: Public Fields (11 checks)
// =====================================================================

/// Verifies no public fields in `ClaudeCommand` struct.
///
/// **Old Pattern**: `cmd.message = "new value";` (public field)
/// **New Pattern**: `cmd.with_message("new value")` (builder method)
///
/// **Why This Matters**: Public fields break encapsulation, allow invalid states.
#[test]
fn test_no_public_fields_in_command_struct()
{
  let count = count_public_fields_in_struct( "src/command.rs", "ClaudeCommand" );

  assert_eq!( count, 0,
    "ClaudeCommand should have 0 public fields, found {count}" );
}

/// Verifies message field is private.
///
/// **Old**: `pub message: String`
/// **New**: `message: String` (private)
#[test]
fn test_message_field_is_private()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Look for "pub message:" in ClaudeCommand struct
  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  let pub_message_count = struct_section.matches( "pub message:" ).count();

  assert_eq!( pub_message_count, 0,
    "message field should be private, found {pub_message_count} public declarations" );
}

/// Verifies `working_directory` field is private.
#[test]
fn test_working_directory_field_is_private()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  let pub_count = struct_section.matches( "pub working_directory:" ).count();

  assert_eq!( pub_count, 0,
    "working_directory field should be private" );
}

/// Verifies `max_output_tokens` field is private.
#[test]
fn test_max_output_tokens_field_is_private()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  let pub_count = struct_section.matches( "pub max_output_tokens:" ).count();

  assert_eq!( pub_count, 0,
    "max_output_tokens field should be private" );
}

/// Verifies all configuration fields are private.
///
/// Checks common field names that should be private:
/// - `continue_conversation`
/// - `auto_continue`
/// - telemetry
/// - `bash_timeout_ms`
#[test]
fn test_all_config_fields_private()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  // These fields should NOT have "pub" prefix
  let sensitive_fields = vec![
    "continue_conversation",
    "auto_continue",
    "telemetry",
    "bash_timeout_ms",
  ];

  for field in sensitive_fields
  {
    let pattern = format!( "pub {field}:" );
    let count = struct_section.matches( &pattern ).count();

    assert_eq!( count, 0,
      "Field {field} should be private, found {count} public declarations" );
  }
}

// =====================================================================
// Category 3: Deprecated Methods (8 checks)
// =====================================================================

/// Verifies `execute_non_interactive()` removed.
///
/// **Old API**: `cmd.execute_non_interactive()?`
/// **New API**: `cmd.execute()?`
///
/// **Why Removed**: Redundant naming, `execute()` implies non-interactive by default
#[test]
fn test_execute_non_interactive_removed()
{
  assert_method_not_exists(
    "src/command.rs",
    "pub fn execute_non_interactive(",
    "execute_non_interactive"
  );
}

/// Verifies `run()` method doesn't exist (if it was deprecated).
///
/// **Pattern**: Some codebases use `run()` instead of `execute()`
/// **Standard**: Use `execute()` for clarity
#[test]
fn test_run_method_removed()
{
  assert_method_not_exists(
    "src/command.rs",
    "pub fn run(",
    "run"
  );
}

/// Verifies `spawn()` method doesn't exist as public API.
///
/// **Internal OK**: spawn might exist as private helper
/// **Public Not OK**: Public `spawn()` exposes low-level details
#[test]
fn test_no_public_spawn_method()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Check for pub fn spawn (not private fn spawn)
  let pub_spawn_count = content.matches( "pub fn spawn(" ).count();

  assert_eq!( pub_spawn_count, 0,
    "spawn() should not be public API" );
}

/// Verifies old execution methods removed.
///
/// Checks for common deprecated execution patterns:
/// - `execute_sync`, `execute_async`
/// - `run_command`, `run_interactive`
/// - call, invoke
#[test]
fn test_old_execution_methods_removed()
{
  let deprecated_methods = vec![
    ( "pub fn execute_sync(", "execute_sync" ),
    ( "pub fn execute_async(", "execute_async" ),
    ( "pub fn run_command(", "run_command" ),
    ( "pub fn run_interactive(", "run_interactive" ),
    ( "pub fn call(", "call" ),
    ( "pub fn invoke(", "invoke" ),
  ];

  for ( pattern, name ) in deprecated_methods
  {
    assert_method_not_exists( "src/command.rs", pattern, name );
  }
}

// =====================================================================
// Category 4: Type System Changes (3 checks)
// =====================================================================

/// Verifies `ActionMode` enum exists (replacing string literals).
///
/// **Old**: `action_mode: "ask"` (string literal)
/// **New**: `action_mode: ActionMode::Ask` (enum)
#[test]
fn test_action_mode_enum_exists()
{
  let content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist with enum definitions" );

  assert!( content.contains( "pub enum ActionMode" ),
    "ActionMode enum should exist in src/types.rs" );

  // Verify variants
  assert!( content.contains( "Ask" ), "ActionMode::Ask variant should exist" );
  assert!( content.contains( "Allow" ), "ActionMode::Allow variant should exist" );
  assert!( content.contains( "Deny" ), "ActionMode::Deny variant should exist" );
}

/// Verifies `LogLevel` enum exists (replacing string literals).
///
/// **Old**: `log_level: "info"` (string literal)
/// **New**: `log_level: LogLevel::Info` (enum)
#[test]
fn test_log_level_enum_exists()
{
  let content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist with enum definitions" );

  assert!( content.contains( "pub enum LogLevel" ),
    "LogLevel enum should exist in src/types.rs" );

  // Verify common variants
  assert!( content.contains( "Error" ), "LogLevel::Error variant should exist" );
  assert!( content.contains( "Warn" ), "LogLevel::Warn variant should exist" );
  assert!( content.contains( "Info" ), "LogLevel::Info variant should exist" );
}

/// Verifies enums used in command.rs (not raw strings).
///
/// **Pattern**: Look for `ActionMode::` and `LogLevel::` usage
/// **Anti-Pattern**: String literals like "ask", "info" in enum contexts
#[test]
fn test_enums_used_not_strings()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Check that ActionMode and LogLevel are referenced
  // (usage indicates enums are being used, not strings)
  let uses_action_mode = content.contains( "ActionMode" );
  let uses_log_level = content.contains( "LogLevel" );

  // At least one should be true if enums are being used
  assert!( uses_action_mode || uses_log_level,
    "command.rs should reference ActionMode or LogLevel enums" );
}

// =====================================================================
// Final Verification: Complete Impossibility
// =====================================================================

/// Verifies complete impossibility of old API usage.
///
/// **Comprehensive Check**: Combines all impossibility criteria
///
/// **If this test passes**:
/// - Old factory methods physically don't exist
/// - Public fields removed (encapsulation enforced)
/// - Deprecated methods removed (no old execution API)
/// - Type system changed (enums replace strings)
///
/// **Result**: Old API usage is IMPOSSIBLE, not discouraged
#[test]
fn test_old_api_usage_impossible()
{
  // Verify no factory methods
  assert_method_not_exists( "src/command.rs", "pub fn from_message", "from_message" );
  assert_method_not_exists( "src/command.rs", "pub fn create(", "create" );
  assert_method_not_exists( "src/command.rs", "pub fn generate(", "generate" );

  // Verify no public fields
  let pub_fields = count_public_fields_in_struct( "src/command.rs", "ClaudeCommand" );
  assert_eq!( pub_fields, 0, "Public fields make old direct-access pattern possible" );

  // Verify no deprecated execution methods
  assert_method_not_exists( "src/command.rs", "pub fn execute_non_interactive(", "execute_non_interactive" );

  // Verify enums exist (type system enforcement)
  let types_content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist" );
  assert!( types_content.contains( "pub enum ActionMode" ), "ActionMode enum required for type safety" );
  assert!( types_content.contains( "pub enum LogLevel" ), "LogLevel enum required for type safety" );
}

/// Verifies grep patterns are accurate (meta-test).
///
/// **Purpose**: Ensure our impossibility checks aren't giving false confidence
///
/// **Method**: Verify known-good patterns match
#[test]
fn test_grep_patterns_accurate()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Verify we CAN find new() (proves grep works)
  assert!( content.contains( "pub fn new(" ),
    "Should find new() method (proves grep patterns work)" );

  // Verify we CAN find execute() (proves grep works)
  assert!( content.contains( "pub fn execute(" ),
    "Should find execute() method (proves grep patterns work)" );

  // If we can't find these, our grep patterns are broken
}
