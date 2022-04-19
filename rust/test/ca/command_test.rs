
use wca::*;
use maplit::hashmap;
use wtest_basic::*;

//

fn _basic()
{
  let command = CommandOptions::default()
  .hint( "hint" )
  .long_hint( "long_hint" )
  .phrase( "phrase" )
  .subject_hint( "subject_hint" )
  .property_hint( "prop1", "hint of prop1" )
  .property_hint( "prop2", "hint of prop2" )
  .property_alias( "property_alias", "a1" )
  .property_alias( "property_alias", "a2" )
  .routine( &| _i : &instruction::Instruction | { println!( "hello" ); Ok( () ) } )
  .form();

  dbg!( &command );

  assert_eq!( command.hint, "hint".to_string() );
  assert_eq!( command.long_hint, "long_hint".to_string() );
  assert_eq!( command.phrase, "phrase".to_string() );
  assert_eq!( command.subject_hint, "subject_hint".to_string() );

  let properties_hints = hashmap!
  {
    "prop1".to_string() => "hint of prop1".to_string(),
    "prop2".to_string() => "hint of prop2".to_string(),
  };
  assert_eq!( command.properties_hints, properties_hints );

  let properties_aliases = hashmap!
  {
    "property_alias".to_string() => vec![ "a1".to_string(), "a2".to_string() ],
  };
  assert_eq!( command.properties_aliases, properties_aliases );
}

//

fn _shortcut()
{
  let command = CommandOptions::default()
  .h( "hint2" )
  .lh( "long_hint2" )
  .ro( &| _i : &instruction::Instruction | { println!( "hello" ); Ok( () ) } )
  .form();

  dbg!( &command );

  assert_eq!( command.hint, "hint2".to_string() );
  assert_eq!( command.long_hint, "long_hint2".to_string() );
  assert_eq!( command.phrase, "".to_string() );
  assert_eq!( command.subject_hint, "".to_string() );
}

//

fn _callback_trivial()
{
  let command = CommandOptions::default()
  .hint( "hint" )
  .long_hint( "long_hint" )
  .phrase( "phrase" )
  .subject_hint( "subject_hint" )
  .property_hint( "prop1", "hint of prop1" )
  .property_hint( "prop2", "hint of prop2" )
  .property_alias( "property_alias", "a1" )
  .property_alias( "property_alias", "a2" )
  .routine( &| _i : &instruction::Instruction | { println!( "hello" ); Ok( () ) } )
  .form();

  let perform = command.perform( &instruction::Instruction::default() );
  assert!( perform.is_ok() );
}

//

test_suite!
{
  basic,
  shortcut,
  callback_trivial,
}

