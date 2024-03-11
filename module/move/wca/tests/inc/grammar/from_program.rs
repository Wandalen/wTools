use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser::former().form();

    // init converter
    let dictionary = &Dictionary::former()
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
    let verifier = Verifier;

    // parse program with only one command
    let raw_program = parser.program( ".command1 subject" ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();
    a_true!( grammar_program.commands.len() == 1 );
    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_program.commands[ 0 ].subjects );

    // parse program several commands
    let raw_program = parser.program( ".command1 first_subj .command2 second_subj" ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();
    a_true!( grammar_program.commands.len() == 2 );
    a_id!( vec![ Value::String( "first_subj".to_string() ) ], grammar_program.commands[ 0 ].subjects );
    a_id!( vec![ Value::String( "second_subj".to_string() ) ], grammar_program.commands[ 1 ].subjects );
  }
}

//

tests_index!
{
  basic,
}
