//! Integration tests for `unilang::parser` re-export.
//!
//! Verifies that `unilang_parser` is correctly re-exported as `unilang::parser`,
//! providing access to all parser functionality including `cli_parser` module.

// Test that parser module is accessible via re-export
#[ test ]
fn parser_reexport_basic_types()
{
  // Verify Parser and options are accessible
  let options = unilang::parser::UnilangParserOptions::default();
  let parser = unilang::parser::Parser::new( options );

  // Parse a simple instruction
  let result = parser.parse_single_instruction( "cmd arg1 arg2" );
  assert!( result.is_ok() );

  let instruction = result.unwrap();
  assert_eq!( instruction.command_path_slices.len(), 1 );
  assert_eq!( instruction.command_path_slices[ 0 ], "cmd" );
}

// Test that cli_parser module is accessible via re-export
#[ test ]
fn parser_reexport_cli_parser_basic()
{
  use unilang::parser::cli_parser::{ parse_cli_args, CliParams, CliParseResult };

  #[ derive( Default ) ]
  struct TestParams
  {
    timeout: u64,
  }

  impl CliParams for TestParams
  {
    fn process_param( &mut self, key : &str, value : &str ) -> Result< bool, String >
    {
      match key
      {
        "timeout" =>
        {
          self.timeout = value.parse().map_err( | e | format!( "{e}" ) )?;
          Ok( true )
        }
        _ => Ok( false ),
      }
    }
  }

  let args = vec![ "timeout::5000".to_string(), "hello".to_string(), "world".to_string() ];
  let result : CliParseResult< TestParams > = parse_cli_args( &args ).unwrap();

  assert_eq!( result.params.timeout, 5000 );
  assert_eq!( result.message, "hello world" );
}

// Test that cli_parser advanced features are accessible via re-export
#[ test ]
fn parser_reexport_cli_parser_advanced()
{
  use std::collections::{ BTreeSet, HashMap };
  use unilang::parser::cli_parser::{ CliParser, CliParamsAdvanced, CliParseResultAdvanced };

  type TestConfig = HashMap< String, u64 >;

  #[ derive( Default ) ]
  struct AdvancedParams
  {
    verbosity: u8,
  }

  impl CliParamsAdvanced< TestConfig > for AdvancedParams
  {
    fn process_param( &mut self, key : &str, value : &str ) -> Result< Option< &'static str >, String >
    {
      match key
      {
        "v" | "verbosity" =>
        {
          self.verbosity = value.parse().map_err( | e | format!( "{e}" ) )?;
          Ok( Some( "verbosity" ) )
        }
        _ => Ok( None ),
      }
    }

    fn apply_defaults( &mut self, config : &TestConfig, explicit : &BTreeSet< String > )
    {
      if !explicit.contains( "verbosity" )
      {
        self.verbosity = ( *config.get( "verbosity" ).unwrap_or( &2 ) ).min( 255 ) as u8;
      }
    }
  }

  let mut config = TestConfig::new();
  config.insert( "verbosity".to_string(), 3 );

  let args = vec![ "hello".to_string() ];
  let result : CliParseResultAdvanced< AdvancedParams > = CliParser::new()
    .with_config( &config )
    .parse( &args )
    .unwrap();

  // Config default applied since verbosity not explicitly set
  assert_eq!( result.params.verbosity, 3 );
  assert_eq!( result.message, "hello" );
  assert!( result.explicit_params.is_empty() );
}

// Test prelude accessibility
#[ test ]
fn parser_reexport_prelude()
{
  // Verify prelude items are accessible
  use unilang::parser::prelude::*;

  let options = UnilangParserOptions::default();
  let parser = Parser::new( options );
  let result = parser.parse_single_instruction( "test" );
  assert!( result.is_ok() );
}
