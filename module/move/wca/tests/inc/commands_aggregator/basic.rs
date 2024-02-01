use super::*;

//

tests_impls!
{
  fn simple()
  {
    let ca = CommandsAggregator::former()
    .grammar( // list of commands -> Collect all to GrammarConverter
    [
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form(),
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .form(),
    ])
    .executor( // hashmap of routines -> ExecutorConverter
    [
      ( "command".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
      ( "command2".to_owned(), Routine::new( | _ | { println!( "Command2" ); Ok( () ) } ) ),
    ])
    .build();

    a_id!( (), ca.perform( ".command2 .help" ).unwrap() ); // raw string -> GrammarProgram -> ExecutableProgram -> execute

    a_id!( (), ca.perform( ".help command" ).unwrap() );
    a_id!( (), ca.perform( ".help command2" ).unwrap() );
    a_id!( (), ca.perform( ".help help" ).unwrap() );

    a_id!( (), ca.perform( ".help.command" ).unwrap() );
    a_id!( (), ca.perform( ".help.command2" ).unwrap() );
    a_id!( (), ca.perform( ".help.help" ).unwrap() );

    a_true!( ca.perform( ".help.help.help" ).is_err() );
  }

  fn with_only_general_help()
  {
    let ca = CommandsAggregator::former()
    .grammar( // list of commands -> Collect all to GrammarConverter
    [
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form(),
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .form(),
    ])
    .executor( // hashmap of routines -> ExecutorConverter
    [
      ( "command".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
      ( "command2".to_owned(), Routine::new( | _ | { println!( "Command2" ); Ok( () ) } ) ),
    ])
    .help_variants([ HelpVariants::General ])
    .build();

    a_id!( (), ca.perform( ".help" ).unwrap() ); // raw string -> GrammarProgram -> ExecutableProgram -> execute

    a_true!( ca.perform( ".help command" ).is_err() );

    a_true!( ca.perform( ".help.command" ).is_err() );
  }

  fn custom_converters()
  {
    let grammar = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command2" )
      .form()
    )
    .form();

    let executor = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let ca = CommandsAggregator::former()
    .grammar_converter( grammar )
    .executor_converter( executor )
    .build();

    a_id!( (), ca.perform( ".command" ).unwrap() );
  }

  fn custom_parser()
  {
    let parser = Parser::former()
    .command_prefix( '-' )
    .form();

    let ca = CommandsAggregator::former()
    .parser( parser )
    .grammar(
    [
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form(),
    ])
    .executor(
    [
      ( "command".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
    ])
    .build();

    a_id!( (), ca.perform( "-command" ).unwrap() );
  }

  fn dot_command()
  {
    let ca = CommandsAggregator::former()
    .grammar(
    [
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "cmd.first" )
      .form(),
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "cmd.second" )
      .form(),
    ])
    .executor(
    [
      ( "cmd.first".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
      ( "cmd.second".to_owned(), Routine::new( | _ | { println!( "Command2" ); Ok( () ) } ) ),
    ])
    .build();

    a_id!( (), ca.perform( "." ).unwrap() );
    a_id!( (), ca.perform( ".cmd." ).unwrap() );

    a_true!( ca.perform( ".c." ).is_err() );
  }

  fn error_types()
  {
    let ca = CommandsAggregator::former()
    .grammar(
    [
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form(),
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command_with_execution_error" )
      .form(),
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command_without_executor" )
      .form(),
    ])
    .executor(
    [
      ( "command".to_owned(), Routine::new( | _ | { println!( "Command" ); Ok( () ) } ) ),
      ( "command_with_execution_error".to_owned(), Routine::new( | _ | { println!( "Command" ); Err( err!("todo") ) } ) ),
    ])
    .build();

    a_true!( ca.perform( ".command" ).is_ok() );
    // Expect execution error
    a_true!
    ( 
      matches!
      (
        ca.perform( ".command_with_execution_error" ), 
        Err( Error::Execution( _ ) ) 
      ), 
      "Unexpected error type, expected Error::Execution."
    );
    // Expect ValidationError::GrammarConverter
    a_true!
    (
      matches!
      (
        ca.perform( ".help.help.help" ), 
        Err( Error::Validation( ValidationError::GrammarConverter( _ ) ) ) 
      ), 
      "Unexpected validation error type, expected ValidationError::GrammarConverter."
    );
    // Expect ValidationError::Parser
    a_true!
    (
      matches!
      (
        ca.perform( "command" ), 
        Err( Error::Validation( ValidationError::Parser { .. } ) )
      ), 
      "Unexpected validation error type, expected ValidationError::Parser."
    );
    // Expect ValidationError::ExecutorConverter
    a_true!
    (
      matches!
      (
        ca.perform( ".command_without_executor" ), 
        Err( Error::Validation( ValidationError::ExecutorConverter( _ ) ) ) 
      ), 
      "Unexpected validation error type, expected ValidationError::ExecutorConverter."
    );
  }

  // tests bug fix when passing a subject with a colon character
  // example: passing the path to a directory with a colon in its name
  fn path_subject_with_colon() 
  {
    let grammar = GrammarConverter::former()
    .command
    (
      TheModule::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "A path to directory.", TheModule::Type::Path, true )
      .form()
    )
    .form();

    let executor = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let ca = CommandsAggregator::former()
    .grammar_converter( grammar )
    .executor_converter( executor )
    .build();

    let command = r#".command "./path:to_dir" "#;

    a_id!( (), ca.perform( command ).unwrap() );

    let wrong_command = r#".command ./path:to_dir "#;

    a_true!
    (
      matches!
      (
        ca.perform( wrong_command ), 
        Err( Error::Validation( ValidationError::Parser { .. } ) ) 
      ), 
      "It is a sentence that can not be parsed: `/path:to_dir`"
    );
  }

  fn string_subject_with_colon() 
  {
    let grammar = GrammarConverter::former()
    .command
    (
      TheModule::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "Any string.", TheModule::Type::String, true )
      .property( "nightly", "Some property.", TheModule::Type::String, true )
      .form()
    )
    .form();

    let executor = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let ca = CommandsAggregator::former()
    .grammar_converter( grammar.clone() )
    .executor_converter( executor )
    .build();

    let command = r#".command qwe:rty nightly:true "#;

    let parser = Parser::former().form();

    use TheModule::CommandParser;
    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( raw_command ).unwrap();

    a_id!( (), ca.perform( command ).unwrap() );

    a_id!( grammar_command.subjects, vec![ TheModule::Value::String("qwe:rty".into()) ] );
  }

  fn no_prop_subject_with_colon() 
  {
    let grammar = GrammarConverter::former()
    .command
    (
      TheModule::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "Any string.", TheModule::Type::String, true )
      .form()
    )
    .form();

    let executor = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let ca = CommandsAggregator::former()
    .grammar_converter( grammar.clone() )
    .executor_converter( executor )
    .build();

    let command = r#".command qwe:rty"#;

    let parser = Parser::former().form();

    use TheModule::CommandParser;
    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( raw_command ).unwrap();

    a_id!( (), ca.perform( command ).unwrap() );

    a_id!( grammar_command.subjects, vec![ TheModule::Value::String("qwe:rty".into()) ] );
  }

  // qqq: subject should be parsed if optional property is not specified
  fn optional_prop_subject_with_colon() 
  {
    let grammar = GrammarConverter::former()
    .command
    (
      TheModule::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "Any string.", TheModule::Type::String, true )
      .property( "nightly", "Some property.", TheModule::Type::String, true )
      .form()
    )
    .form();

    let executor = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let ca = CommandsAggregator::former()
    .grammar_converter( grammar.clone() )
    .executor_converter( executor )
    .build();

    let command = r#".command qwe:rty"#;

    let parser = Parser::former().form();

    use TheModule::CommandParser;
    let raw_command = parser.command( command ).unwrap();
    let grammar_command = grammar.to_command( raw_command ).unwrap();

    a_id!( (), ca.perform( command ).unwrap() );

    a_id!( grammar_command.subjects, vec![ TheModule::Value::String("qwe:rty".into()) ] );
  }
}

//

tests_index!
{
  simple,
  with_only_general_help,
  custom_converters,
  custom_parser,
  dot_command,
  error_types,
  path_subject_with_colon,
  string_subject_with_colon,
  no_prop_subject_with_colon,
  optional_prop_subject_with_colon,
}
