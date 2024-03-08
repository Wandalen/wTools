use super::*;

//

tests_impls!
{
  fn basic()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let raw_command = parser.command( ".command" ).unwrap();
    let grammar_command = verifier.to_command( raw_command ).unwrap();
    let exec_command = executor_converter.to_command( grammar_command ).unwrap();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );
  }

  fn with_subject()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject( "hint", Type::String, false )
      .form()
    )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "command",
      Routine::new( |( args, _ )| args.get( 0 ).map( | a | println!( "{a:?}" )).ok_or_else( || err!( "Subject not found" ) ) )
    )
    .form();

    // with subject
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = verifier.to_command( raw_command ).unwrap();
    let exec_command = executor_converter.to_command( grammar_command ).unwrap();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // without subject
    let raw_command = parser.command( ".command" ).unwrap();
    let grammar_command = verifier.to_command( raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn with_property()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop", "about prop", Type::String, true )
      .form()
    )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "command",
      Routine::new( |( _, props )| props.get( "prop" ).map( | a | println!( "{a:?}" )).ok_or_else( || err!( "Prop not found" ) ) )
    )
    .form();

    // with property
    let raw_command = parser.command( ".command prop:value" ).unwrap();
    let grammar_command = verifier.to_command( raw_command ).unwrap();
    let exec_command = executor_converter.to_command( grammar_command ).unwrap();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // with subject and without property
    let raw_command = parser.command( ".command subject" ).unwrap();
    let grammar_command = verifier.to_command( raw_command );
    a_true!( grammar_command.is_err() );

    // with subject and with property
    let raw_command = parser.command( ".command subject prop:value" ).unwrap();
    let grammar_command = verifier.to_command( raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn with_context()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "check" )
      .form()
    )
    .form();

    let mut ctx = wca::Context::default();
    ctx.insert( 1 );
    // init executor
    let executor = Executor::former()
    .context( ctx )
    .form();

    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "check",
      Routine::new_with_ctx
      (
        | _, ctx |
        ctx
        .get_ref()
        .ok_or_else( || err!( "Have no value" ) )
        .and_then( | &x : &i32 | if x != 1 { Err( err!( "x not eq 1" ) ) } else { Ok( () ) } )
      )
    )
    .form();

    let raw_command = parser.command( ".check" ).unwrap();
    let grammar_command = verifier.to_command( raw_command ).unwrap();
    let exec_command = executor_converter.to_command( grammar_command ).unwrap();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );
  }

  fn without_routine()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let verifier = Verifier::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former().form();

    let raw_command = parser.command( ".command" ).unwrap();
    let grammar_command = verifier.to_command( raw_command ).unwrap();

    let exec_command = executor_converter.to_command( grammar_command );
    a_true!( exec_command.is_err() );
  }
}

//

tests_index!
{
  basic,
  with_subject,
  with_property,
  with_context,
  without_routine,
}
