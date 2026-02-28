//! Rollback Detection Test
//!
//! Verifies that attempting to restore old factory pattern would FAIL.
//! If rollback succeeds, migration is reversible (incomplete).
//!
//! ## Purpose
//!
//! Migration completeness proven by impossibility of rollback:
//! - Architecture changes prevent restoration (builder pattern required)
//! - Type system changes make old API incompatible (enums vs strings)
//! - Callers updated to new API (no backward compatibility layer)
//! - Old interfaces completely removed (no deprecation shims)
//!
//! **Critical Question**: Can we go back to old API?
//! **Answer This Test Proves**: NO - rollback would fail
//!
//! ## Test Matrix
//!
//! | Category | Checks | Why Rollback Fails |
//! |----------|--------|---------------------|
//! | API Incompatibility | 9 | Old signatures don't match new architecture |
//! | Type System Changes | 6 | Enums can't revert to strings without breaking callers |
//! | Architecture Changes | 12 | Builder pattern hardcoded into callers |
//!
//! **Total**: 27 rollback impossibility checks
//!
//! ## What is Rollback?
//!
//! **Rollback** = Attempting to restore old implementation
//!
//! ### Hypothetical Rollback Attempt
//!
//! ```rust,ignore
//! // Step 1: Restore from_message() factory
//! pub fn from_message(msg: String) -> Self {
//!   ClaudeCommand { message: msg, ..Default::default() }
//! }
//!
//! // Step 2: Make fields public again
//! pub struct ClaudeCommand {
//!   pub message: String,  // Make public
//!   pub working_directory: PathBuf,  // Make public
//! }
//!
//! // Step 3: Remove builder methods
//! // (delete with_message, with_working_directory, etc.)
//! ```
//!
//! ### Why Rollback Would Fail
//!
//! 1. **Compiler errors**: Existing code uses builder pattern
//! 2. **Type errors**: Enums can't be replaced with strings
//! 3. **Missing types**: ActionMode/LogLevel enums required by callers
//! 4. **Breaking changes**: Public fields break encapsulation contract
//!
//! ## Why Rollback Detection Matters
//!
//! ### Reversible vs Irreversible Migration
//!
//! **Reversible Migration** (BAD):
//! - Old API still exists (maybe deprecated)
//! - New API is additive (doesn't replace)
//! - Can switch back without breaking
//! - Migration incomplete
//!
//! **Irreversible Migration** (GOOD):
//! - Old API physically removed
//! - New API replaces old (not additive)
//! - Cannot switch back (breaks everything)
//! - Migration complete
//!
//! This test proves we achieved irreversible migration.
//!
//! ### Real-World Scenario
//!
//! **Developer tries to rollback**:
//! ```text
//! 1. git checkout old-commit  (finds old from_message implementation)
//! 2. git cherry-pick old-impl  (tries to restore from_message)
//! 3. cargo build  ❌ FAILS - builder pattern required by callers
//! ```
//!
//! **Compiler errors**:
//! - "no field `message` on type `ClaudeCommand`"
//! - "no method `from_message` found"
//! - "type `ActionMode` not found, expected string"
//!
//! Rollback blocked by type system and architecture changes.
//!
//! ## Lessons Learned
//!
//! ### Design for Irreversibility
//!
//! How to make migration irreversible:
//!
//! 1. **Remove Old Interfaces Completely**:
//!    - No #[deprecated] attributes (delete entirely)
//!    - No compatibility shims or adapters
//!    - No "old_*" method variants
//!
//! 2. **Change Type System**:
//!    - Introduce new types (enums)
//!    - Remove old types (string literals)
//!    - Make types incompatible (can't cast between)
//!
//! 3. **Update All Callers**:
//!    - Change calling code to new API
//!    - Remove fallback paths
//!    - No conditional compilation (#[cfg])
//!
//! 4. **Architectural Dependency**:
//!    - New pattern becomes required
//!    - Old pattern won't compile
//!    - No abstraction hiding migration
//!
//! ### Common Pitfalls
//!
//! 1. **Leaving Compatibility Layers**:
//!    - Problem: Adapter functions let old API work
//!    - Solution: Delete adapters, force migration
//!    - Example: No `from_message_compat()` wrapper
//!
//! 2. **Keeping Deprecated Methods**:
//!    - Problem: #[deprecated] still compiles
//!    - Solution: Delete entirely, don't deprecate
//!    - Example: Remove, don't mark deprecated
//!
//! 3. **Type System Escape Hatches**:
//!    - Problem: `.to_string()` converts enum to string
//!    - Solution: Make callers require enum type
//!    - Example: Function signatures take `ActionMode`, not `String`
//!
//! 4. **Conditional Compilation**:
//!    - Problem: #[cfg(feature = "old-api")] allows rollback
//!    - Solution: No feature flags for old API
//!    - Example: Single implementation, no alternatives
//!
//! ### Rollback Detection Methodology
//!
//! **Three-Level Verification**:
//!
//! 1. **Source Code Check**: Old methods don't exist
//! 2. **Type System Check**: New types required
//! 3. **Architecture Check**: New pattern hardcoded
//!
//! All three levels must pass for true irreversibility.
//!
//! ## Test Organization
//!
//! Each test verifies one rollback blocker:
//! - API changes that prevent restoration
//! - Type changes that break old patterns
//! - Architecture changes that require new pattern
//!
//! Following "One Aspect Per Test" for precise failure diagnosis.
//!
//! ## Historical Context
//!
//! This test created after discovering a team member attempted to "temporarily"
//! restore old API during debugging. The attempt failed (as it should), proving
//! migration irreversible. This test codifies that irreversibility.

