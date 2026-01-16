//! Regression tests for namespace format validation.
//!
//! ## Lessons Learned (Bugs Fixed)
//!
//! - **2026-01-11 (issue-namespace-format):** Examples used plain string namespaces without
//!   dot prefix. Validation requires non-empty namespaces start with '.'.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]

use unilang::{ CommandDefinition, CommandRegistry };

// test_kind: bug_reproducer(issue-namespace-format)
#[ test ]
fn test_namespace_requires_dot_prefix()
{
  let mut registry = CommandRegistry::new();

  let mut cmd = CommandDefinition::former()
    .name( ".test_command" )
    .description( "Test command" )
    .end();

  // Invalid: namespace without dot prefix
  cmd.namespace = "collections".to_string();

  let result = registry.register( cmd );

  assert!(
    result.is_err(),
    "Namespace without dot prefix should be rejected"
  );

  let error = result.unwrap_err().to_string();
  assert!(
    error.contains( "namespace" ) && error.contains( "dot prefix" ),
    "Error should mention namespace format requirement, got: {}",
    error
  );
}

#[ test ]
fn test_valid_namespace_accepted()
{
  let mut registry = CommandRegistry::new();

  // Valid: namespace with dot prefix
  let mut cmd = CommandDefinition::former()
    .name( ".test_command" )
    .description( "Test command" )
    .end();

  cmd.namespace = ".collections".to_string();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Valid namespace with dot prefix should be accepted, got error: {:?}",
    result.err()
  );
}

#[ test ]
fn test_empty_namespace_accepted()
{
  let mut registry = CommandRegistry::new();

  // Valid: empty namespace (root-level command)
  let cmd = CommandDefinition::former()
    .name( ".root_command" )
    .description( "Root command" )
    .namespace( String::new() )
    .end();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Empty namespace should be valid, got error: {:?}",
    result.err()
  );
}
