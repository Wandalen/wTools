//!
//! A trivial example.
//!

use wca::{ CommandsAggregator, Order, Type, VerifiedCommand };

fn f1( o : VerifiedCommand )
{
  println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props );
}

fn exit()
{
  println!( "just exit" );

  std::process::exit( 0 )
}

fn main() -> error_tools::error::untyped::Result< () >
{
  let ca = CommandsAggregator::former()
  .command( "exit" )
    .hint( "just exit" )
    .routine( || exit() )
    .end()
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
    .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
    .routine( f1 )
    .end()
  .order( Order::Lexicography )
  .perform()
  ;

  // aaa : aaa2 : for Bohdan : that should work
  // let ca = wca::CommandsAggregator::former()
  // .command( "echo" )
  //   .hint( "prints all subjects and properties" )
  //   .subject( "Subject", wca::Type::String, true )
  //   .property( "property", "simple property", wca::Type::String, true )
  //   .routine( f1 )
  //   .end()
  // .command( "exit" )
  //   .hint( "just exit" )
  //   .routine( || exit() )
  //   .end()
  // .perform()
  // ;
  // ca.execute( input ).unwrap();
  //aaa: works

  let input: Vec< String > = std::env::args().skip( 1 ).collect();
  ca.perform( input )?;
  
  Ok( () )
}