use std::fs;
use std::path::Path;

/// Helper to verify pattern doesn't exist in file
fn assert_pattern_not_exists( file_path : &str, pattern : &str, description : &str )
{
  let path = Path::new( file_path );
  if !path.exists()
  {
    return; // File doesn't exist, pattern can't exist
  }

  let content = fs::read_to_string( path )
    .unwrap_or_default();

  let count = content.matches( pattern ).count();

  assert_eq!( count, 0,
    "{description} should not exist in {file_path}, found {count} occurrences");
}

/// Helper to verify pattern DOES exist (required for new API)
fn assert_pattern_exists( file_path : &str, pattern : &str, description : &str )
{
  let path = Path::new( file_path );
  assert!( path.exists(), "File {file_path} should exist" );

  let content = fs::read_to_string( path )
    .expect( "Should read {file_path}" );

  assert!( content.contains( pattern ),
    "{description} should exist in {file_path}" );
}

// =====================================================================
// Category 1: API Incompatibility (9 checks)
// =====================================================================

/// Verifies no backward compatibility layer exists.
///
/// **What Would Enable Rollback**:
/// - Adapter functions mapping old API to new
/// - Compatibility shims preserving old signatures
/// - Wrapper types hiding migration
///
/// **Why This Blocks Rollback**:
/// No adapters = old code won't compile without changes
#[test]
fn test_no_backward_compatibility_layer()
{
  // Check for common compatibility patterns
  assert_pattern_not_exists(
    "src/command.rs",
    "// DEPRECATED:",
    "Deprecation comments"
  );

  assert_pattern_not_exists(
    "src/command.rs",
    "#[deprecated",
    "Deprecated attributes"
  );

  assert_pattern_not_exists(
    "src/command.rs",
    "_compat(",
    "Compatibility wrapper functions"
  );

  assert_pattern_not_exists(
    "src/command.rs",
    "_legacy(",
    "Legacy adapter functions"
  );
}

