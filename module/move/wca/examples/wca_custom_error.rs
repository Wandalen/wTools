//!
//! # Handling Errors with CommandsAggregator
//!
//! This module provides an example of how to use `wca::CommandsAggregator` to manage error handling in a command-line interface. The `CommandsAggregator` offers a fluent interface for defining commands and associating them with various error types, making it straightforward to handle and present errors in a structured way.
//!
//! ## Purpose
//!
//! The primary goal of this example is to showcase how `CommandsAggregator` facilitates error handling, whether errors are simple strings, custom typed errors, untyped errors, or errors with additional context. This approach ensures that error management is both consistent and extensible.
//!

#[ derive( Debug, error_tools::typed::Error )]
enum CustomError
{
  #[ error( "this is typed error" ) ]
  TheError,
}

fn main() -> error_tools::error::untyped::Result< () >
{
  let ca = wca::CommandsAggregator::former()
  .command( "error.string" )
    .hint( "Returns error as a string" )
    .routine( || { Err( "this is string error" ) } )
  .end()
  .command( "error.typed" )
    .hint( "Returns error as a custom error" )
    .routine( || { Err( CustomError::TheError ) } )
  .end()
  .command( "error.untyped" )
    .hint( "Returns error as untyped error" )
    .routine( || { Err( error_tools::error::untyped::format_err!( "this is untyped error" ) ) } )
  .end()
  .command( "error.with_context" )
    .hint( "Returns error as untyped error with context" )
    .routine( || { Err( error_tools::error::untyped::format_err!( "this is untyped error" ).context( "with context" ) ) } )
  .end()
  .perform();

  let args: Vec< String > = std::env::args().skip( 1 ).collect();
  () = ca.perform( args )?;
  
  Ok( () )
}