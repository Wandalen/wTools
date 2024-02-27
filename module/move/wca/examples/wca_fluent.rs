//!
//! # Fluent interface example
//!
//! This module introduces a fluent interface implemented via the `wca::CommandsAggregator`, which provides an intuitive method chaining mechanism for creating a command-line interface.
//!
//! The fluent interface and function chaining make it easy to add, update, or modify commands without breaking the application's flow. This design allows for extensibility while keeping the methods structured and clear, making it a good fit for complex CLI applications' needs.
//!


use wca::{ Args, Context };

fn main()
{

  let ca = wca::CommandsAggregator::former()
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject( "Subject", wca::Type::String, true )
    .property( "property", "simple property", wca::Type::String, true )
    .routine( | args : Args, props | { println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" ) } )
    .perform()
  .command( "inc" )
    .hint( "This command increments a state number each time it is called consecutively. (E.g. `.inc .inc`)" )
    .routine( | ctx : Context | { let i : &mut i32 = ctx.get_or_default(); println!( "i = {i}" ); *i += 1; } )
    .perform()
  .command( "error" )
    .hint( "prints all subjects and properties" )
    .subject( "Error message", wca::Type::String, true )
    .routine( | args : Args | { println!( "Returns an error" ); Err( format!( "{}", args.get_owned::< String >( 0 ).unwrap_or_default() ) ) } )
    .perform()
  .command( "exit" )
    .hint( "just exit" )
    .routine( || { println!( "exit" ); std::process::exit( 0 ) } )
    .perform()
  .perform();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args ).unwrap();

}
