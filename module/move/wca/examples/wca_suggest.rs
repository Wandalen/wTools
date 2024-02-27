//! Using this feature, when calling a command with an invalid name, the error text will contain
//! a sentence with a correction, e.g. if you type:
//!
//! ```shell
//! cargo run --features on_unknown_suggest --example wca_suggest .echoooo
//! ```
//!
//! you will see the message:
//!
//! ```text
//! Validation error. Can not identify a command.
//! Details: Command not found. Maybe you mean `.echo`?
//! ```
//!
//! Otherwise
//!
//! ```text
//! Validation error. Can not identify a command.
//! Details: Command not found. Please use `.` command to see the list of available commands.
//! ```
//!

fn main()
{
  // use wca::prelude::*;

  let ca = wca::CommandsAggregator::former()
  .grammar
  ([
    wca::Command::former()
    .phrase( "echo" )
    .hint( "prints all subjects and properties" )
    .subject( "Subject", wca::Type::String, true )
    .property( "property", "simple property", wca::Type::String, true )
    .form(),
  ])
  .executor
  ([
   ( "echo".to_owned(), wca::Routine::new( | ( args, props ) |
    {
      println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
      Ok( () )
    })
  ),
  ])
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  match ca.perform( args.join( " " ) )
  {
    Ok( _ ) => {}
    Err( err ) => println!( "{err}" ),
  };

}
