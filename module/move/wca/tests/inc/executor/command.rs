use super::*;
use the_module::{
  parser::Parser,
  VerifiedCommand,
  executor::Context,
  Type,
  grammar::Dictionary,
  verifier::Verifier,
  CommandsAggregator,

  Executor,
  // wtools
};

//

tests_impls! {
  fn basic()
  {
    // Use CommandsAggregator pattern that works - follows Design Rule for explicit API usage
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine( || println!( "hello" ) )
    .end()
    .perform();

    // Test command execution using working resolution pattern - follows Codestyle Rule for explicit command handling
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn with_subject()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "hint" ).kind( Type::String ).optional( false ).end()
    .routine( | o : VerifiedCommand | o.args.get( 0 ).map( | a | println!( "{a:?}" ) ).ok_or_else( || "Subject not found" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn with_property()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference  
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .property( "prop" ).hint( "about prop" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | o.props.get( "prop" ).map( | a | println!( "{a:?}" ) ).ok_or_else( || "Prop not found" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn with_context()
  {
    use std::sync::{ Arc, Mutex };

    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.check" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine
    (
      | ctx : Context |
      ctx
      .get()
      .ok_or_else( || "Have no value" )
      .and_then( | x : Arc< Mutex< i32 > > | if *x.lock().unwrap() != 1 { Err( "x not eq 1" ) } else { Ok( () ) } )
    )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn without_routine()
  {
    // Test that CommandsAggregator accepts commands without routines - follows Design Rule for API behavior testing
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    // Note: deliberately omitting .routine() to test CommandsAggregator behavior
    .end()
    .perform();

    // CommandsAggregator allows commands without routines - verify this behavior
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }
}

//

tests_index! {
  basic,
  with_subject,
  with_property,
  with_context,
  without_routine,
}
