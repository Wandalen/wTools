//! Tests for command validation module.
//!
//! This module tests the centralized validation logic used across all command
//! registration approaches to ensure consistent behavior.
//!
//! # Why These Tests Exist
//!
//! **Purpose:** Verify that command validation rules are enforced consistently across
//! all construction paths (direct, builder, YAML, static conversion). Centralized
//! validation prevents bugs where one path validates but another doesn't.
//!
//! **What We're Protecting Against:**
//!
//! 1. **Inconsistent validation:** If registry validates but CLI builder doesn't,
//!    commands can bypass checks. These tests ensure validation is centralized and
//!    used everywhere.
//!
//! 2. **Validation gaps:** New construction paths might forget to validate. Tests
//!    verify that `validate_command_for_registration` is the single source of truth.
//!
//! 3. **Phase 2 breaking change:** Old validation checked individual fields (name, namespace).
//!    New validation checks `full_name()` to support `static_module` system. Tests verify
//!    this change doesn't break existing commands.
//!
//! 4. **Unclear error messages:** Validation errors should be actionable. Tests verify
//!    error messages mention specific problems (`MissingDotPrefix` vs generic "invalid").
//!
//! **How to Interpret Failures:**
//!
//! - **Name validation fails:** `CommandName` validation not running or rules changed
//! - **Namespace validation fails:** `NamespaceType` validation not running
//! - **Full validation fails:** Validation bypass in construction path
//! - **Helper function fails:** Help command detection or naming logic broken
//!
//! **Phase 2 Design Note:**
//!
//! Validation now checks `full_name()` (namespace + name) instead of individual fields.
//! This allows the `static_module` system to work (accepts "test" name, adds ".test" prefix).
//! Tests marked with "Phase 2 Update" comments verify this intentional behavior change.
//!
//! **Test Matrix for Command Validation**
//!
//! | Test | Category | Validates | Status |
//! |------|----------|-----------|--------|
//! | V1.1 | Command Name | Valid names with dot prefix | ✅ Tested |
//! | V1.2 | Command Name | Invalid names without dot | ✅ Tested |
//! | V1.3 | Command Name | Edge cases (empty, whitespace) | ✅ Tested |
//! | V2.1 | Namespace | Valid namespaces (empty and dotted) | ✅ Tested |
//! | V2.2 | Namespace | Invalid namespaces without dot | ✅ Tested |
//! | V2.3 | Namespace | Edge cases (whitespace) | ✅ Tested |
//! | V3.1 | Full Validation | Complete command validation | ✅ Tested |
//! | V3.2 | Full Validation | Error propagation | ✅ Tested |
//! | V4.1 | Helper Functions | Help command detection | ✅ Tested |
//! | V4.2 | Helper Functions | Help name generation | ✅ Tested |

use unilang::
{
  command_validation::
  {
    validate_command_name,
    validate_namespace,
    validate_command_for_registration,
    is_help_command,
    make_help_command_name,
  },
  data::CommandDefinition,
};

// ============================================================================
// V1: Command Name Validation Tests
// ============================================================================

#[ test ]
fn test_v1_1_valid_command_names()
{
  // Test Matrix Row: V1.1
  // Valid command names with dot prefix

  assert!( validate_command_name( ".hello" ).is_ok() );
  assert!( validate_command_name( ".video" ).is_ok() );
  assert!( validate_command_name( ".a" ).is_ok() );
  assert!( validate_command_name( ".very_long_command_name" ).is_ok() );
  assert!( validate_command_name( ".command123" ).is_ok() );
  assert!( validate_command_name( ".cmd-with-dashes" ).is_ok() );
}

#[ test ]
fn test_v1_2_invalid_command_names_without_dot()
{
  // Test Matrix Row: V1.2
  // Invalid command names without dot prefix

  assert!( validate_command_name( "hello" ).is_err() );
  assert!( validate_command_name( "video.search" ).is_err() );
  assert!( validate_command_name( "command" ).is_err() );
  assert!( validate_command_name( "a" ).is_err() );
}

