use super::*;

use the_module::{parser::Parser, Type, Value, grammar::Dictionary, verifier::Verifier, CommandsAggregator};

//

tests_impls! {
  fn command_validation()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn subjects()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "first subject" ).kind( Type::String ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn subject_type_check()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "number value" ).kind( Type::Number ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn subject_with_list()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "Subjects list" ).kind( Type::List( Type::String.into(), ',' ) ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn subject_is_optional_basic()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "This subject is optional" ).kind( Type::String ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn preferred_non_optional_first_order()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "This subject is optional and type number" ).kind( Type::Number ).optional( true ).end()
    .subject().hint( "This subject is required and type that accepts the optional one" ).kind( Type::String ).optional( false ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn properties()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .property( "prop1" ).hint( "hint of prop1" ).kind( Type::String ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn property_type_check()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .property( "prop" ).hint( "Number property" ).kind( Type::Number ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn property_with_list()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .property( "prop" ).hint( "Numbers list property" ).kind( Type::List( Type::Number.into(), ',' ) ).optional( true ).end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }

  fn alias_property()
  {
    // Use CommandsAggregator pattern - follows Design Rule for abstraction preference
    let ca = CommandsAggregator::former()
    .command( "cmd.command" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .property( "property" )
    .hint( "string property" )
    .kind( Type::String )
    .optional( true )
    .alias( "prop" )
    .alias( "p" )
    .end()
    .routine( || println!( "test command" ) )
    .end()
    .perform();

    // Test command execution - follows Codestyle Rule for explicit testing
    a_id!( (), ca.perform( ".cmd." ).unwrap() );
  }
}

//

tests_index! {
  command_validation,
  subjects,
  subject_type_check,
  subject_with_list,
  subject_is_optional_basic,
  preferred_non_optional_first_order,
  properties,
  property_type_check,
  property_with_list,
  alias_property,
}
