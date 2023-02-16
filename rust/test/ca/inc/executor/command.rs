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
    let raw_command = parser.command( ".command" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    // init executor
    let executor = Executor::former().form();

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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command, |( args, _ )| args.get( 0 ).map( | a | println!( "{a}" )).ok_or_else( || err!( "Subject not found" ) ) )
    .form();

    // with subject
    let raw_command = parser.command( ".command subject" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    // init executor
    let executor = Executor::former().form();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // without subject
    let raw_command = parser.command( ".command" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_none() );
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
    // TODO: Builder
    let parser = Parser
    {
      command_prefix : '.',
      prop_delimeter : ':',
      namespace_delimeter : "|".into(),
    };

    // init converter
    let converter = wca::Converter::former()
    .command( command, |( _, props )| props.get( "prop" ).map( | a | println!( "{a}" )).ok_or_else( || err!( "Prop not found" ) ) )
    .form();

    // with property
    let raw_command = parser.command( ".command prop:value" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    // init executor
    let executor = Executor::former().form();

    // execute the command
    a_true!( executor.command( exec_command ).is_ok() );

    // without subject
    let raw_command = parser.command( ".command" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    // init executor
    let executor = Executor::former().form();

    // execute the command
    a_true!( executor.command( exec_command ).is_err() );
  }

  fn with_context()
  {
    let check = wca::Command::former()
    .hint( "hint" )
    .long_hint( "long_hint" )
    .phrase( "check" )
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
      check,
      | _, ctx |
      ctx
      .get_ref()
      .ok_or_else( || err!( "Have no value" ) )
      .and_then( | &x : &i32 | if x != 1 { Err( err!( "x not eq 1" ) ) } else { Ok( () ) } )
    )
    .form();

    // increment value
    let raw_command = parser.command( ".check" );
    a_true!( raw_command.is_ok() );
    let raw_command = raw_command.unwrap();

    let exec_command = converter.to_command( raw_command );
    a_true!( exec_command.is_some() );
    let exec_command = exec_command.unwrap();

    let mut ctx = wca::Context::default();
    ctx.insert( 1 );
    // init executor
    let executor = Executor::former()
    .kind( ExecutorType::Simple )
    .context( ctx )
    .form();

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
