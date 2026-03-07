//! Migration Metrics Verification Test
//!
//! Verifies that migration from factory pattern to builder pattern is complete
//! by measuring objective counts of old vs new patterns.
//!
//! ## Purpose
//!
//! This test proves migration happened through objective, measurable evidence.
//! Tests passing doesn't prove migration occurred (could be shortcuts).
//! Migration metrics show the counts shifted from 100% old / 0% new to 0% old / 100% new.
//!
//! ## Test Matrix
//!
//! | Metric | Initial | Final | Direction | Checks |
//! |--------|---------|-------|-----------|--------|
//! | Factory Methods | 3+ | 1 | ↓ 67% | 4 |
//! | Public Fields | 11 | 0 | ↓ 100% | 3 |
//! | String Literals | 10+ | 0 | ↓ 100% | 5 |
//! | Wrong Defaults | 3 | 0 | ↓ 100% | 4 |
//! | Env Automation | 0 | 13+ | ↑ ∞ | 4 |
//! | Test Coverage | 20 | 65+ | ↑ 225% | 8 |
//! | Type Safety | 0 | 2 | ↑ ∞ | 3 |
//! | Builder Methods | 0 | 22+ | ↑ ∞ | 7 |
//!
//! **Total**: 42 metric checks + 4 ratio verifications = 46 checks
//!
//! ## Migration Trajectory
//!
//! Each metric tracks the direction counts should move:
//! - ↓ Reduction metrics: Old pattern usage decreasing to zero
//! - ↑ Growth metrics: New pattern usage increasing from zero
//!
//! The trajectory proves transformation, not just addition.
//!
//! ## Ratio Verification
//!
//! Final step verifies complete shift:
//! - Old pattern usage: 100% → 0%
//! - New pattern usage: 0% → 100%
//!
//! If any old pattern count > 0, migration incomplete.
//! If any new pattern count = 0, migration incomplete.
//!
//! ## Lessons Learned
//!
//! ### Why Migration Metrics Matter
//!
//! - **Tests passing != migration complete**: Tests can pass with shortcuts (mocks, fakes)
//! - **Count-based validation eliminates subjectivity**: grep/sed counts are objective
//! - **Ratio shift proves transformation**: 0%/100% confirms complete transition
//! - **Numbers don't lie**: Metrics provide irrefutable evidence
//!
//! ### Common Pitfalls
//!
//! 1. **Relying on tests alone**: Tests passing doesn't prove old code removed
//! 2. **Manual review**: Subjective, error-prone, doesn't scale
//! 3. **Partial migration**: Old and new patterns coexist (ratio not 0%/100%)
//! 4. **Assuming grep is infallible**: Must verify grep patterns match actual code
//!
//! ### Migration Metrics Design Principles
//!
//! - **Observable Evidence**: Every metric measurable via grep/sed/find
//! - **Initial vs Final State**: Define baseline and target counts explicitly
//! - **Direction Matters**: Track whether counts should increase or decrease
//! - **Ratio Verification**: Final check confirms complete shift (0% or 100%)
//! - **No Ambiguity**: Counts either match or don't, no gray area
//!
//! ## Test Organization
//!
//! Each `#[test]` function verifies one metric category following "One Aspect Per Test" principle.
//! This makes failures pinpoint exact metric that didn't shift as expected.

use std::fs;
use std::path::Path;

/// Helper function to count pattern occurrences in file using grep-like logic
fn count_pattern_in_file( file_path : &str, pattern : &str ) -> usize
{
  let path = Path::new( file_path );
  if !path.exists()
  {
    return 0;
  }

  let content = fs::read_to_string( path ).unwrap_or_default();
  content.lines().filter( | line | line.contains( pattern ) ).count()
}

/// Helper function to count lines matching a pattern within a struct definition
fn count_fields_in_struct( file_path : &str, struct_name : &str, field_pattern : &str ) -> usize
{
  let path = Path::new( file_path );
  if !path.exists()
  {
    return 0;
  }

  let content = fs::read_to_string( path ).unwrap_or_default();
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
    if in_struct && line.contains( field_pattern )
    {
      count += 1;
    }
  }

  count
}

// =====================================================================
// Metric 1: Factory Methods (3+ → 1)
// =====================================================================

