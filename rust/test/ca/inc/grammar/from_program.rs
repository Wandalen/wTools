use super::*;

//

fn ok_program_parser( parser : &Parser, program : &str ) -> Program< Namespace< RawCommand > >
{
  let raw_program = parser.program( program );
  a_true!( raw_program.is_ok() );
  raw_program.unwrap()
}

fn ok_program_grammar( grammar : &GrammarConverter, raw : Program< Namespace< RawCommand > > ) -> Program< Namespace< GrammarCommand > >
{
  let grammar_program = grammar.to_program( raw );
  // a_true!( grammar_program.is_some() );
  // grammar_program.unwrap()
  grammar_program
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

    // parse program with only one command
    let raw_program = ok_program_parser( &parser, ".command1 subject" );

    // convert program
    let exec_program = ok_program_grammar( &grammar_converter, raw_program );
    a_true!( exec_program.namespaces.len() == 1 );
    a_true!( exec_program.namespaces[ 0 ].commands.len() == 1 );
    a_id!( vec![ "subject".to_string() ], exec_program.namespaces[ 0 ].commands[ 0 ].subjects );

    // parse program several namespaces
    let raw_program = ok_program_parser( &parser, ".command1 first_subj .also .command2 second_subj" );

    // convert program
    let exec_program = ok_program_grammar( &grammar_converter, raw_program );
    a_true!( exec_program.namespaces.len() == 2 );
    a_true!( exec_program.namespaces[ 0 ].commands.len() == 1 );
    a_id!( vec![ "first_subj".to_string() ], exec_program.namespaces[ 0 ].commands[ 0 ].subjects );
    a_true!( exec_program.namespaces[ 1 ].commands.len() == 1 );
    a_id!( vec![ "second_subj".to_string() ], exec_program.namespaces[ 1 ].commands[ 0 ].subjects );
  }
}

//

tests_index!
{
  basic,
}
