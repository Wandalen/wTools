use super::*;

//

fn ok_command_parser( parser : &Parser, command : &str ) -> RawCommand
{
  let raw_command = parser.command( command );
  a_true!( raw_command.is_ok() );
  raw_command.unwrap()
}

fn ok_command_grammar( grammar : &GrammarConverter, raw : RawCommand ) -> GrammarCommand
{
  let grammar_command = grammar.to_command( raw );
  a_true!( grammar_command.is_some() );
  grammar_command.unwrap()
}

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
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // existed command
    let raw_command = ok_command_parser( &parser, ".command" );

    let exec_command = grammar_converter.to_command( raw_command );
    a_true!( exec_command.is_some() );

    // not existed command
    let raw_command = ok_command_parser( &parser, ".invalid_command" );

    let exec_command = grammar_converter.to_command( raw_command );
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
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // with only one subject
    let raw_command = ok_command_parser( &parser, ".command subject" );
    let exec_command = ok_command_grammar( &grammar_converter, raw_command );

    a_id!( vec![ "subject".to_string() ], exec_command.subjects );
    a_true!( exec_command.properties.is_empty() );

    // with more subjects that it is setted
    let raw_command = ok_command_parser( &parser, ".command subject1 subject2" );

    let exec_command = grammar_converter.to_command( raw_command );
    a_true!( exec_command.is_none() );

    // with subject and property that isn't declareted
    let raw_command = ok_command_parser( &parser, ".command subject prop:value" );

    let exec_command = grammar_converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.properties.is_empty() );

    // with property that isn't declareted and without subject
    let raw_command = ok_command_parser( &parser, ".command prop:value" );

    let exec_command = grammar_converter.to_command( raw_command );
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
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // with only one property
    let raw_command = ok_command_parser( &parser, ".command prop1:value1" );
    let exec_command = ok_command_grammar( &grammar_converter, raw_command );

    a_true!( exec_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), "value1".to_string() ) ]), exec_command.properties );

    // with property re-write
    let raw_command = ok_command_parser( &parser, ".command prop1:value prop1:another_value" );
    let exec_command = ok_command_grammar( &grammar_converter, raw_command );

    a_true!( exec_command.subjects.is_empty() );
    a_id!( HashMap::from_iter([ ( "prop1".to_string(), "another_value".to_string() ) ]), exec_command.properties );

    // with undeclareted property
    let raw_command = ok_command_parser( &parser, ".command undeclareted_prop:value" );

    let exec_command = grammar_converter.to_command( raw_command );
    // ? or fail?
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    a_true!( exec_command.subjects.is_empty() );
    a_true!( exec_command.properties.is_empty() );

    // with undeclareted subject
    let raw_command = ok_command_parser( &parser, ".command subject prop1:value" );

    let exec_command = grammar_converter.to_command( raw_command );
    a_true!( exec_command.is_none() );
  }
}

//

tests_index!
{
  command_validation,
  subjects,
  properties,
}
