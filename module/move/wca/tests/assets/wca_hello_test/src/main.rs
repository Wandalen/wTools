fn main()
{
  use wca::{ Type, VerifiedCommand };

  let ca = wca::CommandsAggregator::former()
  .command( "echo" )
    .hint( "prints all subjects and properties" )
    .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
    .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) } )
    .end()
  .perform();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args ).unwrap();
}