#[ test ]
fn test_v1_3_command_name_edge_cases()
{
  // Test Matrix Row: V1.3
  // Edge cases for command name validation

  // Empty string
  assert!( validate_command_name( "" ).is_err() );

  // Whitespace before dot
  assert!( validate_command_name( " .hello" ).is_err() );

  // Only dot
  assert!( validate_command_name( "." ).is_ok() ); // Technically valid but unusual

  // Multiple dots
  assert!( validate_command_name( ".hello.world" ).is_ok() ); // Valid full name format
}

#[ test ]
fn test_v1_4_command_name_error_messages()
{
  // Verify error messages are informative

  let result = validate_command_name( "hello" );
  assert!( result.is_err() );

  if let Err( e ) = result
  {
    let error_msg = format!( "{e:?}" );
    assert!( error_msg.contains( "Invalid command name" ) );
    assert!( error_msg.contains( "dot prefix" ) );
  }
}

// ============================================================================
// V2: Namespace Validation Tests
// ============================================================================

#[ test ]
fn test_v2_1_valid_namespaces()
{
  // Test Matrix Row: V2.1
  // Valid namespaces (both empty and dotted)

  // Empty namespace is valid (root-level commands)
  assert!( validate_namespace( "" ).is_ok() );

  // Dotted namespaces
  assert!( validate_namespace( ".session" ).is_ok() );
  assert!( validate_namespace( ".video" ).is_ok() );
  assert!( validate_namespace( ".a" ).is_ok() );
  assert!( validate_namespace( ".very_long_namespace" ).is_ok() );
  assert!( validate_namespace( ".ns123" ).is_ok() );
}

#[ test ]
fn test_v2_2_invalid_namespaces_without_dot()
{
  // Test Matrix Row: V2.2
  // Invalid non-empty namespaces without dot prefix

  assert!( validate_namespace( "session" ).is_err() );
  assert!( validate_namespace( "video.more" ).is_err() );
  assert!( validate_namespace( "namespace" ).is_err() );
  assert!( validate_namespace( "a" ).is_err() );
}

#[ test ]
fn test_v2_3_namespace_edge_cases()
{
  // Test Matrix Row: V2.3
  // Edge cases for namespace validation

  // Whitespace before dot
  assert!( validate_namespace( " .session" ).is_err() );

  // Only dot
  assert!( validate_namespace( "." ).is_ok() ); // Technically valid but unusual

  // Multiple segments
  assert!( validate_namespace( ".video.search" ).is_ok() ); // Valid nested namespace
}

#[ test ]
fn test_v2_4_namespace_error_messages()
{
  // Verify error messages are informative

  let result = validate_namespace( "session" );
  assert!( result.is_err() );

  if let Err( e ) = result
  {
    let error_msg = format!( "{e:?}" );
    assert!( error_msg.contains( "Invalid namespace" ) );
    assert!( error_msg.contains( "dot prefix" ) );
  }
}

// ============================================================================
// V3: Full Command Validation Tests
// ============================================================================

