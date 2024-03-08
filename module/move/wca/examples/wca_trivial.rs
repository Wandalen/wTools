//!
//! A trivial example.
//!

fn main()
{

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
  .perform();

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

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args.join( " " ) ).unwrap();
}
