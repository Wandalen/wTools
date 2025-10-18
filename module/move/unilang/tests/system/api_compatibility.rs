//! Test Matrix for Public API Accessibility
//!
//! | ID   | Test Case                          | Expected Result                     |
//! |------|------------------------------------|-------------------------------------|
//! | T1.1 | Import from root namespace         | All core types accessible           |
//! | T1.2 | Import from prelude                | Essential types accessible          |
//! | T1.3 | Import from specific modules       | Module-specific types accessible    |
//! | T1.4 | Create basic command flow          | Full workflow compiles and runs     |

/// Tests that core types can be imported from the root namespace.
/// Test Combination: T1.1
#[ test ]
fn test_root_namespace_imports()
{
  // These imports should work from the root namespace
  use unilang::CommandRegistry;
  use unilang::CommandDefinition;
  use unilang::ArgumentDefinition;
  use unilang::Kind;
  use unilang::OutputData;
  use unilang::ErrorData;
  use unilang::Value;
  use unilang::Pipeline;
  use unilang::VerifiedCommand;
  use unilang::ExecutionContext;
  use unilang::ArgumentAttributes;
  
  // Verify types exist by creating instances or references
  #[allow(deprecated)]
  #[allow(deprecated)]
  let _registry = CommandRegistry::new();
  let kind = Kind::String;
  let _attrs = ArgumentAttributes::default();
  
  // Use the types to avoid unused warnings
  assert_eq!(core::mem::size_of::<CommandDefinition>(), core::mem::size_of::<CommandDefinition>());
  assert_eq!(core::mem::size_of::<ArgumentDefinition>(), core::mem::size_of::<ArgumentDefinition>());
  assert_eq!(core::mem::size_of::<OutputData>(), core::mem::size_of::<OutputData>());
  assert_eq!(core::mem::size_of::<ErrorData>(), core::mem::size_of::<ErrorData>());
  let _value = Value::String("test".to_string());
  assert_eq!(core::mem::size_of::<Pipeline>(), core::mem::size_of::<Pipeline>());
  assert_eq!(core::mem::size_of::<VerifiedCommand>(), core::mem::size_of::<VerifiedCommand>());
  let _ctx = ExecutionContext::default();
  assert_eq!(core::mem::size_of::<Kind>(), core::mem::size_of::<Kind>());
  core::hint::black_box(kind);
}

/// Tests that essential types can be imported from prelude.
/// Test Combination: T1.2
#[ test ]
fn test_prelude_imports()
{
  use unilang::prelude::*;
  
  // Verify prelude contains essential types
  #[allow(deprecated)]
  #[allow(deprecated)]
  let _registry = CommandRegistry::new();
  core::hint::black_box(Kind::String);
  let _output = OutputData
  {
    content : "test".to_string(),
    format : "text".to_string(),
  };
}

/// Tests that types can be imported from specific modules.
/// Test Combination: T1.3
#[ test ]
fn test_module_specific_imports()
{
  // Data module
  use unilang::data::
  {
    CommandDefinition,
    ArgumentDefinition,
    Kind,
    OutputData,
    ErrorData,
    ArgumentAttributes,
  };
  
  // Types module
  use unilang::types::
  {
    Value,
  };
  
  // Registry module
  use unilang::registry::
  {
    CommandRegistry,
    CommandRoutine,
  };
  
  // Import ExecutionContext from interpreter
  use unilang::interpreter::ExecutionContext;
  
  // Semantic module
  use unilang::semantic::
  {
    VerifiedCommand,
  };
  
  // Pipeline module
  use unilang::pipeline::
  {
    Pipeline,
    CommandResult,
    BatchResult,
    process_single_command,
    validate_single_command,
  };
  
  // Help module
  use unilang::help::HelpGenerator;
  
  // Verify imports work by using all types
  #[allow(deprecated)]
  #[allow(deprecated)]
  let registry = CommandRegistry::new();
  let _value = Value::String( "test".to_string() );
  core::hint::black_box(Kind::String);
  let _attrs = ArgumentAttributes::default();
  core::hint::black_box(Option::<CommandDefinition>::None);
  core::hint::black_box(Option::<ArgumentDefinition>::None);
  // Test that all major types are accessible from prelude
  assert_eq!(core::mem::size_of::<OutputData>(), core::mem::size_of::<OutputData>());
  assert_eq!(core::mem::size_of::<ErrorData>(), core::mem::size_of::<ErrorData>());
  assert_eq!(core::mem::size_of::<CommandRoutine>(), core::mem::size_of::<CommandRoutine>());
  let ctx = ExecutionContext::default();
  assert_eq!(core::mem::size_of::<ExecutionContext>(), core::mem::size_of::<ExecutionContext>());
  assert_eq!(core::mem::size_of::<VerifiedCommand>(), core::mem::size_of::<VerifiedCommand>());
  assert_eq!(core::mem::size_of::<Pipeline>(), core::mem::size_of::<Pipeline>());
  assert_eq!(core::mem::size_of::<CommandResult>(), core::mem::size_of::<CommandResult>());
  assert_eq!(core::mem::size_of::<BatchResult>(), core::mem::size_of::<BatchResult>());
  // Test that convenience functions are accessible
  let _process_result = process_single_command(".help", &registry, ctx.clone());
  let _validate_result = validate_single_command(".help", &registry);
  let _help_gen = HelpGenerator::new(&registry);
}