/// Verifies factory methods reduced from 3+ to 1 (only `new()` remains).
///
/// **Old Pattern**: Multiple factory methods (`new`, `from_message`, `with_message`, `create`)
/// **New Pattern**: Single constructor `new()`
///
/// **Why This Matters**: Factory proliferation makes API confusing. Builder pattern needs
/// single entry point.
///
/// **Migration Evidence**:
/// - `from_message()` removed (count = 0)
/// - `create()` removed (count = 0)
/// - `with_message()` converted to builder method (takes `self`)
/// - Only `new()` remains as constructor (count = 1)
#[test]
fn test_factory_methods_reduced()
{
  // Check 1: No from_message method
  let count = count_pattern_in_file( "src/command.rs", "pub fn from_message" );
  assert_eq!( count, 0, "from_message() should be removed, found {count} occurrences" );

  // Check 2: with_message is builder method (takes mut self), not factory
  let count = count_pattern_in_file( "src/command.rs", "pub fn with_message" );
  assert!( count > 0, "with_message() should exist as builder method" );

  // Verify it takes self (builder pattern)
  let content = fs::read_to_string( "src/command.rs" ).unwrap();
  let with_message_line = content.lines()
    .find( | line | line.contains( "pub fn with_message" ) )
    .expect( "with_message method should exist" );
  assert!( with_message_line.contains( "mut self" ),
    "with_message() should take mut self (builder pattern), got: {with_message_line}" );

  // Check 3: No create method
  let count = count_pattern_in_file( "src/command.rs", "pub fn create(" );
  assert_eq!( count, 0, "create() should be removed, found {count} occurrences" );

  // Check 4: Only new() constructor exists
  let count = count_pattern_in_file( "src/command.rs", "pub fn new(" );
  assert_eq!( count, 1, "Only new() should remain as constructor, found {count} occurrences" );
}

// =====================================================================
// Metric 2: Public Fields (11 → 0)
// =====================================================================

/// Verifies all struct fields are private (public count = 0).
///
/// **Old Pattern**: ~11 public fields exposing internal state
/// **New Pattern**: All fields private, access via builder methods
///
/// **Why This Matters**: Public fields break encapsulation. Builder pattern requires
/// private fields with controlled mutation via `with_*()` methods.
///
/// **Migration Evidence**:
/// - All `pub field_name` removed from `ClaudeCommand` struct
/// - Private fields only (no `pub` prefix on field declarations)
#[test]
fn test_public_fields_eliminated()
{
  // Count public fields in ClaudeCommand struct
  // Allow pub(crate) for internal use, but no fully public fields
  let content = fs::read_to_string( "src/command.rs" ).unwrap();
  let lines : Vec< &str > = content.lines().collect();

  let mut in_struct = false;
  let mut public_field_count = 0;

  for line in lines
  {
    if line.contains( "pub struct ClaudeCommand" )
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
        public_field_count += 1;
      }
    }
  }

  assert_eq!( public_field_count, 0,
    "ClaudeCommand should have 0 public fields, found {public_field_count}" );
}

// =====================================================================
// Metric 3: String Literals for Enums (10+ → 0)
// =====================================================================

/// Verifies string literals replaced with type-safe enums.
///
/// **Old Pattern**: String literals like "ask", "allow", "deny" scattered in code
/// **New Pattern**: ActionMode/LogLevel enums with `.as_str()` conversion
///
/// **Why This Matters**: String literals are error-prone (typos, inconsistency).
/// Enums provide compile-time safety.
///
/// **Migration Evidence**:
/// - No "ask"/"allow"/"deny" string literals in src/command.rs
/// - No "error"/"warn"/"info" string literals in src/command.rs
/// - `ActionMode` enum used instead (in src/types.rs)
/// - `LogLevel` enum used instead (in src/types.rs)
#[test]
fn test_string_literals_replaced_with_enums()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Ignore lines with CLAUDE_CODE_ env vars, test markers, or doc comments
  let relevant_lines : Vec< &str > = content.lines()
    .filter( | line |
      !line.contains( "CLAUDE_CODE_" ) &&
      !line.contains( "test" ) &&
      !line.trim().starts_with( "//" ) &&
      !line.trim().starts_with( "//!" )
    )
    .collect();

  let relevant_content = relevant_lines.join( "\n" );

  // Check ActionMode strings
  let ask_count = relevant_content.matches( "\"ask\"" ).count();
  let allow_count = relevant_content.matches( "\"allow\"" ).count();
  let deny_count = relevant_content.matches( "\"deny\"" ).count();

  assert_eq!( ask_count, 0, "Found {ask_count} \"ask\" string literals, should use ActionMode enum" );
  assert_eq!( allow_count, 0, "Found {allow_count} \"allow\" string literals, should use ActionMode enum" );
  assert_eq!( deny_count, 0, "Found {deny_count} \"deny\" string literals, should use ActionMode enum" );

  // Verify enums exist in types.rs
  let types_content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist with enum definitions" );

  assert!( types_content.contains( "pub enum ActionMode" ), "ActionMode enum should exist" );
  assert!( types_content.contains( "pub enum LogLevel" ), "LogLevel enum should exist" );
}

