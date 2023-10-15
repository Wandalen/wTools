fn main()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  {
    use wca::*;

    let ca = CommandsAggregator::former()
    .grammar(
    [
      Command::former()
      .phrase( "echo" )
      .hint( "prints all subjects and properties" )
      .subject( "Subject", Type::String, true )
      .property( "property", "simple property", Type::String, true )
      .form(),
    ])
    .executor(
    [
      ( "echo".to_owned(), Routine::new( |( args, props )|
      {
        println!( "= Args\n{args:?}\n\n= Properties\n{props:?}\n" );
        Ok( () )
      })),
    ])
    .build();

    let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
    ca.perform( args.join( " " ) ).unwrap();
  }
}