/// Tests a complete workflow using the public API.
/// Test Combination: T1.4
#[ test ]
fn test_complete_workflow()
{
  use unilang::prelude::*;
  use unilang::
  {
    ArgumentAttributes,
    VerifiedCommand,
    ExecutionContext,
    CommandRoutine,
  };
  
  // Create a registry
  #[allow(deprecated)]
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  
  // Define a command
  let greet_cmd = CommandDefinition::former()
    .name( ".greet" )
    .namespace( String::new() )
    .description( "Greets a person".to_string() )
    .hint( "Simple greeting" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( true )
    .deprecation_message( String::new() )
    .http_method_hint( "GET".to_string() )
    .examples( vec![ ".greet name::\"Alice\"".to_string() ] )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "name" )
        .kind( Kind::String )
        .hint( "Person to greet" )
        .description( "Name of person to greet".to_string() )
        .attributes( ArgumentAttributes::default() )
        .validation_rules( vec![] )
        .aliases( vec![] )
        .tags( vec![] )
        .end()
    ])
    .end();
  
  // Define a routine
  let routine : CommandRoutine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
  {
    let name = cmd.arguments.get( "name" )
      .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
      .unwrap_or_else( || "World".to_string() );
    
    Ok( OutputData
    {
      content : format!( "Hello, {name}!" ),
      format : "text".to_string(),
    })
  });
  
  // Register the command
  #[allow(deprecated)]
    #[allow(deprecated)]
  registry.command_add_runtime( &greet_cmd, routine )
    .expect( "Failed to register command" );
  
  // Verify command was registered - registry doesn't expose commands() method
  
  // Test with Pipeline API
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".greet name::\"Test\"" );
  
  assert!( result.success );
  assert_eq!( result.outputs[ 0 ].content, "Hello, Test!" );
}

/// Tests that namespace re-exports work correctly.
/// This ensures the `mod_interface` pattern is properly implemented.
#[ test ]
fn test_namespace_structure()
{
  // Test own namespace (if it exists)
  // use unilang::own::*;
  use unilang::CommandRegistry;
  #[allow(deprecated)]
  #[allow(deprecated)]
  let _registry = CommandRegistry::new();
  
  // Test exposed namespace exists and has expected types
  {
    use unilang::exposed::*;
    assert_eq!(core::mem::size_of::<CommandDefinition>(), core::mem::size_of::<CommandDefinition>());
  }
  
  // Test orphan namespace exists and has expected types
  {
    use unilang::orphan::*;
    assert_eq!(core::mem::size_of::<Kind>(), core::mem::size_of::<Kind>());
  }
}

/// Tests that commonly needed type combinations work together.
#[ test ]
fn test_common_use_patterns()
{
  // Pattern 1: Minimal imports for basic usage
  use unilang::{ CommandRegistry, Pipeline };
  
  #[allow(deprecated)]
  #[allow(deprecated)]
  let registry = CommandRegistry::new();
  let _pipeline = Pipeline::new( registry );
  
  // Pattern 2: Import for command definition
  use unilang::
  {
    CommandDefinition,
    ArgumentDefinition,
    Kind,
    ArgumentAttributes,
  };
  
  let _cmd = CommandDefinition::former()
    .name( ".test" )
    .namespace( String::new() )
    .description( "Test command".to_string() )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "arg" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes::default() )
        .end()
    ])
    .end();
  
  // Pattern 3: Import for error handling
  use unilang::ErrorData;
  
  let _error_data = ErrorData::new(
    "TEST001".to_string(),
    "Test error".to_string(),
  );
}