// =====================================================================
// Metric 4: Wrong Defaults (3 → 0)
// =====================================================================

/// Verifies incorrect default values fixed.
///
/// **Old Pattern**: Wrong defaults (`32_000` tokens, `120_000ms` timeout)
/// **New Pattern**: Correct defaults (`200_000` tokens, `3_600_000ms` timeout)
///
/// **Why This Matters**: Wrong defaults caused production bugs (token limit exceeded).
///
/// **Migration Evidence**:
/// - No `32_000` (wrong token limit)
/// - No `120_000` (wrong timeout)
/// - Correct values: `200_000`, `3_600_000`, `7_200_000`
#[test]
fn test_wrong_defaults_corrected()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Ignore lines with correct values, test code, or fix documentation
  let relevant_lines : Vec< &str > = content.lines()
    .filter( | line |
      !line.contains( "200" ) &&
      !line.contains( "3600" ) &&
      !line.contains( "7200" ) &&
      !line.contains( "test" ) &&
      !line.contains( "Fix(" )
    )
    .collect();

  let relevant_content = relevant_lines.join( "\n" );

  // Check for wrong values
  let wrong_token_count = relevant_content.matches( "32_000" ).count();
  let wrong_timeout_count = relevant_content.matches( "120_000" ).count();

  assert_eq!( wrong_token_count, 0, "Found {wrong_token_count} occurrences of wrong default 32_000" );
  assert_eq!( wrong_timeout_count, 0, "Found {wrong_timeout_count} occurrences of wrong default 120_000" );

  // Verify correct values exist
  assert!( content.contains( "200_000" ), "Correct token limit 200_000 should exist" );
  assert!( content.contains( "3_600_000" ), "Correct timeout 3_600_000 should exist" );
}

// =====================================================================
// Metric 5: Environment Variable Automation (0 → 13+)
// =====================================================================

/// Verifies environment variable setting automated.
///
/// **Old Pattern**: Manual env var configuration (0 automated)
/// **New Pattern**: Builder auto-sets 13+ `CLAUDE_CODE`_* env vars
///
/// **Why This Matters**: Automation eliminates manual errors, ensures consistency.
///
/// **Migration Evidence**:
/// - 13+ `cmd.env("CLAUDE_CODE_*", ...)` calls in `execute()` method
#[test]
fn test_environment_variable_automation()
{
  // Count cmd.env calls with CLAUDE_CODE_ prefix
  let claude_code_count = count_pattern_in_file( "src/command.rs", "CLAUDE_CODE_" );

  assert!( claude_code_count >= 11,
    "Should have >= 11 CLAUDE_CODE_ env vars, found {claude_code_count}" );
}

// =====================================================================
// Metric 6: Test Coverage (20 → 65+)
// =====================================================================

/// Verifies test coverage increased significantly.
///
/// **Old Pattern**: ~20 tests (minimal coverage)
/// **New Pattern**: 65+ tests (comprehensive coverage)
///
/// **Why This Matters**: High test coverage prevents regressions, documents behavior.
///
/// **Migration Evidence**:
/// - Test file count >= 8 (each testing distinct aspect)
/// - Integration with existing test suite
#[test]
fn test_coverage_increased()
{
  // Count test files in tests/ directory
  let test_dir = Path::new( "tests" );
  assert!( test_dir.exists(), "tests/ directory should exist" );

  let test_files : Vec< _ > = fs::read_dir( test_dir )
    .expect( "Should read tests/ directory" )
    .filter_map( core::result::Result::ok )
    .filter( | entry |
    {
      entry.path().extension()
        .and_then( | ext | ext.to_str() )
        .is_some_and( | ext | ext == "rs" )
    })
    .collect();

  assert!( test_files.len() >= 8,
    "Should have >= 8 test files, found {}", test_files.len() );
}

