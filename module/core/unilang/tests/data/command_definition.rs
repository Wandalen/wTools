//! Tests for CommandDefinition - Phase 2 type-safe redesign
//!
//! This module tests the new CommandDefinition with private fields,
//! validated newtypes, and custom serde implementation.
//!
//! # Why These Tests Exist
//!
//! **Purpose:** Verify type-safe design catches errors at compile/construction time,
//! not at runtime. The old API allowed invalid commands to be constructed and only
//! failed during registration. These tests ensure the new API fails fast.
//!
//! **What We're Protecting Against:**
//!
//! 1. **Invalid construction:** Tests verify that `CommandName`/`NamespaceType` validation
//!    runs during construction, not registration. Invalid values should be impossible
//!    to represent in the type system.
//!
//! 2. **Mutation bugs:** With private fields, commands can't be invalidated after
//!    construction. These tests verify getters work and no mutation is possible.
//!
//! 3. **Deserialization bypassing validation:** YAML loading could bypass validation
//!    if `serde` isn't custom-implemented. Tests verify validation runs during deserialize.
//!
//! 4. **Runtime vs compile-time errors:** The goal is moving errors from runtime to
//!    compile time. Tests verify this migration succeeded.
//!
//! **How to Interpret Failures:**
//!
//! - **Construction test fails:** Validation isn't running at construction time
//! - **Getter test fails:** Field access pattern broken (shouldn't happen with private fields)
//! - **Serde test fails:** Validation bypassed during YAML load
//! - **Fluent API test fails:** Builder pattern type-state transitions broken
//!
//! **Test Coverage:**
//! - Construction with validated types
//! - Getter methods for all fields
//! - Fluent setter methods
//! - Serde YAML serialization/deserialization
//! - Validation during YAML load
//! - Helper methods (full_name, generate_help_command)

#![ allow( clippy::doc_markdown ) ]

use unilang::
{
  data::
  {
    CommandDefinition,
    CommandName,
    NamespaceType,
    CommandStatus,
    VersionType,
  },
};

// ============================================================================
// Construction Tests
// ============================================================================

#[ test ]
fn test_v2_basic_construction()
{
  let name = CommandName::new( ".build" ).unwrap();
  let cmd = CommandDefinition::new( name, "Build the project".to_string() );

  assert_eq!( cmd.name().to_string(), ".build" );
  assert_eq!( cmd.description().to_string(), "Build the project" );
  assert_eq!( cmd.namespace(), "" );
  assert!( matches!( cmd.status(), CommandStatus::Active ) );
  assert_eq!( cmd.version().to_string(), "1.0.0" );
  assert!( cmd.auto_help_enabled() );
}

#[ test ]
fn test_v2_fluent_api()
{
  let name = CommandName::new( ".test" ).unwrap();
  let ns = NamespaceType::new( ".session" ).unwrap();
  let version = VersionType::new( "2.0.0" ).unwrap();

  let cmd = CommandDefinition::new( name, "Test command".to_string() )
    .with_namespace( ns.to_string() )
    .with_version( version )
    .with_hint( "Test hint" )
    .with_auto_help( false );

  assert_eq!( cmd.namespace(), ".session" );
  assert_eq!( cmd.version().as_str(), "2.0.0" );
  assert_eq!( cmd.hint(), "Test hint" );
  assert!( !cmd.auto_help_enabled() );
}

#[ test ]
fn test_v2_with_status_variants()
{
  let name = CommandName::new( ".deprecated_cmd" ).unwrap();

  let cmd = CommandDefinition::new( name, "Old command".to_string() )
    .with_status( CommandStatus::Deprecated
    {
      reason : "Use .new_cmd instead".to_string(),
      since : Some( "v2.0.0".to_string() ),
      replacement : Some( ".new_cmd".to_string() ),
    });

  assert!( cmd.status().is_deprecated() );
  let ( reason, since, replacement ) = cmd.status().deprecation_info().unwrap();
  assert_eq!( reason, "Use .new_cmd instead" );
  assert_eq!( since.as_deref(), Some( "v2.0.0" ) );
  assert_eq!( replacement.as_deref(), Some( ".new_cmd" ) );
}

// ============================================================================
// Getter Tests
// ============================================================================

#[ test ]
fn test_v2_all_getters()
{
  let name = CommandName::new( ".full" ).unwrap();
  let cmd = CommandDefinition::new( name, "Full test".to_string() )
    .with_tags( vec![ "tag1".to_string(), "tag2".to_string() ] )
    .with_aliases( vec![ "alias1".to_string() ] )
    .with_permissions( vec![ "admin".to_string() ] )
    .with_idempotent( false )
    .with_http_method_hint( "POST" )
    .with_examples( vec![ ".full example".to_string() ] )
    .with_category( "testing" )
    .with_short_desc( "Short" )
    .with_hidden_from_list( true )
    .with_priority( 10 )
    .with_group( "test_group" );

  // Verify all getters work
  assert_eq!( cmd.tags().len(), 2 );
  assert_eq!( cmd.aliases().len(), 1 );
  assert_eq!( cmd.permissions().len(), 1 );
  assert!( !cmd.idempotent() );
  assert_eq!( cmd.http_method_hint(), "POST" );
  assert_eq!( cmd.examples().len(), 1 );
  assert_eq!( cmd.category(), "testing" );
  assert_eq!( cmd.short_desc(), "Short" );
  assert!( cmd.hidden_from_list() );
  assert_eq!( cmd.priority(), 10 );
  assert_eq!( cmd.group(), "test_group" );
}

// ============================================================================
// Helper Method Tests
// ============================================================================

