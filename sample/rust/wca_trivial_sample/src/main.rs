use wca::*;
use wca::instruction::Instruction;

fn main()
{
  let help_command : Command = wca::CommandOptions::default()
  .hint( "Get help." )
  .long_hint( "Get help for command [command]" )
  .phrase( ".help" )
  .routine( &| _i : &Instruction | { println!( "this is help" ); Ok( () ) } )
  .form();

  let commands = std::collections::HashMap::from([ ( ".help".to_string(), help_command ) ]);

  /* */

  let ca = wca::commands_aggregator()
  .commands().replace( commands ).end()
  .form();
  let got = ca.instruction_perform( ".help" );
  /* print : this is help */
  assert_eq!( got, Ok( () ) );
}
