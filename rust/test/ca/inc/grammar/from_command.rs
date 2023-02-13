use super::*;

//

tests_impls!
{
  fn command_validation()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .routine( | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // init parser
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::from( vec![ command ] );

    // existed command
    let raw_command = parser.command( ".command" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );

    // not existed command
    let raw_command = parser.command( ".invalid_command" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_none() );

    // invalid command syntax
    let raw_command = parser.command( "invalid_command" );
    a_true!( raw_command.is_err() );
  }

  fn subjects()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .subject_hint( "first subject" )
    .routine( | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // init parser
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::from( vec![ command ] );

    // with only one subject
    let raw_command = parser.command( ".command subject" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_id!( vec![ "subject".to_string() ], exec_command.subjects );
    a_true!( exec_command.properties.is_empty() );

    // with more subjects that it is setted
    let raw_command = parser.command( ".command subject1 subject2" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    // ! FAILS
    // ? what it must to do?
    // * take all subjects that user give
    // * take a certain number of items and error if there are more or less (or ignore the rest) or what?
    a_id!( vec![ "subject1".to_string() ], exec_command.subjects );
    a_true!( exec_command.properties.is_empty() );

    // with property. It isn't declareted
    let raw_command = parser.command( ".command prop:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_true!( exec_command.properties.is_empty() );
  }

  fn properties()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .property_hint( "prop1", "hint of prop1" )
    .routine( | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // init parser
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::from( vec![ command ] );

    // with only one property
    let raw_command = parser.command( ".command prop1:value1" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), "value1".to_string() ) ]), exec_command.properties );

    // with property re-write
    let raw_command = parser.command( ".command prop1:value prop1:another_value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), "another_value".to_string() ) ]), exec_command.properties );

    // with undeclareted property
    let raw_command = parser.command( ".command undeclareted_prop:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_true!( exec_command.properties.is_empty() );

    // with undeclareted subject
    let raw_command = parser.command( ".command subject prop1:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), "value".to_string() ) ]), exec_command.properties );
  }

  fn with_context()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "with_context" )
    .routine_with_ctx( | _, _ : wca::Context | { println!( "hello" ); Ok( () ) } )
    .form();

    // init parser
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::from( vec![ command ] );

    // parse command
    let raw_command = parser.command( ".with_context" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    // convert command
    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_true!( exec_command.properties.is_empty() );
    a_true!( matches!( exec_command.routine, wca::Routine::WithContext( _ ) ) );
  }

  fn without_context()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "without_context" )
    .routine( | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // init parser
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::from( vec![ command ] );

    // parse command
    let raw_command = parser.command( ".without_context" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    // convert command
    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_true!( exec_command.properties.is_empty() );
    a_true!( matches!( exec_command.routine, wca::Routine::WithoutContext( _ ) ) );
  }
}

//


tests_index!
{
  command_validation,
  subjects,
  properties,
  with_context,
  without_context,
}