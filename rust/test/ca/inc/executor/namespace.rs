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
    .routine( | _ | { println!( "hello" ); Ok( () ) } )
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
    let converter = wca::Converter::from( vec![ command ] );

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
    .routine_with_ctx
    (
      | _, ctx |
      ctx
      .get_mut()
      .ok_or_else( || err!( "Have no value" ) )
      .and_then( | x : &mut i32 | { *x += 1; Ok( () ) } )
    )
    .form();

    let check = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "eq" )
    .subject_hint( "number" )
    .routine_with_ctx
    (
      | ( args, _ ), ctx |
      ctx
      .get_ref()
      .ok_or_else( || err!( "Have no value" ) )
      .and_then
      (
        | &x : &i32 |
        {
          let y = args.get( 0 ).ok_or_else( || err!( "Have no subject" ) )?;
          let y = y.parse::< i32 >().map_err( | _ | err!( "Failed to parse `{}`", y ) )?;

          if x != y { Err( err!( "expected {} eq {}", x, y ) ) } else { Ok( () ) }
        }
      )
    )
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
    let converter = wca::Converter::from( vec![ inc, check ] );

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
