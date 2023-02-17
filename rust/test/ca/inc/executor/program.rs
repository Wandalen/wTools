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

fn ok_program_exec( exec : &ExecutorConverter, grammar : Program< Namespace< GrammarCommand > > ) -> Program< Namespace< ExecutableCommand > >
{
  let exec_program = exec.to_program( grammar );
  // a_true!( exec_programs_some() );
  // exec_programnwrap()
  exec_program
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

    // existed command | unknown command will fails on converter
    let raw_program = ok_program_parser( &parser, ".command" );
    let grammar_program = ok_program_grammar( &grammar_converter, raw_program );
    let exec_program = ok_program_exec( &executor_converter, grammar_program );

    // execute the command
    a_true!( executor.program( exec_program ).is_ok() );
  }

  fn with_context()
  {
    let inc = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "inc" )
    .form();

    let check = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "eq" )
    .subject_hint( "number" )
    .form();

    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command( inc )
    .command( check )
    .form();

    // starts with 0
    let mut ctx = wca::Context::default();
    ctx.insert( 0 );
    // init simple executor
    let executor = Executor::former()
    .context( ctx )
    .kind( ExecutorType::Simple )
    .form();
    let executor_converter = ExecutorConverter::former()
    .routine
    (
      "inc",
      Routine::new_with_ctx
      (
        | _, ctx |
        ctx
        .get_mut()
        .ok_or_else( || err!( "Have no value" ) )
        .and_then( | x : &mut i32 | { *x += 1; Ok( () ) } )
      )
    )
    .routine
    (
      "eq",
      Routine::new_with_ctx
      (
        | ( args, _ ), ctx |
        ctx
        .get_ref()
        .ok_or_else( || err!( "Have no value" ) )
        .and_then
        (
          | &x : &i32 |
          {
            let y = args.get( 0 ).ok_or_else( || err!( "" ) )?;
            let y = y.parse::< i32 >().map_err( | _ | err!( "" ) )?;

            if dbg!( x ) != y { Err( err!( "{} not eq {}", x, y ) ) } else { Ok( () ) }
          }
        )
      )
    )
    .form();

    // value in context = 0
    let raw_program = ok_program_parser( &parser, ".eq 1" );
    let grammar_program = ok_program_grammar( &grammar_converter, raw_program );
    let exec_program = ok_program_exec( &executor_converter, grammar_program );

    a_true!( executor.program( exec_program ).is_err() );

    // value in context = 0 + 1 = 1 | 1 + 1 + 1 = 3
    let raw_program = ok_program_parser( &parser, ".inc .eq 1 .also .eq 1 .inc .inc .eq 3" );
    let grammar_program = ok_program_grammar( &grammar_converter, raw_program );
    let exec_program = ok_program_exec( &executor_converter, grammar_program );

    a_true!( executor.program( exec_program ).is_ok() );

    // starts with 0
    let mut ctx = wca::Context::default();
    ctx.insert( 0 );
    // init resetable executor
    let executor = Executor::former()
    .context( ctx )
    .kind( ExecutorType::ResetsContext )
    .form();

    // value in context = 0
    let raw_program = ok_program_parser( &parser, ".eq 1" );
    let grammar_program = ok_program_grammar( &grammar_converter, raw_program );
    let exec_program = ok_program_exec( &executor_converter, grammar_program );

    a_true!( executor.program( exec_program ).is_err() );

    // value in context = 0 + 1 = 1 | 0 + 1 + 1 = 2
    let raw_program = ok_program_parser( &parser, ".inc .eq 1 .also .eq 0 .inc .inc .eq 2" );
    let grammar_program = ok_program_grammar( &grammar_converter, raw_program );
    let exec_program = ok_program_exec( &executor_converter, grammar_program );

    a_true!( executor.program( exec_program ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
