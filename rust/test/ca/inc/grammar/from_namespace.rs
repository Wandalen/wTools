use super::*;

//

fn ok_namespace_parser( parser : &Parser, namespace : &str ) -> Namespace< RawCommand >
{
  let raw_namespace = parser.namespace( namespace );
  a_true!( raw_namespace.is_ok() );
  raw_namespace.unwrap()
}

fn ok_namespace_grammar( grammar : &GrammarConverter, raw : Namespace< RawCommand > ) -> Namespace< GrammarCommand >
{
  let grammar_namespace = grammar.to_namespace( raw );
  // a_true!( grammar_namespace.is_some() );
  // grammar_namespace.unwrap()
  grammar_namespace
}

tests_impls!
{
  fn basic()
  {
    let command1 = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command1" )
    .subject_hint( "subject" )
    .form();

    let command2 = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command2" )
    .subject_hint( "subject" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( command1 )
    .command( command2 )
    .form();

    // parse namespace with only one command
    let raw_namespace = ok_namespace_parser( &parser, ".command1 subject" );

    // convert namespace
    let exec_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    a_true!( exec_namespace.commands.len() == 1 );
    a_id!( vec![ "subject".to_string() ], exec_namespace.commands[ 0 ].subjects );

    // parse namespace with only several command
    let raw_namespace = ok_namespace_parser( &parser, ".command1 first_subj .command2 second_subj" );

    // convert namespace
    let exec_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    a_true!( exec_namespace.commands.len() == 2 );
    a_id!( vec![ "first_subj".to_string() ], exec_namespace.commands[ 0 ].subjects );
    a_id!( vec![ "second_subj".to_string() ], exec_namespace.commands[ 1 ].subjects );
  }

  fn with_invalid_command()
  {
    let command1 = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command1" )
    .subject_hint( "subject" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( command1 )
    .form();

    // parse namespace with only several command
    let raw_namespace = ok_namespace_parser( &parser, ".command1 first_subj .invalid_command second_subj" );

    // convert namespace
    // ? Or it must fail beacause of unknown command?
    let exec_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    a_true!( exec_namespace.commands.len() == 1 );
    a_id!( vec![ "first_subj".to_string() ], exec_namespace.commands[ 0 ].subjects );
  }
}

//

tests_index!
{
  basic,
  with_invalid_command
}
