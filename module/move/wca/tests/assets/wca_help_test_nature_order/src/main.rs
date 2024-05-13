fn main()
{
  use wca::{ Type, VerifiedCommand };

  let ca = wca::CommandsAggregator::former()
  .command( "c" )
    .hint( "c" )
    .property( "c-property" ).kind( Type::String ).optional( true ).end()
    .property( "b-property" ).kind( Type::String ).optional( true ).end()
    .property( "a-property" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!("c") } )
    .end()
  .command( "b" )
    .hint( "b" )
    .property( "b-property" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!("b") } )
    .end()
  .command( "a" )
    .hint( "a" )
    .property( "a-property" ).kind( Type::String ).optional( true ).end()
    .routine( | o : VerifiedCommand | { println!("a") } )
    .end()
  .with_nature_sort( true )
  .perform();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  ca.perform( args ).unwrap();
}