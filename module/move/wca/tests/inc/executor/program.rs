use super::*;
use the_module::
{
  parser::Parser,
  VerifiedCommand,
  executor::Context, Type,
  grammar::Dictionary,
  verifier::Verifier,

  Executor,
  // wtools
};

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
      wca::grammar::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "command" )
      .routine( || println!( "hello" ) )
      .form()
    )
    .form();
    let verifier = Verifier;

    // init executor
    let executor = Executor::former().form();

    // existed command | unknown command will fail on converter
    let raw_program = parser.parse( [ ".command" ] ).unwrap();
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();

    // execute the command
    a_true!( executor.program( dictionary, grammar_program ).is_ok() );
  }

  fn with_context()
  {
    use std::sync::{ Arc, Mutex };
    use error::untyped::Error;

    // init parser
    let parser = Parser;

    // init converter
    let dictionary = &Dictionary::former()
    .command
    (
      wca::grammar::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "inc" )
      .routine
      (
        | ctx : Context |
        ctx
        .get()
        .ok_or_else( || "Have no value" )
        .and_then( | x : Arc< Mutex< i32 > > | { *x.lock().unwrap() += 1; Ok( () ) } )
      )
      .form()
    )
    .command
    (
      wca::grammar::Command::former()
      .hint( "hint" )
      .long_hint( "long_hint" )
      .phrase( "eq" )
      .subject().hint( "number" ).kind( Type::Number ).optional( true ).end()
      .routine
      (
        | ctx : Context, o : VerifiedCommand |
        ctx
        .get()
        .ok_or_else( || "Have no value".to_string() )
        .and_then
        (
          | x : Arc< Mutex< i32 > > |
          {
            let x = x.lock().unwrap();
            let y : i32 = o.args.get( 0 ).ok_or_else( || "Missing subject".to_string() ).unwrap().to_owned().into();

            if dbg!( *x ) != y { Err( format!( "{} not eq {}", x, y ) ) } else { Ok( () ) }
          }
        )
      )
      .form()
    )
    .form();
    let verifier = Verifier;

    // starts with 0
    let ctx = wca::executor::Context::new( Mutex::new( 0 ) );
    // init simple executor
    let executor = Executor::former()
    .context( ctx )
    .form();

    // value in context = 0
    let raw_program = parser.parse( [ ".eq", "1" ] ).unwrap();
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();

    a_true!( executor.program( dictionary, grammar_program ).is_err() );

    // value in context = 1 + 1 + 1 = 3
    let raw_program = parser.parse( [ ".eq", "0", ".inc", ".inc", ".eq", "2" ] ).unwrap();
    let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();

    a_true!( executor.program( dictionary, grammar_program ).is_ok() );
  }
}

//

tests_index!
{
  basic,
  with_context,
}
