//!
//! Task 024: Additional Integration & Stress Tests for Multiple Parameter Collection
//!
//! This test suite provides additional comprehensive coverage beyond the core test suite,
//! focusing on integration scenarios, edge cases, and stress testing to ensure the
//! Task 024 fix is robust in real-world usage scenarios.
//!

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Test routine for stress tests
#[allow(clippy::unnecessary_wraps)]
fn stress_test_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Stress test executed successfully".to_string(),
    format : "text".to_string(),
  })
}

/// Helper to parse and analyze a command
fn parse_and_analyze( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_single_instruction( input )
    .map_err( |e| format!( "Parse error: {:?}", e ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  let verified_commands = analyzer.analyze()
    .map_err( |e| format!( "Semantic analysis error: {:?}", e ) )?;

  Ok( verified_commands )
}

/// Helper to extract string list from Value
fn extract_string_list( value : &Value ) -> Vec< String >
{
  match value {
    Value::List( list ) => list.iter().map( |v| match v {
      Value::String( s ) => s.clone(),
      _ => panic!( "Expected string in list" ),
    }).collect(),
    Value::String( s ) => vec![ s.clone() ],
    _ => panic!( "Expected string or list" ),
  }
}

/// Test multiple parameters with different data types
#[ test ]
fn test_multiple_parameters_mixed_types()
{
  let mut registry = CommandRegistry::new();

  // Create command with mixed parameter types
  let cmd = CommandDefinition::former()
    .name( ".mixed_test" )
    .description( "Test command with mixed parameter types" )
    .arguments( vec![
      ArgumentDefinition {
        name : "files".to_string(),
        description : "File paths".to_string(),
        kind : Kind::String,
        hint : "File paths".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "count".to_string(),
        description : "Count values".to_string(),
        kind : Kind::Integer,
        hint : "Integer counts".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : true,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "enabled".to_string(),
        description : "Boolean flags".to_string(),
        kind : Kind::Boolean,
        hint : "Boolean values".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : true,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  let input = r#".mixed_test files::"file1.txt" files::"file2.txt" count::10 count::20 enabled::true enabled::false"#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Mixed types test should succeed" );

  let verified_cmd = &verified_commands[0];

  // Check files (strings)
  let files_value = verified_cmd.arguments.get( "files" ).unwrap();
  let files = extract_string_list( files_value );
  assert_eq!( files.len(), 2 );
  assert_eq!( files[0], "file1.txt" );
  assert_eq!( files[1], "file2.txt" );

  // Check counts (integers)
  let count_value = verified_cmd.arguments.get( "count" ).unwrap();
  match count_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      match ( &list[0], &list[1] ) {
        ( Value::Integer( a ), Value::Integer( b ) ) => {
          assert_eq!( *a, 10 );
          assert_eq!( *b, 20 );
        },
        _ => panic!( "Expected integers in count list" ),
      }
    },
    _ => panic!( "Count should be a list" ),
  }

  // Check enabled (booleans)
  let enabled_value = verified_cmd.arguments.get( "enabled" ).unwrap();
  match enabled_value {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      match ( &list[0], &list[1] ) {
        ( Value::Boolean( a ), Value::Boolean( b ) ) => {
          assert!( *a );
          assert!( !*b );
        },
        _ => panic!( "Expected booleans in enabled list" ),
      }
    },
    _ => panic!( "Enabled should be a list" ),
  }

  println!( "✅ Mixed types multiple parameters: files={:?}, counts=[10,20], enabled=[true,false]", files );
}

/// Test very large number of multiple parameters (stress test)
#[ test ]
fn test_stress_many_multiple_parameters()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".stress_test" )
    .description( "Stress test with many parameters" )
    .arguments( vec![
      ArgumentDefinition {
        name : "item".to_string(),
        description : "Items to process".to_string(),
        kind : Kind::String,
        hint : "String items".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Create input with 100 parameters
  let mut input = String::from( ".stress_test" );
  for i in 1..=100 {
    input.push_str( &format!( r#" item::"item_{}""#, i ) );
  }

  let start = std::time::Instant::now();
  let verified_commands = parse_and_analyze( &registry, &input ).expect( "Stress test should succeed" );
  let duration = start.elapsed();

  let verified_cmd = &verified_commands[0];
  let item_value = verified_cmd.arguments.get( "item" ).unwrap();
  let items = extract_string_list( item_value );

  assert_eq!( items.len(), 100, "Should have 100 items" );
  assert_eq!( items[0], "item_1" );
  assert_eq!( items[49], "item_50" );
  assert_eq!( items[99], "item_100" );

  // Performance requirement: should complete quickly
  assert!( duration.as_millis() < 500, "Stress test should complete in under 500ms, took: {:?}", duration );

  println!( "✅ Stress test: 100 multiple parameters processed in {:?}", duration );
}

/// Test multiple parameters with complex quoted values
#[ test ]
fn test_multiple_parameters_complex_quoting()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".quote_test" )
    .description( "Test complex quoting scenarios" )
    .arguments( vec![
      ArgumentDefinition {
        name : "command".to_string(),
        description : "Commands with complex quotes".to_string(),
        kind : Kind::String,
        hint : "Shell commands".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Test complex quoting scenarios
  let input = r#".quote_test command::"echo 'hello world'" command::"grep \"pattern\" file.txt" command::"ls -la /path/with spaces/""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Complex quoting should succeed" );

  let verified_cmd = &verified_commands[0];
  let command_value = verified_cmd.arguments.get( "command" ).unwrap();
  let commands = extract_string_list( command_value );

  assert_eq!( commands.len(), 3 );
  assert_eq!( commands[0], "echo 'hello world'" );
  assert_eq!( commands[1], r#"grep "pattern" file.txt"# );
  assert_eq!( commands[2], "ls -la /path/with spaces/" );

  println!( "✅ Complex quoting: {:?}", commands );
}

/// Test multiple parameters with error conditions
#[ test ]
fn test_multiple_parameters_error_conditions()
{
  let mut registry = CommandRegistry::new();

  // Command that requires specific types
  let cmd = CommandDefinition::former()
    .name( ".type_test" )
    .description( "Test type validation with multiple parameters" )
    .arguments( vec![
      ArgumentDefinition {
        name : "numbers".to_string(),
        description : "Integer numbers".to_string(),
        kind : Kind::Integer,
        hint : "Integer values".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Test valid case first
  let valid_input = r#".type_test numbers::10 numbers::20 numbers::30"#;
  let result = parse_and_analyze( &registry, valid_input );
  assert!( result.is_ok(), "Valid numbers should work" );

  // Test invalid type case
  let invalid_input = r#".type_test numbers::10 numbers::"not_a_number" numbers::30"#;
  let result = parse_and_analyze( &registry, invalid_input );
  assert!( result.is_err(), "Invalid number should fail" );

  println!( "✅ Error conditions handled correctly" );
}

/// Test multiple parameters with aliases
#[ test ]
fn test_multiple_parameters_with_aliases()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".alias_test" )
    .description( "Test aliases with multiple parameters" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input values".to_string(),
        kind : Kind::String,
        hint : "Input strings".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "i".to_string(), "inp".to_string() ],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Test mixing canonical name and aliases
  let input = r#".alias_test input::"value1" i::"value2" inp::"value3""#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Aliases should work" );

  let verified_cmd = &verified_commands[0];
  let input_value = verified_cmd.arguments.get( "input" ).unwrap();
  let inputs = extract_string_list( input_value );

  assert_eq!( inputs.len(), 3 );
  assert_eq!( inputs[0], "value1" );
  assert_eq!( inputs[1], "value2" );
  assert_eq!( inputs[2], "value3" );

  println!( "✅ Aliases with multiple parameters: {:?}", inputs );
}

/// Test multiple parameters with real-world command patterns
#[ test ]
fn test_real_world_command_patterns()
{
  let mut registry = CommandRegistry::new();

  // Simulate a realistic build system command
  let cmd = CommandDefinition::former()
    .name( ".build" )
    .description( "Build system command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "target".to_string(),
        description : "Build targets".to_string(),
        kind : Kind::String,
        hint : "Target names".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "t".to_string() ],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "feature".to_string(),
        description : "Features to enable".to_string(),
        kind : Kind::String,
        hint : "Feature names".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : true,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "f".to_string() ],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "jobs".to_string(),
        description : "Number of parallel jobs".to_string(),
        kind : Kind::Integer,
        hint : "Job count".to_string(),
        attributes : ArgumentAttributes {
          multiple : false,
          optional : true,
          default : Some( "1".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "j".to_string() ],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Realistic build command
  let input = r#".build target::"lib" target::"bin" feature::"simd" feature::"json" feature::"cli" jobs::4"#;
  let verified_commands = parse_and_analyze( &registry, input ).expect( "Real-world pattern should succeed" );

  let verified_cmd = &verified_commands[0];

  // Check targets
  let target_value = verified_cmd.arguments.get( "target" ).unwrap();
  let targets = extract_string_list( target_value );
  assert_eq!( targets, vec![ "lib", "bin" ] );

  // Check features
  let feature_value = verified_cmd.arguments.get( "feature" ).unwrap();
  let features = extract_string_list( feature_value );
  assert_eq!( features, vec![ "simd", "json", "cli" ] );

  // Check jobs (single value)
  let jobs_value = verified_cmd.arguments.get( "jobs" ).unwrap();
  match jobs_value {
    Value::Integer( n ) => assert_eq!( *n, 4 ),
    _ => panic!( "Jobs should be integer" ),
  }

  println!( "✅ Real-world pattern: targets={:?}, features={:?}, jobs=4", targets, features );
}

/// Test backward compatibility with existing single parameter usage
#[ test ]
fn test_backward_compatibility_comprehensive()
{
  let mut registry = CommandRegistry::new();

  // Traditional single parameter command
  let cmd = CommandDefinition::former()
    .name( ".traditional" )
    .description( "Traditional single parameter command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "file".to_string(),
        description : "Single file".to_string(),
        kind : Kind::String,
        hint : "File path".to_string(),
        attributes : ArgumentAttributes {
          multiple : false, // Explicitly single
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition {
        name : "mode".to_string(),
        description : "Processing mode".to_string(),
        kind : Kind::String,
        hint : "Mode name".to_string(),
        attributes : ArgumentAttributes {
          multiple : false,
          optional : true,
          default : Some( "auto".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Test 1: Single parameter stays single
  let single_input = r#".traditional file::"single.txt""#;
  let result1 = parse_and_analyze( &registry, single_input ).expect( "Single parameter should work" );
  let file_value1 = result1[0].arguments.get( "file" ).unwrap();
  match file_value1 {
    Value::String( s ) => assert_eq!( s, "single.txt" ),
    _ => panic!( "Single parameter should remain as string" ),
  }

  // Test 2: Multiple parameters automatically collected
  let multiple_input = r#".traditional file::"first.txt" file::"second.txt""#;
  let result2 = parse_and_analyze( &registry, multiple_input ).expect( "Multiple parameters should be auto-collected" );
  let file_value2 = result2[0].arguments.get( "file" ).unwrap();
  match file_value2 {
    Value::List( list ) => {
      assert_eq!( list.len(), 2 );
      let files = extract_string_list( file_value2 );
      assert_eq!( files, vec![ "first.txt", "second.txt" ] );
    },
    _ => panic!( "Multiple parameters should be collected into list" ),
  }

  // Test 3: Default values still work
  let default_input = r#".traditional file::"test.txt""#;
  let result3 = parse_and_analyze( &registry, default_input ).expect( "Default should work" );
  let mode_value = result3[0].arguments.get( "mode" ).unwrap();
  match mode_value {
    Value::String( s ) => assert_eq!( s, "auto" ),
    _ => panic!( "Default value should be string" ),
  }

  println!( "✅ Backward compatibility: single params preserved, multiple auto-collected, defaults work" );
}

/// Performance benchmark for multiple parameter processing
#[ test ]
fn test_performance_benchmark()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".perf_bench" )
    .description( "Performance benchmark command" )
    .arguments( vec![
      ArgumentDefinition {
        name : "data".to_string(),
        description : "Data items".to_string(),
        kind : Kind::String,
        hint : "Data values".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.command_add_runtime( &cmd, Box::new( stress_test_routine ) ).unwrap();

  // Benchmark different parameter counts
  let test_sizes = vec![ 10, 50, 100, 500 ];

  for size in test_sizes {
    let mut input = String::from( ".perf_bench" );
    for i in 1..=size {
      input.push_str( &format!( r#" data::"data_item_{}""#, i ) );
    }

    let start = std::time::Instant::now();
    let verified_commands = parse_and_analyze( &registry, &input ).expect( "Benchmark should succeed" );
    let duration = start.elapsed();

    let verified_cmd = &verified_commands[0];
    let data_value = verified_cmd.arguments.get( "data" ).unwrap();
    let data_items = extract_string_list( data_value );

    assert_eq!( data_items.len(), size, "Should have {} items", size );

    // Performance requirement: should scale reasonably and be efficient
    // Allow more time for larger datasets due to semantic analysis complexity
    let max_duration_ms = if size <= 100 { size as u64 * 5 } else { size as u64 * 10 }; // 5ms per item for small sets, 10ms for large
    assert!( duration.as_millis() <= max_duration_ms as u128,
             "Processing {} items should take under {}ms, took: {:?}",
             size, max_duration_ms, duration );

    println!( "✅ Performance: {} items processed in {:?}", size, duration );
  }
}