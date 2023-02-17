use super::*;

//

fn ok_command_parser( parser : &Parser, command : &str ) -> RawCommand
{
  let raw_command = parser.command( command );
  a_true!( raw_command.is_ok() );
  raw_command.unwrap()
}

fn ok_command_grammar( grammar : &GrammarConverter, raw : RawCommand ) -> GrammarCommand
{
  let grammar_command = grammar.to_command( raw );
  a_true!( grammar_command.is_some() );
  grammar_command.unwrap()
}

fn ok_command_exec( exec : &ExecutorConverter, grammar : GrammarCommand ) -> ExecutableCommand
{
  let exec_command = exec.to_command( grammar );
  a_true!( exec_command.is_some() );
  exec_command.unwrap()
}

tests_impls!
{
  fn basic()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine( "command", Routine::new( | _ | { println!( "hello" ); Ok( () ) } ) )
    .form();

    let raw_command = ok_command_parser( &parser, ".command" );
    let grammar_command = ok_command_grammar( &grammar_converter, raw_command );
    let exec_command = ok_command_exec( &executor_converter, grammar_command );

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );
  }

  fn with_subject()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .subject_hint( "hint" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "command",
      Routine::new( |( args, _ )| args.get( 0 ).map( | a | println!( "{a}" )).ok_or_else( || err!( "Subject not found" ) ) )
    )
    .form();

    // with subject
    let raw_command = ok_command_parser( &parser, ".command subject" );
    let grammar_command = ok_command_grammar( &grammar_converter, raw_command );
    let exec_command = ok_command_exec( &executor_converter, grammar_command );

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // without subject
    let raw_command = ok_command_parser( &parser, ".command" );
    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_none() );
  }

  fn with_property()
  {
    let command = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "command" )
    .property_hint( "prop", "about prop" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( command )
    .form();

    // init executor
    let executor = Executor::former().form();
    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "command",
      Routine::new( |( _, props )| props.get( "prop" ).map( | a | println!( "{a}" )).ok_or_else( || err!( "Prop not found" ) ) )
    )
    .form();

    // with property
    let raw_command = ok_command_parser( &parser, ".command prop:value" );
    let grammar_command = ok_command_grammar( &grammar_converter, raw_command );
    let exec_command = ok_command_exec( &executor_converter, grammar_command );

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // with subject and without property
    let raw_command = ok_command_parser( &parser, ".command subject" );
    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_none() );

    // with subject and with property
    let raw_command = ok_command_parser( &parser, ".command subject prop:value" );
    let grammar_command = grammar_converter.to_command( raw_command );
    a_true!( grammar_command.is_none() );
  }

  fn with_context()
  {
    let check = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "check" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( check )
    .form();

    let mut ctx = wca::Context::default();
    ctx.insert( 1 );
    // init executor
    let executor = Executor::former()
    .kind( ExecutorType::Simple )
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

    let raw_command = ok_command_parser( &parser, ".check" );
    let grammar_command = ok_command_grammar( &grammar_converter, raw_command );
    let exec_command = ok_command_exec( &executor_converter, grammar_command );

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_subject,
  with_property,
  with_context,
}