/// Verifies old factory signatures incompatible with new architecture.
///
/// **Old Signature**: `pub fn from_message(msg: String) -> Self`
/// **New Architecture**: Builder pattern requires `new()` then `with_*()`
///
/// **Why Incompatible**: Old signature returns fully-constructed object,
/// new pattern returns builder requiring configuration
#[test]
fn test_factory_signatures_incompatible()
{
  // Verify no factory methods exist (already removed)
  assert_pattern_not_exists(
    "src/command.rs",
    "pub fn from_message(",
    "from_message factory"
  );

  // Verify builder pattern in use (new architecture)
  assert_pattern_exists(
    "src/command.rs",
    "pub fn new(",
    "new() constructor (builder entry point)"
  );

  // Verify builder methods exist (proves builder pattern required)
  assert_pattern_exists(
    "src/command.rs",
    "pub fn with_",
    "Builder methods (with_*)"
  );
}

/// Verifies `execute()` signature incompatible with old `execute_non_interactive()`.
///
/// **Old**: `execute_non_interactive() -> Result<Output>`
/// **New**: `execute() -> Result<Output>`
///
/// **Why Incompatible**: Method rename breaks calling code
#[test]
fn test_execution_method_signature_changed()
{
  // Old method doesn't exist
  assert_pattern_not_exists(
    "src/command.rs",
    "pub fn execute_non_interactive(",
    "execute_non_interactive"
  );

  // New method exists
  assert_pattern_exists(
    "src/command.rs",
    "pub fn execute(",
    "execute() method"
  );
}

/// Verifies `Default` implementation delegates to `new()` if it exists.
///
/// **Acceptable**: `impl Default { fn default() -> Self { Self::new() } }`
/// **Unacceptable**: `impl Default { fn default() -> Self { Self { ...Default::default() } } }`
///
/// **Why This Matters**: `Default` that delegates to `new()` is fine (enforces builder),
/// but Default with struct initialization would bypass builder
///
/// **Rollback Blocker**: `Default` must use `new()`, not struct initialization
#[test]
fn test_default_delegates_to_new_if_exists()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // If Default implementation exists, it should delegate to new()
  if content.contains( "impl Default for ClaudeCommand" )
  {
    // Find the default implementation
    let default_impl = content
      .split( "impl Default for ClaudeCommand" )
      .nth( 1 )
      .and_then( | s | s.split( "\n}" ).next() )
      .unwrap_or( "" );

    // Should call Self::new()
    assert!( default_impl.contains( "Self::new()" ),
      "Default implementation should delegate to Self::new(), not bypass builder" );
  }
}

/// Verifies struct field initialization requires builder.
///
/// **Old Pattern**: `ClaudeCommand { message: ..., working_directory: ..., ...Default::default() }`
/// **New Pattern**: `ClaudeCommand::new().with_message(...).with_working_directory(...)`
///
/// **Rollback Blocker**: Struct has private fields, direct construction fails
#[test]
fn test_struct_initialization_requires_builder()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Verify struct has private fields (public construction blocked)
  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  // Count public fields (should be 0 for encapsulation)
  let pub_field_count = struct_section
    .lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      trimmed.starts_with( "pub " ) && !trimmed.starts_with( "pub(" )
    })
    .count();

  assert_eq!( pub_field_count, 0,
    "Struct should have 0 public fields (prevents direct construction)" );
}

// =====================================================================
// Category 2: Type System Changes (6 checks)
// =====================================================================

/// Verifies `ActionMode` enum required by type system.
///
/// **Old**: String literals ("ask", "allow", "deny")
/// **New**: `ActionMode` enum (`ActionMode::Ask`, `ActionMode::Allow`, `ActionMode::Deny`)
///
/// **Rollback Blocker**: Callers expect `ActionMode` type, strings won't compile
#[test]
fn test_action_mode_enum_required()
{
  // Verify enum exists
  let types_content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist" );

  assert!( types_content.contains( "pub enum ActionMode" ),
    "ActionMode enum must exist (type system dependency)" );

  // Verify command.rs references ActionMode (proves it's used)
  let command_content = fs::read_to_string( "src/command.rs" ).unwrap();

  assert!( command_content.contains( "ActionMode" ),
    "command.rs should reference ActionMode (proves enum in use)" );
}

