use super::*;
use wtools::error::BasicError;
use wca::
{
  Args,
  NoSubject,
  NoProperties,
  InstructionParser,
  Properties,
  string::parse_request::OpType,
};

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
    .routine(  | _ : Args< NoSubject, NoProperties >  | { println!( "hello" ); Ok( () ) } )
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
    .ro(  | _ : Args< NoSubject, NoProperties >  | { println!( "hello" ); Ok( () ) } )
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
    .routine( | _ : Args< NoSubject, NoProperties > | { println!( "hello" ); Ok( () ) } )
    .form();

    let instruction = wca::instruction::DefaultInstructionParser::former()
    .form()
    .parse( ".phrase" )
    .unwrap();

    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );
  }

  //

  fn perform_with_subject()
  {
    let parser = wca::instruction::DefaultInstructionParser::former().form();

    let command = wca::Command::former()
    .hint( "hint" )
    .subject_hint( "" )
    .routine(  | _ : Args< String, NoProperties >  | { println!( "hello" ); Ok( () ) } )
    .form();
    let instruction = parser.parse( ".get subj" ).unwrap();
    let perform = command.perform( &instruction );
    assert!( perform.is_err() );

    let command = wca::Command::former()
    .hint( "hint" )
    .subject_hint( "subject" )
    .routine(  | _ : Args< String, NoProperties >  | { println!( "hello" ); Ok( () ) } )
    .form();
    let instruction = parser.parse( ".get subj" ).unwrap();
    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );
  }

  //

  fn perform_with_props()
  {
    struct TestProperties
    {
      prop1: i32,
    }
    // TODO: Replace with the derive macro.
    impl Properties for TestProperties
    {
      fn parse( properties : &HashMap< String, OpType< String > > ) -> Result< Self, BasicError >
      {
        if let Some( prop1 ) = properties.get( "prop1" )
        {
          let props = TestProperties
          {
            prop1: prop1.clone().primitive().unwrap().parse().unwrap(),
          };

          return Ok( props );
        }

        Err( BasicError::new( "Not found" ) )
      }
    }

    let parser = wca::instruction::DefaultInstructionParser::former().form();

    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "phrase" )
    .subject_hint( "subject_hint" )
    .property_hint( "prop1", "hint of prop1" )
    .property_hint( "prop2", "hint of prop2" )
    .property_alias( "property_alias", "a1" )
    .property_alias( "property_alias", "a2" )
    .routine(  | _ : Args< String, TestProperties >  | { println!( "hello" ); Ok( () ) } )
    .form();

    let instruction = parser.parse( ".get subj prop1:1" ).unwrap();
    let perform = command.perform( &instruction );
    assert!( perform.is_ok() );

    let instruction = parser.parse( ".get subj unknown:1" ).unwrap();
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
