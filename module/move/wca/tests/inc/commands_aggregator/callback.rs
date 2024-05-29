use super::*;
use std::sync::{ Arc, Mutex };

#[ test ]
fn changes_state_of_local_variable_on_perform()
{
  let history = Arc::new( Mutex::new( vec![] ) );

  let ca_history = Arc::clone( &history );
  let ca = CommandsAggregator::former()
  .command( "command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine( || println!( "command" ) )
    .end()
  .command( "command2" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine( || println!( "command2" ) )
    .end()
  .callback
  (
    move | input, program |
    ca_history.lock().unwrap()
    .push(
    (
      input.to_string(),
      program.commands.clone() )
    ))
  .perform();

  {
    assert!( history.lock().unwrap().is_empty() );
  }

  {
    ca.perform( ".command" ).unwrap();
    let current_history = history.lock().unwrap();
    assert_eq!( [ ".command" ], current_history.iter().map( |( input, _ )| input ).collect::< Vec< _ > >().as_slice() );
    assert_eq!( 1, current_history.len() );
  }

  {
    ca.perform( ".command2" ).unwrap();
    let current_history = history.lock().unwrap();
    assert_eq!( [ ".command", ".command2" ], current_history.iter().map( |( input, _ )| input ).collect::< Vec< _ > >().as_slice() );
    assert_eq!( 2, current_history.len() );
  }
}