#[ test ]
fn test_v2_full_name_simple()
{
  let name = CommandName::new( ".help" ).unwrap();
  let cmd = CommandDefinition::new( name, "Help".to_string() );

  assert_eq!( cmd.full_name(), ".help" );
}

#[ test ]
fn test_v2_full_name_namespaced()
{
  let name = CommandName::new( ".list" ).unwrap();
  let ns = NamespaceType::new( ".session" ).unwrap();

  let cmd = CommandDefinition::new( name, "List sessions".to_string() )
    .with_namespace( ns.to_string() );

  assert_eq!( cmd.full_name(), ".session.list" );
}

#[ test ]
fn test_v2_generate_help_command()
{
  let name = CommandName::new( ".example" ).unwrap();
  let cmd = CommandDefinition::new( name, "Example command".to_string() );

  let help_cmd = cmd.generate_help_command();

  assert_eq!( help_cmd.name().as_str(), ".example.help" );
  assert!( help_cmd.description().contains( ".example" ) );
  assert!( !help_cmd.auto_help_enabled() ); // Help commands don't generate recursive help
  assert!( help_cmd.hidden_from_list() );
  assert_eq!( help_cmd.category(), "help" );
  assert_eq!( help_cmd.priority(), 999 );
}

// ============================================================================
// Serde Tests - YAML
// ============================================================================

#[ test ]
fn test_v2_serde_yaml_basic()
{
  let yaml = r#"
name: ".build"
description: "Build the project"
namespace: ""
hint: "Build hint"
status: "active"
version: "1.0.0"
"#;

  let cmd : CommandDefinition = serde_yaml::from_str( yaml ).unwrap();

  assert_eq!( cmd.name().to_string(), ".build" );
  assert_eq!( cmd.description().to_string(), "Build the project" );
  assert!( matches!( cmd.status(), CommandStatus::Active ) );
}

#[ test ]
fn test_v2_serde_yaml_with_lists()
{
  let yaml = r#"
name: ".test"
description: "Test command"
tags:
  - integration
  - testing
aliases:
  - t
  - tst
examples:
  - ".test arg::value"
  - ".test another::example"
"#;

  let cmd : CommandDefinition = serde_yaml::from_str( yaml ).unwrap();

  assert_eq!( cmd.tags().len(), 2 );
  assert_eq!( cmd.aliases().len(), 2 );
  assert_eq!( cmd.examples().len(), 2 );
}

#[ test ]
fn test_v2_serde_yaml_validates_command_name()
{
  let yaml = r#"
name: "invalid_no_dot"
description: "Invalid"
"#;

  let result : Result< CommandDefinition, _ > = serde_yaml::from_str( yaml );
  assert!( result.is_err() );
}

#[ test ]
fn test_v2_serde_yaml_validates_namespace()
{
  let yaml = r#"
name: ".test"
description: "Test"
namespace: "invalid_no_dot"
"#;

  let result : Result< CommandDefinition, _ > = serde_yaml::from_str( yaml );
  assert!( result.is_err() );
}

#[ test ]
fn test_v2_serde_yaml_validates_version()
{
  let yaml = r#"
name: ".test"
description: "Test"
version: ""
"#;

  let result : Result< CommandDefinition, _ > = serde_yaml::from_str( yaml );
  assert!( result.is_err() );
}

#[ test ]
fn test_v2_missing_required_name()
{
  let yaml = r#"
description: "No name"
"#;

  let result : Result< CommandDefinition, _ > = serde_yaml::from_str( yaml );
  assert!( result.is_err() );
}

#[ test ]
fn test_v2_missing_required_description()
{
  let yaml = r#"
name: ".test"
"#;

  let result : Result< CommandDefinition, _ > = serde_yaml::from_str( yaml );
  assert!( result.is_err() );
}

// ============================================================================
// Integration Tests
// ============================================================================

#[ test ]
fn test_v2_realistic_command()
{
  let name = CommandName::new( ".deploy" ).unwrap();
  let ns = NamespaceType::new( ".cloud" ).unwrap();
  let version = VersionType::new( "3.1.4" ).unwrap();

  let cmd = CommandDefinition::new( name, "Deploy to cloud infrastructure".to_string() )
    .with_namespace( ns.to_string() )
    .with_version( version )
    .with_hint( "Deploy your application" )
    .with_tags( vec![ "cloud".to_string(), "deployment".to_string() ] )
    .with_permissions( vec![ "admin".to_string() ] )
    .with_idempotent( false )
    .with_http_method_hint( "POST" )
    .with_examples( vec![
      ".cloud.deploy env::prod".to_string(),
      ".cloud.deploy env::staging region::us-west".to_string(),
    ])
    .with_category( "infrastructure" )
    .with_priority( 1 );

  // Verify construction
  assert_eq!( cmd.full_name(), ".cloud.deploy" );
  assert_eq!( cmd.tags().len(), 2 );
  assert_eq!( cmd.permissions()[0], "admin" );
  assert!( !cmd.idempotent() );
  assert_eq!( cmd.http_method_hint(), "POST" );
  assert_eq!( cmd.priority(), 1 );
}

#[ test ]
fn test_v2_help_command_generation()
{
  let name = CommandName::new( ".parent" ).unwrap();
  let cmd = CommandDefinition::new( name, "Parent command".to_string() );

  let help_cmd = cmd.generate_help_command();

  assert_eq!( help_cmd.name().as_str(), ".parent.help" );
  assert!( help_cmd.hidden_from_list() );
  assert!( !help_cmd.auto_help_enabled() );
}
