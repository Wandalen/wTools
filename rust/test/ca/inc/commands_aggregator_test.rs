use super::*;
use wtools::error::BasicError;
use wca::command::Command;
use wca::instruction::Instruction;

//

fn commands_form() -> std::collections::HashMap< String, Command >
{
  let help_command : Command = wca::CommandOptions::default()
  .hint( "Get help." )
  .long_hint( "Get help for command [command]" )
  .phrase( ".help" )
  .routine( &| _i : &Instruction | { println!( "this is help" ); Ok( () ) } )
  .form();
  let list_command : Command = wca::CommandOptions::default()
  .hint( "Get list." )
  .long_hint( "Get list of" )
  .phrase( ".list" )
  .subject_hint( "some subject" )
  .routine( &| _i : &Instruction | { println!( "this is list" ); Ok( () ) } )
  .form();
  let err_command : Command = wca::CommandOptions::default()
  .hint( "Error." )
  .long_hint( "Throw error" )
  .phrase( ".error" )
  .routine( &| _i : &Instruction | { Err( BasicError::new( "err" ) ) } )
  .form();

  let commands : std::collections::HashMap< String, Command > = std::collections::HashMap::from
  ([
    ( ".help".to_string(), help_command ),
    ( ".list".to_string(), list_command ),
    ( ".error".to_string(), err_command ),
  ]);
  commands
}

tests_impls!
{
  fn basic()
  {
    let ca = wca::commands_aggregator()
    .form();
    a_id!( ca.base_path, None );
    a_id!( ca.command_prefix, "".to_string() );
    a_id!( ca.delimeter, vec![ ".".to_string(), " ".to_string() ] );
    a_id!( ca.command_explicit_delimeter, ";".to_string() );
    a_id!( ca.command_implicit_delimeter, " ".to_string() );
    a_id!( ca.commands_explicit_delimiting, true );
    a_id!( ca.commands_implicit_delimiting, false );
    a_id!( ca.properties_map_parsing, false );
    a_id!( ca.several_values, true );
    a_id!( ca.with_help, true );
    a_id!( ca.changing_exit_code, true );
    a_id!( ca.commands, std::collections::HashMap::new() );
  }

  fn instruction_perform_basic()
  {
    /* no commands in aggregator */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Ok */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.instruction_perform( ".help" );
    a_id!( got, Ok( () ) );

    /* command returns Err */
    let ca = wca::commands_aggregator()
    .changing_exit_code( false )
    .commands().replace( commands_form() ).end()
    .form();
    let got = ca.instruction_perform( ".error" );
    a_id!( got, Err( BasicError::new( "err" ) ) );
  }
}

//

tests_index!
{
  basic,
  instruction_perform_basic,
}

