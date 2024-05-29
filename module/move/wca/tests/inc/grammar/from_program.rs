use super::*;

//

tests_impls!
{
  fn basic()
  {
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command1" )
      .subject().hint( "subject" ).kind( Type::String ).optional( true ).end()
      .form()
    )
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .subject().hint( "subject" ).kind( Type::String ).optional( true ).end()
      .form()
    )
    .form();
    let verifier = Verifier;

    // parse program with only one command
    let raw_program = parser.parse( [ ".command1", "subject" ] ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();
    a_true!( grammar_program.commands.len() == 1 );
    a_id!( vec![ Value::String( "subject".to_string() ) ], grammar_program.commands[ 0 ].args.0 );

    // parse program several commands
    let raw_program = parser.parse( [ ".command1", "first_subj", ".command2", "second_subj" ] ).unwrap();

    // convert program
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();
    a_true!( grammar_program.commands.len() == 2 );
    a_id!( vec![ Value::String( "first_subj".to_string() ) ], grammar_program.commands[ 0 ].args.0 );
    a_id!( vec![ Value::String( "second_subj".to_string() ) ], grammar_program.commands[ 1 ].args.0 );
  }
}

//

tests_index!
{
  basic,
}
