//!
//! Shortcut to minimize boilerplate.
//!

use wca::CommandExt;

/// Example of a command.
fn echo( () : (), args : wca::Args, _ : wca::Props ) -> Result< (), () >
{
  let mut args = args.0.into_iter();
  wca::parse_args!( args, value: String );

  println!( "{value}" );

  Ok( () )
}

/// Entry point.
fn main()
{
  let args = std::env::args().skip( 1 ).collect::< Vec< _ > >().join( " " );
  let aggregator = wca::cui( () )
  .command( echo.arg( "string", wca::Type::String ) )
  .build()
  ;
  aggregator.perform( args ).unwrap();
}
