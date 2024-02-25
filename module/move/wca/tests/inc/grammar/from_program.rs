use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command1" )
      .subject( "subject", Type::String, true )
      .form()
    )
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .subject( "subject", Type::String, true )
      .form()
    )
    .form();

    // parse program with only one command
    let raw_program = parser.program( ".command1 subject" ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( raw_program ).unwrap();
    a_true!( grammar_program.namespaces.len() == 1 );
    a_true!( grammar_program.namespaces[ 0 ].commands.len() == 1 );
    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_program.namespaces[ 0 ].commands[ 0 ].subjects );

    // parse program several namespaces
    let raw_program = parser.program( ".command1 first_subj .also .command2 second_subj" ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( raw_program ).unwrap();
    a_true!( grammar_program.namespaces.len() == 2 );
    a_true!( grammar_program.namespaces[ 0 ].commands.len() == 1 );
    a_id!( vec![ Value::String( "first_subj".to_string() ) ], grammar_program.namespaces[ 0 ].commands[ 0 ].subjects );
    a_true!( grammar_program.namespaces[ 1 ].commands.len() == 1 );
    a_id!( vec![ Value::String( "second_subj".to_string() ) ], grammar_program.namespaces[ 1 ].commands[ 0 ].subjects );
  }
}

//

tests_index!
{
  basic,
}
