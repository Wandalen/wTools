use super::*;
//

tests_impls!
{
  fn basic()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
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

    // existed command | unknown command will fails on converter
    let raw_program = parser.program( ".command" ).unwrap();
    let grammar_program = grammar_converter.to_program( raw_program ).unwrap();
    let exec_program = executor_converter.to_program( grammar_program ).unwrap();

    // execute the command
    a_true!( executor.program( exec_program ).is_ok() );
  }

  fn with_context()
  {
    // init parser
    let parser = Parser::former().form();

    // init converter
    let grammar_converter = GrammarConverter::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "inc" )
      .form()
    )
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "eq" )
      .subject( "number", Type::Number, true )
      .form()
    )
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
            let y : i32 = args.get( 0 ).ok_or_else( || err!( "" ) )?.to_owned().into();

            if dbg!( x ) != y { Err( err!( "{} not eq {}", x, y ) ) } else { Ok( () ) }
          }
        )
      )
    )
    .form();

    // value in context = 0
    let raw_program = parser.program( ".eq 1" ).unwrap();
    let grammar_program = grammar_converter.to_program( raw_program ).unwrap();
    let exec_program = executor_converter.to_program( grammar_program ).unwrap();

    a_true!( executor.program( exec_program ).is_err() );

    // value in context = 0 + 1 = 1 | 1 + 1 + 1 = 3
    let raw_program = parser.program( ".inc .eq 1 .also .eq 1 .inc .inc .eq 3" ).unwrap();
    let grammar_program = grammar_converter.to_program( raw_program ).unwrap();
    let exec_program = executor_converter.to_program( grammar_program ).unwrap();

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
    let raw_program = parser.program( ".eq 1" ).unwrap();
    let grammar_program = grammar_converter.to_program( raw_program ).unwrap();
    let exec_program = executor_converter.to_program( grammar_program ).unwrap();

    a_true!( executor.program( exec_program ).is_err() );

    // value in context = 0 + 1 = 1 | 0 + 1 + 1 = 2
    let raw_program = parser.program( ".inc .eq 1 .also .eq 0 .inc .inc .eq 2" ).unwrap();
    let grammar_program = grammar_converter.to_program( raw_program ).unwrap();
    let exec_program = executor_converter.to_program( grammar_program ).unwrap();

    a_true!( executor.program( exec_program ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
