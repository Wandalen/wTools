fn main()
{
  use wca::
  {
    CommandsAggregator, Command, Routine, Type,
  };

  let ca = wca::CommandsAggregator::former()
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject( "Subject", Type::String, true )
    .property( "property", "simple property", Type::String, true )
    .routine( | args : Args, props | { println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" ) } )
    .perform()
  .perform();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args ).unwrap();
}