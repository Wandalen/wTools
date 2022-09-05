use super::*;
// qqq : for Dima : bad /* aaa : Dmytro : fixed */

//

tests_impls!
{
  fn basic()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "phrase" )
    .subject_hint( "subject_hint" )
    .property_hint( "prop1", "hint of prop1" )
    .property_hint( "prop2", "hint of prop2" )
    .property_alias( "property_alias", "a1" )
    .property_alias( "property_alias", "a2" )
    .routine( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();

    dbg!( &command );

    a_id!( command.hint, "hint".to_string() );
    a_id!( command.long_hint, "long_hint".to_string() );
    a_id!( command.phrase, "phrase".to_string() );
    a_id!( command.subject_hint, "subject_hint".to_string() );

    let properties_hints = hmap!
    {
      "prop1".to_string() => "hint of prop1".to_string(),
      "prop2".to_string() => "hint of prop2".to_string(),
    };
    a_id!( command.properties_hints, properties_hints );

    let properties_aliases = hmap!
    {
      "property_alias".to_string() => vec![ "a1".to_string(), "a2".to_string() ],
    };
    a_id!( command.properties_aliases, properties_aliases );
  }

  //

  fn shortcut()
  {
    let command = wca::Command::former()
    .h( "hint2" )
    .lh( "long_hint2" )
    .ro( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();

    dbg!( &command );

    a_id!( command.hint, "hint2".to_string() );
    a_id!( command.long_hint, "long_hint2".to_string() );
    a_id!( command.phrase, "".to_string() );
    a_id!( command.subject_hint, "".to_string() );
  }

  //

  fn perform_trivial()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "phrase" )
    .subject_hint( "subject_hint" )
    .property_hint( "prop1", "hint of prop1" )
    .property_hint( "prop2", "hint of prop2" )
    .property_alias( "property_alias", "a1" )
    .property_alias( "property_alias", "a2" )
    .routine( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();

    let instruction = wca::instruction::instruction_parse()
    .instruction( "" )
    .perform();
    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );
  }

  //

  fn perform_with_subject()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .subject_hint( "" )
    .routine( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();
    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get subj" )
    .perform();
    let perform = command.perform( &instruction );
    assert!( perform.is_err() );

    let command = wca::Command::former()
    .hint( "hint" )
    .subject_hint( "subject" )
    .routine( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();
    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get subj" )
    .perform();
    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );
  }

  //

  fn perform_with_props()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "phrase" )
    .subject_hint( "subject_hint" )
    .property_hint( "prop1", "hint of prop1" )
    .property_hint( "prop2", "hint of prop2" )
    .property_alias( "property_alias", "a1" )
    .property_alias( "property_alias", "a2" )
    .routine( &| _i : &wca::instruction::Instruction | { println!( "hello" ); Ok( () ) } )
    .form();

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get subj prop1:1" )
    .perform();
    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );

    let instruction = wca::instruction::instruction_parse()
    .instruction( ".get subj unknown:1" )
    .perform();
    let perform = command.perform( &instruction );
    assert!( perform.is_err() );
  }
}

//

tests_index!
{
  basic,
  shortcut,
  perform_trivial,
  perform_with_subject,
  perform_with_props,
}
