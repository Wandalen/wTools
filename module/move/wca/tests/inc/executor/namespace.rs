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
    let raw_namespace = parser.namespace( ".command" ).unwrap();
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace ).unwrap();
    let exec_namespace = executor_converter.to_namespace( grammar_namespace ).unwrap();

    // execute the command
    a_true!( executor.namespace( exec_namespace ).is_ok() );
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

    // init executor
    let executor = Executor::former()
    .context( ctx )
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
            let y : i32 = args.get( 0 ).ok_or_else( || err!( "" ) ).unwrap().to_owned().into();

            if dbg!( x ) != y { Err( err!( "{} not eq {}", x, y ) ) } else { Ok( () ) }
          }
        )
      )
    )
    .form();

    // value in context = 0
    let raw_namespace = parser.namespace( ".eq 1" ).unwrap();
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace ).unwrap();
    let exec_namespace = executor_converter.to_namespace( grammar_namespace ).unwrap();

    a_true!( executor.namespace( exec_namespace ).is_err() );

    // value in context = 0 + 1 = 1
    let raw_namespace = parser.namespace( ".inc .eq 1" ).unwrap();
    let grammar_namespace = grammar_converter.to_namespace( raw_namespace ).unwrap();
    let exec_namespace = executor_converter.to_namespace( grammar_namespace ).unwrap();

    a_true!( executor.namespace( exec_namespace ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