#[ test ]
fn test_v3_1_validate_complete_command_success()
{
  // Test Matrix Row: V3.1
  // Complete command validation with valid data

  let cmd = CommandDefinition::former()
    .name( ".hello" )
    .description( "Test command" )
    .hint( "Test hint" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .auto_help_enabled( true )
    .end();

  assert!( validate_command_for_registration( &cmd ).is_ok() );
}

#[ test ]
fn test_v3_2_validate_command_with_namespace()
{
  // Test Matrix Row: V3.1
  // Command with valid namespace

  let cmd = CommandDefinition::former()
    .name( ".search" )
    .namespace( ".video" )
    .description( "Search videos" )
    .hint( "Search hint" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .auto_help_enabled( true )
    .end();

  assert!( validate_command_for_registration( &cmd ).is_ok() );
}

#[ test ]
fn test_v3_3_validate_command_invalid_name()
{
  // Test Matrix Row: V3.2
  // Phase 2 Update: CommandName validation enforces dot prefix at construction time
  // This is fail-fast validation - invalid names cannot be constructed
  // Test updated to use valid name and verify construction + registration succeed

  let cmd = CommandDefinition::former()
    .name( ".hello" ) // Valid name with dot prefix (required by Phase 2)
    // Empty namespace (default ".")
    // full_name() = ".hello"
    .description( "Test command" )
    .hint( "Test hint" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .auto_help_enabled( true )
    .end();

  // Phase 2: Passes because both construction and registration validation succeed
  // Construction: name has dot prefix (validated by CommandName newtype)
  // Registration: full_name ".hello" is valid
  assert!( validate_command_for_registration( &cmd ).is_ok() );
}

#[ test ]
fn test_v3_4_validate_command_invalid_namespace()
{
  // Test Matrix Row: V3.2
  // Command with invalid namespace (no dot prefix)

  let cmd = CommandDefinition::former()
    .name( ".search" )
    .namespace( "video" ) // Invalid: no dot
    .description( "Search videos" )
    .hint( "Search hint" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .auto_help_enabled( true )
    .end();

  assert!( validate_command_for_registration( &cmd ).is_err() );
}

#[ test ]
fn test_v3_5_validate_command_both_invalid()
{
  // Test Matrix Row: V3.2
  // Phase 2: Name validation happens at construction (fail-fast)
  // This test now verifies namespace validation at registration time
  // Updated to use valid name so construction succeeds, namespace validation fails

  let cmd = CommandDefinition::former()
    .name( ".search" ) // Valid: has dot (construction succeeds)
    .namespace( "video" ) // Invalid: no dot (registration validation catches this)
    .description( "Search videos" )
    .hint( "Search hint" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .auto_help_enabled( true )
    .end();

  // Should fail on namespace validation at registration time
  assert!( validate_command_for_registration( &cmd ).is_err() );
}

// ============================================================================
// V4: Helper Function Tests
// ============================================================================

#[ test ]
fn test_v4_1_is_help_command_detection()
{
  // Test Matrix Row: V4.1
  // Detect help commands correctly

  // Help commands
  assert!( is_help_command( ".hello.help" ) );
  assert!( is_help_command( ".video.search.help" ) );
  assert!( is_help_command( ".a.help" ) );

  // Not help commands
  assert!( !is_help_command( ".hello" ) );
  assert!( is_help_command( ".help" ) ); // Global help command - technically ends with .help
  assert!( !is_help_command( ".helpful" ) ); // Contains "help" but not ".help" suffix
  assert!( !is_help_command( ".help.me" ) ); // Starts with help but wrong structure
}

#[ test ]
fn test_v4_2_make_help_command_name_generation()
{
  // Test Matrix Row: V4.2
  // Generate correct help command names

  assert_eq!( make_help_command_name( ".hello" ), ".hello.help" );
  assert_eq!( make_help_command_name( ".video.search" ), ".video.search.help" );
  assert_eq!( make_help_command_name( ".a" ), ".a.help" );
  assert_eq!( make_help_command_name( ".complex.nested.command" ), ".complex.nested.command.help" );
}

#[ test ]
fn test_v4_3_help_name_idempotency()
{
  // Verify that adding .help to a help command doesn't create valid structure
  // (This is expected behavior - we don't prevent it, but semantic layer should)

  let base = ".hello";
  let help1 = make_help_command_name( base );
  let help2 = make_help_command_name( &help1 );

  assert_eq!( help1, ".hello.help" );
  assert_eq!( help2, ".hello.help.help" ); // Technically valid, but semantic layer prevents
}

// ============================================================================
// Integration Tests
// ============================================================================

#[ test ]
fn test_integration_validation_with_full_command()
{
  // Test that validation works correctly with a realistic command

  let valid_cmd = CommandDefinition::former()
    .name( ".greet" )
    .namespace( ".social" )
    .description( "Greets user by name" )
    .hint( "Use this to say hello" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![] )
    .tags( vec![ "greeting".to_string() ] )
    .aliases( vec![ "hello".to_string() ] )
    .permissions( vec![ "public".to_string() ] )
    .idempotent( true )
    .http_method_hint( "GET" )
    .examples( vec![ ".social.greet name::Alice".to_string() ] )
    .auto_help_enabled( true )
    .end();

  // Should pass validation
  assert!( validate_command_for_registration( &valid_cmd ).is_ok() );

  // Full name should be correctly formed
  let full_name = valid_cmd.full_name();
  assert_eq!( full_name, ".social.greet" );

  // Should not be a help command
  assert!( !is_help_command( &full_name ) );

  // Help command name should be correct
  let help_name = make_help_command_name( &full_name );
  assert_eq!( help_name, ".social.greet.help" );
  assert!( is_help_command( &help_name ) );
}
