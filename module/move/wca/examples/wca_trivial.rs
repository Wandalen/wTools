//!
//! A trivial example.
//!

use wca::{ CommandsAggregator, Type, VerifiedCommand };

fn f1( o : VerifiedCommand )
{
  println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props );
}

fn exit()
{
  println!( "just exit" );

  std::process::exit( 0 )
}

fn main()
{
  let ca = CommandsAggregator::former()
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
    .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
    .routine( f1 )
    .end()
  .command( "exit" )
    .hint( "just exit" )
    .routine( || exit() )
    .end()
  .perform()
  ;

  // aaa : qqq2 : for Bohdan : that should work
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

  let input = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( input ).unwrap();
}
