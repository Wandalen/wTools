use super::*;
use wtools::err;

//

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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // existed command | unknown command will fails on converter
    let raw_namespace = parser.namespace( ".command" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    let exec_namespace = converter.to_namespace( raw_namespace );

    // init executor
    let executor = Executor::former().form();

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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command_with_ctx
    (
      inc, 
      | _, ctx |
      ctx
      .get_mut()
      .ok_or_else( || err!( "Have no value" ) )
      .and_then( | x : &mut i32 | { *x += 1; Ok( () ) } )
    )
    .command_with_ctx
    (
      check,
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
    .form();


    // starts with 0
    let mut ctx = wca::Context::default();
    ctx.insert( 0 );

    // init executor
    let executor = Executor::former()
    .context( ctx )
    .form();

    // value in context = 0
    let raw_namespace = parser.namespace( ".eq 1" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    let exec_namespace = converter.to_namespace( raw_namespace );

    a_true!( executor.namespace( exec_namespace ).is_err() );

    // value in context = 0 + 1 = 1
    let raw_namespace = parser.namespace( ".inc .eq 1" );
    a_true!( raw_namespace.is_ok() );
    let raw_namespace = raw_namespace.unwrap();

    let exec_namespace = converter.to_namespace( raw_namespace );

    a_true!( executor.namespace( exec_namespace ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
