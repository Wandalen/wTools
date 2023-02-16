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
      namespace_delimeter : ".also".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command, | _ | { println!( "hello" ); Ok( () ) } )
    .form();

    // existed command | unknown command will fails on converter
    let raw_program = parser.program( ".command" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    let exec_program = converter.to_program( raw_program );

    // init executor
    let executor = Executor::former().form();

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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : ".also".into(),
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
    // init simple executor
    let executor = Executor::former()
    .context( ctx )
    .kind( ExecutorType::Simple )
    .form();

    // value in context = 0
    let raw_program = parser.program( ".eq 1" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    let exec_program = converter.to_program( raw_program );

    a_true!( dbg!( executor.program( exec_program ) ).is_err() );

    // value in context = 0 + 1 = 1 | 1 + 1 + 1 = 3
    let raw_program = parser.program( ".inc .eq 1 .also .eq 1 .inc .inc .eq 3" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    let exec_program = converter.to_program( raw_program );

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
    let raw_program = parser.program( ".eq 1" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    let exec_program = converter.to_program( raw_program );

    a_true!( dbg!( executor.program( exec_program ) ).is_err() );

    // value in context = 0 + 1 = 1 | 0 + 1 + 1 = 2
    let raw_program = parser.program( ".inc .eq 1 .also .eq 0 .inc .inc .eq 2" );
    a_true!( raw_program.is_ok() );
    let raw_program = raw_program.unwrap();

    let exec_program = converter.to_program( raw_program );

    a_true!( executor.program( exec_program ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
