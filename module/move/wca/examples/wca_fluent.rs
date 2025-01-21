//!
//! # Fluent interface example
//!
//! This module introduces a fluent interface implemented via the `wca::CommandsAggregator`, which provides an intuitive method chaining mechanism for creating a command-line interface.
//!
//! The fluent interface and function chaining make it easy to add, update, or modify commands without breaking the application's flow. This design allows for extensibility while keeping the methods structured and clear, making it a good fit for complex CLI applications' needs.
//!


use wca::{ executor::{ Context, Handler }, Type, VerifiedCommand };
use std::sync::{ Arc, Mutex };

fn main() -> error_tools::error::untyped::Result< () >
{

  let ca = wca::CommandsAggregator::former()
  .with_context( Mutex::new( 0 ) )
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject().kind( Type::String ).optional( true ).end()
    .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) } )
    .end()
  .command( "inc" )
    .hint( "This command increments a state number each time it is called consecutively. (E.g. `.inc .inc`)" )
    .routine( | ctx : Context |
    {
      let i : Arc< Mutex< i32 > > = ctx.get().unwrap();
      let mut i = i.lock().unwrap();
      println!( "i = {}", i );
      *i += 1;
    } )
    .end()
  .command( "error" )
    .hint( "prints all subjects and properties" )
    .subject().kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!( "Returns an error" ); Err( format!( "{}", o.args.get_owned::< String >( 0 ).unwrap_or_default() ) ) } )
    .end()
  .command( "exit" )
    .hint( "just exit" )
    .routine( Handler::< _, std::convert::Infallible >::from
    (
      || { println!( "exit" ); std::process::exit( 0 ) }
    ) )
    .end()
  .perform();

  let args: Vec< String > = std::env::args().skip( 1 ).collect();
  ca.perform( args )?;

  Ok( () )
}