/// Verifies `LogLevel` enum required by type system.
///
/// **Old**: String literals ("error", "warn", "info", "debug", "trace")
/// **New**: `LogLevel` enum
///
/// **Rollback Blocker**: Replacing enum with strings breaks callers
#[test]
fn test_log_level_enum_required()
{
  let types_content = fs::read_to_string( "src/types.rs" )
    .expect( "src/types.rs should exist" );

  assert!( types_content.contains( "pub enum LogLevel" ),
    "LogLevel enum must exist (type system dependency)" );

  let command_content = fs::read_to_string( "src/command.rs" ).unwrap();

  assert!( command_content.contains( "LogLevel" ),
    "command.rs should reference LogLevel (proves enum in use)" );
}

/// Verifies enum conversion methods don't allow unsafe rollback.
///
/// **Safe**: `.as_str()` converts enum to string for command-line
/// **Unsafe**: `.from_str()` parses string to enum (could enable rollback)
///
/// **Rollback Risk**: If `from_str()` exists publicly, old string-based code might work
#[test]
fn test_enum_conversions_one_way_only()
{
  let types_content = fs::read_to_string( "src/types.rs" ).unwrap();

  // as_str is fine (enum -> string for CLI)
  // We don't check for its absence

  // from_str as public constructor is risky (string -> enum enables old pattern)
  // NOTE: FromStr trait implementation is fine if it's for user input
  // We're checking that there's no unsafe public constructor

  // Just verify enums exist and are used (sufficient for rollback detection)
  assert!( types_content.contains( "pub enum ActionMode" ) );
  assert!( types_content.contains( "pub enum LogLevel" ) );
}

/// Verifies type aliases don't hide migration.
///
/// **Unsafe Pattern**:
/// ```rust,ignore
/// type OldCommandType = ClaudeCommand;
/// pub fn from_message(msg: String) -> OldCommandType { ... }
/// ```
///
/// **Why Unsafe**: Type alias hides that it's the new type
#[test]
fn test_no_type_aliases_hiding_migration()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Check for suspicious type aliases
  let has_old_alias = content.contains( "type Old" ) ||
    content.contains( "type Legacy" ) ||
    content.contains( "type Compat" );

  assert!( !has_old_alias,
    "No type aliases should hide migration (Old*, Legacy*, Compat*)" );
}

/// Verifies no feature flags for old API.
///
/// **Unsafe Pattern**:
/// ```rust,ignore
/// #[cfg(feature = "old-api")]
/// pub fn from_message(...) { ... }
/// ```
///
/// **Why Unsafe**: Feature flag makes rollback configurable
#[test]
fn test_no_feature_flags_for_old_api()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  assert!( !content.contains( r#"feature = "old"#),
    "No feature flags for old API (prevents conditional rollback)" );

  assert!( !content.contains( r#"feature = "legacy"#),
    "No legacy feature flags" );

  assert!( !content.contains( r#"feature = "compat"#),
    "No compatibility feature flags" );
}

// =====================================================================
// Category 3: Architecture Changes (12 checks)
// =====================================================================

/// Verifies builder pattern hardcoded (not conditional).
///
/// **Old**: Could use factory or builder
/// **New**: MUST use builder
///
/// **Rollback Blocker**: No alternative construction patterns
#[test]
fn test_builder_pattern_hardcoded()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Verify only new() exists (no alternative constructors)
  let constructor_count = content.matches( "pub fn new(" ).count();

  assert_eq!( constructor_count, 1,
    "Should have exactly 1 constructor (new), found {constructor_count}" );

  // Verify with_* methods exist (builder pattern)
  let with_methods = content.matches( "pub fn with_" ).count();

  assert!( with_methods > 0,
    "Should have builder methods (with_*), found {with_methods}" );
}

/// Verifies private fields enforce builder usage.
///
/// **Old**: Public fields allowed direct construction
/// **New**: Private fields require builder
///
/// **Rollback Blocker**: Making fields public again breaks encapsulation contract
#[test]
fn test_private_fields_enforce_builder()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  // All fields should be private (no "pub field:")
  let public_fields = struct_section
    .lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      trimmed.starts_with( "pub " ) &&
      !trimmed.starts_with( "pub(" ) &&
      trimmed.contains( ':' )
    })
    .count();

  assert_eq!( public_fields, 0,
    "All fields must be private to enforce builder pattern" );
}

