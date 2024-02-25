//!
//! A trivial example.
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
    ( "echo".to_owned(), wca::Routine::new( |( args, props )|
    {
      println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
      Ok( () )
    })),
  ])
  .build();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args.join( " " ) ).unwrap();
}