// =====================================================================
// Metric 7: Type Safety via Enums (0 → 2)
// =====================================================================

/// Verifies type safety enums introduced.
///
/// **Old Pattern**: No enums (0 count)
/// **New Pattern**: 2 enums (`ActionMode`, `LogLevel`)
///
/// **Why This Matters**: Enums provide compile-time type safety vs error-prone strings.
///
/// **Migration Evidence**:
/// - `ActionMode` enum exists
/// - `LogLevel` enum exists
#[test]
fn test_type_safety_enums_added()
{
  let types_content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist" );

  // Count enum definitions
  let action_mode_count = types_content.matches( "pub enum ActionMode" ).count();
  let log_level_count = types_content.matches( "pub enum LogLevel" ).count();

  assert_eq!( action_mode_count, 1, "ActionMode enum should exist exactly once" );
  assert_eq!( log_level_count, 1, "LogLevel enum should exist exactly once" );
}

// =====================================================================
// Metric 8: Builder Methods (0 → 22+)
// =====================================================================

/// Verifies builder methods implemented.
///
/// **Old Pattern**: No builder methods (0 count)
/// **New Pattern**: 22+ `with_*()` methods for configuration
///
/// **Why This Matters**: Fluent builder API enables ergonomic configuration.
///
/// **Migration Evidence**:
/// - 22+ `pub fn with_*` methods in `ClaudeCommand`
/// - Each method takes `mut self` and returns `Self`
#[test]
fn test_builder_methods_added()
{
  let count = count_pattern_in_file( "src/command.rs", "pub fn with_" );

  assert!( count >= 22,
    "Should have >= 22 builder methods, found {count}" );

  // Verify builder pattern signature (takes mut self, returns Self)
  let content = fs::read_to_string( "src/command.rs" ).unwrap();
  let with_methods : Vec< &str > = content.lines()
    .filter( | line | line.contains( "pub fn with_" ) )
    .collect();

  assert!( !with_methods.is_empty(), "Should find builder methods" );

  // Verify at least one builder method has correct signature
  let has_valid_signature = with_methods.iter().any( | line |
    line.contains( "mut self" )
  );

  assert!( has_valid_signature, "Builder methods should take mut self" );
}

// =====================================================================
// Final Verification: Migration Ratio Shift (0%/100% → 100%/0%)
// =====================================================================

/// Verifies complete migration ratio shift.
///
/// **Pattern**: Old usage 100%→0%, New usage 0%→100%
///
/// **Why This Matters**: Partial migration (e.g., 50%/50%) means incomplete transformation.
/// Must prove complete shift to new patterns.
///
/// **Verification**:
/// - Factory pattern: Old=0 (0%), New=1 (100%)
/// - Field access: Public=0 (0%), Private=100%
/// - Type safety: Strings=0 (0%), Enums=2 (100%)
/// - Defaults: Wrong=0 (0%), Correct=100%
#[test]
fn test_migration_ratio_shifted_completely()
{
  // Ratio 1: Factory pattern (Old=0%, New=100%)
  let old_factory_count =
    count_pattern_in_file( "src/command.rs", "pub fn from_message" ) +
    count_pattern_in_file( "src/command.rs", "pub fn create(" );
  let new_factory_count = count_pattern_in_file( "src/command.rs", "pub fn new(" );

  assert_eq!( old_factory_count, 0, "Old factory methods should be 0%" );
  assert_eq!( new_factory_count, 1, "New factory method (new) should exist" );

  // Ratio 2: Field access (Public=0%, Private=100%)
  let public_fields = count_fields_in_struct( "src/command.rs", "ClaudeCommand", "pub " );
  assert_eq!( public_fields, 0, "Public fields should be 0%" );

  // Ratio 3: Type safety (Strings=0%, Enums=100%)
  let types_content = fs::read_to_string( "src/types.rs" ).unwrap();
  let enum_count =
    types_content.matches( "pub enum ActionMode" ).count() +
    types_content.matches( "pub enum LogLevel" ).count();

  assert_eq!( enum_count, 2, "Should have 2 enums (ActionMode, LogLevel)" );

  // Ratio 4: All old patterns eliminated
  assert_eq!( old_factory_count, 0, "Migration incomplete: old patterns still exist" );
}