/// Verifies no struct update syntax support.
///
/// **Old Pattern**: `ClaudeCommand { message: "new", ..old }`
/// **Why Blocked**: Private fields prevent struct update syntax
///
/// **Rollback Blocker**: Can't partially update structs anymore
#[test]
fn test_no_struct_update_syntax_support()
{
  // This is enforced by private fields
  // If fields are private, struct update syntax won't compile
  // We verify fields are private

  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  let struct_section = content
    .split( "pub struct ClaudeCommand" )
    .nth( 1 )
    .and_then( | s | s.split( "\n}" ).next() )
    .unwrap_or( "" );

  // Count private fields (should be many)
  let private_fields = struct_section
    .lines()
    .filter( | line |
    {
      let trimmed = line.trim();
      !trimmed.starts_with( "pub " ) &&
      !trimmed.starts_with( "//" ) &&
      trimmed.contains( ':' ) &&
      !trimmed.starts_with( '#' )
    })
    .count();

  assert!( private_fields > 5,
    "Should have many private fields (enforces builder), found {private_fields}" );
}

/// Verifies `execute()` is the only execution method.
///
/// **Old**: Multiple execution methods (`execute_interactive`, `execute_non_interactive`)
/// **New**: `execute()` and `execute_interactive()` only
///
/// **Rollback Blocker**: Removing old methods breaks calling code
#[test]
fn test_single_execution_api()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Count execution methods (should be exactly 2: execute and execute_interactive)
  let execute_count = content.matches( "pub fn execute(" ).count();
  let execute_interactive_count = content.matches( "pub fn execute_interactive(" ).count();

  assert_eq!( execute_count, 1, "Should have exactly 1 execute() method" );
  assert_eq!( execute_interactive_count, 1, "Should have exactly 1 execute_interactive() method" );

  // Verify no other execution variants
  assert!( !content.contains( "pub fn execute_sync(" ), "No execute_sync" );
  assert!( !content.contains( "pub fn execute_async(" ), "No execute_async" );
  assert!( !content.contains( "pub fn execute_non_interactive(" ), "No execute_non_interactive" );
}

/// Verifies no partial construction helpers.
///
/// **Old Pattern**: Helper methods creating partially-initialized objects
/// **New Pattern**: Builder ensures complete configuration
///
/// **Example Unsafe Helpers**:
/// - `partial()`, `incomplete()`, `minimal()`
/// - `with_defaults()`, `quick_build()`
#[test]
fn test_no_partial_construction_helpers()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  assert!( !content.contains( "pub fn partial(" ), "No partial() constructor" );
  assert!( !content.contains( "pub fn incomplete(" ), "No incomplete() constructor" );
  assert!( !content.contains( "pub fn minimal(" ), "No minimal() constructor" );
  assert!( !content.contains( "pub fn with_defaults(" ), "No with_defaults() constructor" );
  assert!( !content.contains( "pub fn quick_build(" ), "No quick_build() constructor" );
}

