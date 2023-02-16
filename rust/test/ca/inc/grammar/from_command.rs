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
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

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

  fn with_same_name()
  {
    let without_subject = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .form();

    let with_subject = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .subject_hint( "subject" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let converter = wca::Converter::former()
    .command( without_subject, | _ | { println!( "hello" ); Ok( () ) } )
    .command( with_subject, |( subjects, _ )| { println!( "hello, {}", subjects[ 0 ] ); Ok( () ) } )
    .form();

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
    .form();

    // init parser
    let parser = Parser::former().form();
    ;

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

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
    a_true!( exec_command.is_none() );

    // with subject and property that isn't declareted
    let raw_command = parser.command( ".command subject prop:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.properties.is_empty() );

    // with property that isn't declareted and without subject
    let raw_command = parser.command( ".command prop:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_none() );
  }

  fn properties()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .property_hint( "prop1", "hint of prop1" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

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
    a_true!( exec_command.is_none() );
  }

  fn with_context()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "with_context" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let converter = wca::Converter::former()
    .command_with_ctx( command, | _, _ | { println!( "hello" ); Ok( () ) } )
    .form();

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
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

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
  with_same_name,
  subjects,
  properties,
  with_context,
  without_context,
}