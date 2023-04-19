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

    a_true!( grammar_converter.to_command( raw_command ).is_err() );

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

  fn subject_with_list()
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
      .subject( "Subjects list", Type::List( Type::String.into(), ',' ) )
      .form()
    )
    .form();

    // with only one subject
    let raw_command = parser.command( ".command first_subject,second_subject,third_subject" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_id!( vec!
    [
      Value::List( vec!
      [
        Value::String( "first_subject".into() ),
        Value::String( "second_subject".into() ),
        Value::String( "third_subject".into() ),
      ])
    ], grammar_command.subjects );
    a_true!( grammar_command.properties.is_empty() );
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

    a_true!( grammar_converter.to_command( raw_command ).is_err() );

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

  fn property_with_list()
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
      .property( "prop", "Numbers list property", Type::List( Type::Number.into(), ',' ) )
      .form()
    )
    .form();

    // with only one subject
    let raw_command = parser.command( ".command prop:1,2,3" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!
    (
      vec![ 1.0, 2.0, 3.0 ],
      Vec::< f64 >::from( grammar_command.properties[ "prop" ].clone() )
    );
  }

  fn alias_property()
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
      .property( "property", "string property", Type::String )
      .property_alias( "property", "prop" )
      .property_alias( "property", "p" )
      .form()
    )
    .form();

    // basic
    let raw_command = parser.command( ".command property:value" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // first alias
    let raw_command = parser.command( ".command prop:value" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // second alias
    let raw_command = parser.command( ".command p:value" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // init converter with layered properties 
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "property", "string property", Type::String )
      // .property( "property", "number property with alredy used name", Type::Number ) // panic because this property name alredy used
      .property_alias( "property", "p" )
      // .property_alias( "property", "proposal" ) // panic at next property beacuse this name alredy used as alias
      .property( "proposal", "string property", Type::String )
      // .property_alias( "proposal", "property" ) // panic because this name alredy used as property name
      // .property_alias( "proposal", "p" ) // panic because this alias alredy used
      .form()
    )
    .form();

    let raw_command = parser.command( ".command p:value" ).unwrap();
    let grammar_command = grammar_converter.to_command( raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );
  }
}

//

tests_index!
{
  command_validation,
  subjects,
  subject_type_check,
  subject_with_list,
  properties,
  property_type_check,
  property_with_list,
  alias_property,
}
