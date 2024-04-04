use super::*;
use the_module::VerifiedCommand;

//

tests_impls!
{
  fn basic()
  {
    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .routine( || println!( "hello" ) )
      .form()
    )
    .form();
    let verifier = Verifier;

    // init executor
    let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();
    let executor = Executor::former().form();

    // execute the command
    a_true!( executor.command( dictionary, grammar_command ).is_ok() );
  }

  fn with_subject()
  {
    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .subject().hint( "hint" ).kind( Type::String ).optional( false ).end()
      .routine( | o : VerifiedCommand | o.args.get( 0 ).map( | a | println!( "{a:?}" )).ok_or_else( || "Subject not found" ) )
      .form()
    )
    .form();
    let verifier = Verifier;

    // init executor
    let executor = Executor::former().form();

    // with subject
    let raw_command = parser.parse( [ ".command", "subject" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // execute the command
    a_true!( executor.command( dictionary, grammar_command ).is_ok() );

    // without subject
    let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn with_property()
  {
    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .property( "prop" ).hint( "about prop" ).kind( Type::String ).optional( true ).end()
      .routine( | o : VerifiedCommand | o.props.get( "prop" ).map( | a | println!( "{a:?}" )).ok_or_else( || "Prop not found" ) )
      .form()
    )
    .form();
    let verifier = Verifier;

    // init executor
    let executor = Executor::former().form();

    // with property
    let raw_command = parser.parse( [ ".command", "prop:value" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // execute the command
    a_true!( executor.command( dictionary, grammar_command ).is_ok() );

    // with subject and without property
    let raw_command = parser.parse( [ ".command", "subject" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );

    // with subject and with property
    let raw_command = parser.parse( [ ".command", "subject", "prop:value" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command );
    a_true!( grammar_command.is_err() );
  }

  fn with_context()
  {
    use std::sync::{ Arc, Mutex };

    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "check" )
      .routine
      (
        | ctx : Context |
        ctx
        .get()
        .ok_or_else( || "Have no value" )
        .and_then( | x : Arc< Mutex< i32 > > | if *x.lock().unwrap() != 1 { Err( "x not eq 1" ) } else { Ok( () ) } )
      )
      .form()
    )
    .form();
    let verifier = Verifier;
    let mut ctx = wca::Context::new( Mutex::new( 1 ) );
    // init executor
    let executor = Executor::former()
    .context( ctx )
    .form();

    let raw_command = parser.parse( [ ".check" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    // execute the command
    a_true!( executor.command( dictionary, grammar_command ).is_ok() );
  }

  #[ should_panic( expected = "A handler function for the command is missing" ) ]
  fn without_routine()
  {
    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .form()
    )
    .form();
    let verifier = Verifier;

    // init executor
    let executor = Executor::former().form();

    let raw_command = parser.parse( [ ".command" ] ).unwrap().commands.remove( 0 );
    let grammar_command = verifier.to_command( dictionary, raw_command ).unwrap();

    a_true!( executor.command( dictionary, grammar_command ).is_err() );
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
