use super::*;

//

fn ok_namespace_parser( parser : &Parser, namespace : &str ) -> Namespace< RawCommand >
{
  let raw_namespace = parser.namespace( namespace );
  a_true!( raw_namespace.is_ok() );
  raw_namespace.unwrap()
}

fn ok_namespace_grammar( grammar : &GrammarConverter, raw : Namespace< RawCommand > ) -> Namespace< GrammarCommand >
{
  let grammar_namespace = grammar.to_namespace( raw );
  // a_true!( grammar_namespace.is_some() );
  // grammar_namespace.unwrap()
  grammar_namespace
}

fn ok_namespace_exec( exec : &ExecutorConverter, grammar : Namespace< GrammarCommand > ) -> Namespace< ExecutableCommand >
{
  let exec_namespace = exec.to_namespace( grammar );
  // a_true!( exec_command.is_some() );
  // exec_command.unwrap()
  exec_namespace
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
    let raw_namespace = ok_namespace_parser( &parser, ".command" );
    let grammar_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    let exec_namespace = ok_namespace_exec( &executor_converter, grammar_namespace );

    // execute the command
    a_true!( executor.namespace( exec_namespace ).is_ok() );
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
            let y = args.get( 0 ).ok_or_else( || err!( "" ) )?;
            let y = y.parse::< i32 >().map_err( | _ | err!( "" ) )?;

            if dbg!( x ) != y { Err( err!( "{} not eq {}", x, y ) ) } else { Ok( () ) }
          }
        )
      )
    )
    .form();

    // value in context = 0
    let raw_namespace = ok_namespace_parser( &parser, ".eq 1" );
    let grammar_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    let exec_namespace = ok_namespace_exec( &executor_converter, grammar_namespace );

    a_true!( executor.namespace( exec_namespace ).is_err() );

    // value in context = 0 + 1 = 1
    let raw_namespace = ok_namespace_parser( &parser, ".inc .eq 1" );
    let grammar_namespace = ok_namespace_grammar( &grammar_converter, raw_namespace );
    let exec_namespace = ok_namespace_exec( &executor_converter, grammar_namespace );

    a_true!( executor.namespace( exec_namespace ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
