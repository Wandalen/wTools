//! Tests for command validation module.
//!
//! This module tests the centralized validation logic used across all command
//! registration approaches to ensure consistent behavior.
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

  let cmd = CommandDefinition
  {
    name : ".hello".to_string(),
    namespace : String::new(),
    description : "Test command".to_string(),
    hint : "Test hint".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
    auto_help_enabled : true,
    ..Default::default()
  };

  assert!( validate_command_for_registration( &cmd ).is_ok() );
}

#[ test ]
fn test_v3_2_validate_command_with_namespace()
{
  // Test Matrix Row: V3.1
  // Command with valid namespace

  let cmd = CommandDefinition
  {
    name : ".search".to_string(),
    namespace : ".video".to_string(),
    description : "Search videos".to_string(),
    hint : "Search hint".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
    auto_help_enabled : true,
    ..Default::default()
  };

  assert!( validate_command_for_registration( &cmd ).is_ok() );
}

#[ test ]
fn test_v3_3_validate_command_invalid_name()
{
  // Test Matrix Row: V3.2
  // Command with invalid name (no dot prefix)

  let cmd = CommandDefinition
  {
    name : "hello".to_string(), // Invalid: no dot
    namespace : String::new(),
    description : "Test command".to_string(),
    hint : "Test hint".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
    auto_help_enabled : true,
    ..Default::default()
  };

  assert!( validate_command_for_registration( &cmd ).is_err() );
}

#[ test ]
fn test_v3_4_validate_command_invalid_namespace()
{
  // Test Matrix Row: V3.2
  // Command with invalid namespace (no dot prefix)

  let cmd = CommandDefinition
  {
    name : ".search".to_string(),
    namespace : "video".to_string(), // Invalid: no dot
    description : "Search videos".to_string(),
    hint : "Search hint".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
    auto_help_enabled : true,
    ..Default::default()
  };

  assert!( validate_command_for_registration( &cmd ).is_err() );
}

#[ test ]
fn test_v3_5_validate_command_both_invalid()
{
  // Test Matrix Row: V3.2
  // Command with both name and namespace invalid

  let cmd = CommandDefinition
  {
    name : "search".to_string(), // Invalid: no dot
    namespace : "video".to_string(), // Invalid: no dot
    description : "Search videos".to_string(),
    hint : "Search hint".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![],
    auto_help_enabled : true,
    ..Default::default()
  };

  // Should fail on name validation first
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

  let valid_cmd = CommandDefinition
  {
    name : ".greet".to_string(),
    namespace : ".social".to_string(),
    description : "Greets user by name".to_string(),
    hint : "Use this to say hello".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    arguments : vec![],
    routine_link : None,
    tags : vec![ "greeting".to_string() ],
    aliases : vec![ "hello".to_string() ],
    permissions : vec![ "public".to_string() ],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : "GET".to_string(),
    examples : vec![ ".social.greet name::Alice".to_string() ],
    auto_help_enabled : true,
    ..Default::default()
  };

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
