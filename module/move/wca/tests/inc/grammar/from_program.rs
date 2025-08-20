use super::*;

use the_module::{parser::Parser, Type, Value, grammar::Dictionary, verifier::Verifier, CommandsAggregator};

//

tests_impls! {
  fn basic()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command1" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "subject" ).kind( Type::String ).optional( true ).end()
    .routine( || println!( "command1" ) )
    .end()
    .command( "cmd.command2" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "subject" ).kind( Type::String ).optional( true ).end()
    .routine( || println!( "command2" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }
}

//

tests_index! {
  basic,
}
