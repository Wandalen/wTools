//!
//! A trivial example.
//!

use wca::{ CommandsAggregator, Args, Props, Type };

fn f1( args : Args, props : Props )
{
  println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
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
    .subject( "Subject", Type::String, true )
    .property( "property", "simple property", Type::String, true )
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
