use super::*;

//

tests_impls!
{
  fn command_validation()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .form();
    let verifier = Verifier;

    // existed command
    let raw_command = parser.command( ".command" ).unwrap();

    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // not existed command
    let raw_command = parser.command( ".invalid_command" ).unwrap();

    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );

    // invalid command syntax
    let raw_command = parser.command( "invalid_command" );
    a_true!( raw_command.is_err() );
  }

  fn subjects()
  {
    // init parser
    let parser = Parser::former().form();
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "first subject" ).kind( Type::String ).end()
      .form()
    )
    .form();

    // init converter
    let verifier = Verifier;

    // with only one subject
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_command.subjects );
    a_true!( grammar_command.properties.is_empty() );

    // with more subjects that it is set
    let raw_command = parser.command( ".command subject1 subject2" ).unwrap();

    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );

    // with subject and property that isn't declared
    let raw_command = parser.command( ".command subject prop:value" ).unwrap();

    a_true!( verifier.to_command( dictionary, raw_command ).is_err() );

    // subject with colon when property not declared
    let raw_command = parser.command( ".command prop:value" ).unwrap();

    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
    a_id!( vec![ Value::String( "prop:value".to_string() ) ], grammar_command.subjects );
    a_true!( grammar_command.properties.is_empty() );
  }

  fn subject_type_check()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "number value" ).kind( Type::Number ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // string when number expected
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );

    // valid negative float number when number expected
    let raw_command = parser.command( ".command -3.14" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
  }

  fn subject_with_list()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "Subjects list" ).kind( Type::List( Type::String.into(), ',' ) ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // with only one subject
    let raw_command = parser.command( ".command first_subject,second_subject,third_subject" ).unwrap();
    let grammar_command = verifier.to_command( &dictionary, raw_command ).unwrap();

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

  fn subject_is_optional_basic()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "This subject is optional" ).kind( Type::String ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // with subject
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // without subject
    let raw_command = parser.command( ".command" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
  }

  fn preferred_non_optional_first_order()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "This subject is optional and type number" ).kind( Type::Number ).optional( true ).end()
      .subject().hint( "This subject is required and type that accepts the optional one" ).kind( Type::String ).optional( false ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // second subject is required, but missing
    let raw_command = parser.command( ".command 42" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err(), "subject identifies as first subject" );

    // first subject is missing
    let raw_command = parser.command( ".command valid_string" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // both subjects exists
    let raw_command = parser.command( ".command 42 string" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // first subject not a number, but both arguments exists
    let raw_command = parser.command( ".command not_a_number string" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err(), "first subject not a number" );
  }

  fn properties()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop1" ).hint( "hint of prop1" ).kind( Type::String ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // with only one property
    let raw_command = parser.command( ".command prop1:value1" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), Value::String( "value1".to_string() ) ) ]), grammar_command.properties );

    // with property re-write
    let raw_command = parser.command( ".command prop1:value prop1:another_value" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), Value::String( "another_value".to_string() ) ) ]), grammar_command.properties );

    // with undeclareted property
    let raw_command = parser.command( ".command undeclareted_prop:value" ).unwrap();

    a_true!( verifier.to_command( dictionary, raw_command ).is_err() );

    // with undeclareted subject
    let raw_command = parser.command( ".command subject prop1:value" ).unwrap();

    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn property_type_check()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop" ).hint( "Number property" ).kind( Type::Number ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // string when number expected
    let raw_command = parser.command( ".command prop:Property" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );

    // valid negative float number when number expected
    let raw_command = parser.command( ".command prop:-3.14" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
  }

  fn property_with_list()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop" ).hint( "Numbers list property" ).kind( Type::List( Type::Number.into(), ',' ) ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // with only one subject
    let raw_command = parser.command( ".command prop:1,2,3" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

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
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "property" )
        .hint( "string property" )
        .kind( Type::String )
        .optional( true )
        .alias( "prop" )
        .alias( "p" )
        .end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // basic
    let raw_command = parser.command( ".command property:value" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // first alias
    let raw_command = parser.command( ".command prop:value" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // second alias
    let raw_command = parser.command( ".command p:value" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( grammar_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "property".to_string(), Value::String( "value".to_string() ) ) ]), grammar_command.properties );

    // init converter with layered properties
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "property" ).hint( "string property" ).kind( Type::String ).optional( true ).alias( "p" ).end()
      .property( "proposal" ).hint( "string property" ).kind( Type::String ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    let raw_command = parser.command( ".command p:value" ).unwrap();
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

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
  subject_is_optional_basic,
  preferred_non_optional_first_order,
  properties,
  property_type_check,
  property_with_list,
  alias_property,
}
