use super::*;

//

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
    let converter = wca::Converter::former()
    .command( command1, | _ | { println!( "hello" ); Ok( () ) } )
    .command( command2, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // parse program with only one command
    let raw_program = parser.program( ".command1 subject" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    // convert program
    let exec_program = converter.to_program( raw_program );
    a_true!( exec_program.namespaces.len() == 1 );
    a_true!( exec_program.namespaces[ 0 ].commands.len() == 1 );
    a_id!( vec![ "subject".to_string() ], exec_program.namespaces[ 0 ].commands[ 0 ].subjects );

    // parse program several namespaces
    let raw_program = parser.program( ".command1 first_subj .also .command2 second_subj" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    // convert program
    let exec_program = converter.to_program( raw_program );
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
