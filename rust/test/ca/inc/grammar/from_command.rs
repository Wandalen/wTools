use super::*;

//

tests_impls!
{
  fn command_validation()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .form();

    // existed command
    let raw_command = parser.command( ".command" ).unwrap();

    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    // not existed command
    let raw_command = parser.command( ".invalid_command" ).unwrap();

    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );

    // invalid command syntax
    let raw_command = parser.command( "invalid_command" );
    a_true!( raw_command.is_err() );
  }

  fn subjects()
  {
    // init parser
    let parser = Parser::former().form();
    ;

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "first subject", Type::String )
      .form()
    )
    .form();

    // with only one subject
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_command.subjects );
    a_true!( grammar_command.properties.is_empty() );

    // with more subjects that it is setted
    let raw_command = parser.command( ".command subject1 subject2" ).unwrap();

    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );

    // with subject and property that isn't declareted
    let raw_command = parser.command( ".command subject prop:value" ).unwrap();

    // ? or fail?
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.properties.is_empty() );

    // with property that isn't declareted and without subject
    let raw_command = parser.command( ".command prop:value" ).unwrap();

    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn subject_type_check()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "number value", Type::Number )
      .form()
    )
    .form();

    // string when number expected
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );

    // valid negative float number when number expected
    let raw_command = parser.command( ".command -3.14" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();
  }

  fn properties()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop1", "hint of prop1", Type::String )
      .form()
    )
    .form();

    // with only one property
    let raw_command = parser.command( ".command prop1:value1" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), Value::String( "value1".to_string() ) ) ]), grammar_command.properties );

    // with property re-write
    let raw_command = parser.command( ".command prop1:value prop1:another_value" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), Value::String( "another_value".to_string() ) ) ]), grammar_command.properties );

    // with undeclareted property
    let raw_command = parser.command( ".command undeclareted_prop:value" ).unwrap();

    // ? or fail?
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_true!( grammar_command.properties.is_empty() );

    // with undeclareted subject
    let raw_command = parser.command( ".command subject prop1:value" ).unwrap();

    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn property_type_check()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop", "Number property", Type::Number )
      .form()
    )
    .form();

    // string when number expected
    let raw_command = parser.command( ".command prop:Property" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_err() );

    // valid negative float number when number expected
    let raw_command = parser.command( ".command prop:-3.14" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();
  }
}

//

tests_index!
{
  command_validation,
  subjects,
  subject_type_check,
  properties,
  property_type_check,
}
