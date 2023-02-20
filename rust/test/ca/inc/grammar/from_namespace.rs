use super::*;

//

tests_impls!
{
  fn basic()
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
      .phrase( "command1" )
      .subject( "subject", Type::String )
      .form()
    )
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .subject( "subject", Type::String )
      .form()
    )
    .form();

    // parse namespace with only one command
    let raw_namespace = parser.namespace( ".command1 subject" ).unwrap();

    // convert namespace
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace ).unwrap();
    a_true!( grammar_namespace.commands.len() == 1 );
    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_namespace.commands[ 0 ].subjects );

    // parse namespace with only several command
    let raw_namespace = parser.namespace( ".command1 first_subj .command2 second_subj" ).unwrap();

    // convert namespace
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace ).unwrap();
    a_true!( grammar_namespace.commands.len() == 2 );
    a_id!( vec![ Value::String( "first_subj".to_string() ) ], grammar_namespace.commands[ 0 ].subjects );
    a_id!( vec![ Value::String( "second_subj".to_string() ) ], grammar_namespace.commands[ 1 ].subjects );
  }

  fn with_invalid_command()
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
      .phrase( "command1" )
      .subject( "subject", Type::String )
      .form()
    )
    .form();

    // parse namespace with only several command
    let raw_namespace = parser.namespace( ".command1 first_subj .invalid_command second_subj" ).unwrap();

    // convert namespace
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace );
    a_true!( grammar_namespace.is_err() );
  }
}

//

tests_index!
{
  basic,
  with_invalid_command
}
