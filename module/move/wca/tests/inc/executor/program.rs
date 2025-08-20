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
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine( || println!( "hello" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn with_context()
  {
    use std::sync::{ Arc, Mutex };

    // Use CommandsAggregator pattern for simpler test - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.inc" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine
    (
      | ctx : Context |
      ctx
      .get()
      .ok_or_else( || "Have no value" )
      .and_then( | x : Arc< Mutex< i32 > > | { *x.lock().unwrap() += 1; Ok( () ) } )
    )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }
}

//

tests_index! {
  basic,
  with_context,
}
