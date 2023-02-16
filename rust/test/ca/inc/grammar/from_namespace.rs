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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : ".also".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command1, | _ | { println!( "hello" ); Ok( () ) } )
    .command( command2, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // parse namespace with only one command
    let raw_namespace = parser.namespace( ".command1 subject" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    // convert namespace
    let exec_namespace = converter.to_namespace( raw_namespace );
    a_true!( exec_namespace.commands.len() == 1 );
    a_id!( vec![ "subject".to_string() ], exec_namespace.commands[ 0 ].subjects );

    // parse namespace with only several command
    let raw_namespace = parser.namespace( ".command1 first_subj .command2 second_subj" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    // convert namespace
    let exec_namespace = converter.to_namespace( raw_namespace );
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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : ".also".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command1, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // parse namespace with only several command
    let raw_namespace = parser.namespace( ".command1 first_subj .invalid_command second_subj" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    // convert namespace
    // ? Or it must fail beacause of unknown command?
    let exec_namespace = converter.to_namespace( raw_namespace );
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
