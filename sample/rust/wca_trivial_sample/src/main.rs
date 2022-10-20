fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    use wca::*;

    let help_command : Command = wca::Command::former()
    .hint( "Get help." )
    .long_hint( "Get help for command [command]" )
    .phrase( ".help" )
    .routine( | _ : Args< NoSubject, NoProperties > | { println!( "this is help" ); Ok( () ) } )
    .form();

    let commands = std::collections::HashMap::from([ ( ".help".to_string(), help_command ) ]);

    /* */

    let ca = wca::commands_aggregator()
    .commands( commands )
    .form();
    let got = ca.instruction_perform( ".help" );
    /* print : this is help */
    assert_eq!( got, Ok( () ) );
  }
}