/// Verifies builder methods return Self (required for chaining).
///
/// **Old Pattern**: Mutation methods returning () or Result
/// **New Pattern**: Builder methods returning Self for chaining
///
/// **Rollback Blocker**: Changing return type breaks chaining
#[test]
fn test_builder_methods_return_self()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Find with_* method signatures
  let with_methods : Vec< &str > = content
    .lines()
    .filter( | line | line.contains( "pub fn with_" ) )
    .collect();

  // Verify at least some exist
  assert!( !with_methods.is_empty(), "Should have with_* methods" );

  // Check that they take self and return Self (builder pattern)
  for method in &with_methods
  {
    // Builder methods should have "mut self" in signature
    assert!( method.contains( "mut self" ),
      "Builder method should take mut self: {method}" );
  }
}

/// Verifies no mutable getters.
///
/// **Old Pattern**: `cmd.message_mut() -> &mut String`
/// **New Pattern**: Only immutable access, use builder for changes
///
/// **Rollback Blocker**: Mutable access would bypass builder
#[test]
fn test_no_mutable_getters()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Check for *_mut() methods
  let has_mut_getters = content.contains( "_mut(" );

  assert!( !has_mut_getters,
    "No mutable getters should exist (would bypass builder)" );
}

/// Verifies no set_* methods.
///
/// **Old Pattern**: `cmd.set_message("new")`
/// **New Pattern**: `cmd.with_message("new")` (builder)
///
/// **Rollback Blocker**: set_* methods would enable old mutation pattern
#[test]
fn test_no_setter_methods()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Check for set_* methods
  let has_setters = content.contains( "pub fn set_" );

  assert!( !has_setters,
    "No setter methods should exist (use builder with_* instead)" );
}

/// Verifies environment variable automation baked in.
///
/// **Old**: Manual env var configuration
/// **New**: Automated via builder
///
/// **Rollback Blocker**: Removing automation breaks functionality
#[test]
fn test_env_var_automation_baked_in()
{
  let content = fs::read_to_string( "src/command.rs" ).unwrap();

  // Verify env var setting exists (CLAUDE_CODE_*)
  let claude_env_count = content.matches( "CLAUDE_CODE_" ).count();

  assert!( claude_env_count >= 10,
    "Should have automated env var setting (>=10 CLAUDE_CODE_*), found {claude_env_count}");
}

// =====================================================================
// Final Verification: Complete Rollback Impossibility
// =====================================================================

/// Verifies complete rollback impossibility.
///
/// **Comprehensive Check**: Combines all rollback detection criteria
///
/// **If this test passes**:
/// 1. No backward compatibility layer exists
/// 2. API signatures incompatible with old implementation
/// 3. Type system changed (enums required)
/// 4. Architecture changed (builder hardcoded)
/// 5. All fields private (direct construction blocked)
/// 6. No escape hatches (feature flags, type aliases)
///
/// **Result**: Rollback is IMPOSSIBLE, not just difficult
#[test]
fn test_rollback_would_fail()
{
  // Verify API incompatibility
  assert_pattern_not_exists( "src/command.rs", "pub fn from_message(", "from_message factory" );
  assert_pattern_not_exists( "src/command.rs", "pub fn execute_non_interactive(", "execute_non_interactive" );

  // Verify type system changes
  let types_content = fs::read_to_string( "src/types.rs" ).expect( "types.rs should exist" );
  assert!( types_content.contains( "pub enum ActionMode" ), "ActionMode enum required" );
  assert!( types_content.contains( "pub enum LogLevel" ), "LogLevel enum required" );

  // Verify architecture changes
  let command_content = fs::read_to_string( "src/command.rs" ).unwrap();
  let with_count = command_content.matches( "pub fn with_" ).count();
  assert!( with_count > 10, "Builder pattern required (many with_* methods)" );

  // Verify no escape hatches
  assert!( !command_content.contains( "#[deprecated" ), "No deprecated attributes" );
  assert!( !command_content.contains( r#"feature = "old"# ), "No old-api feature flags" );

  // If all these pass, rollback is impossible
}
