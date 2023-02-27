fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    use wca::*;

    let ca = CommandsAggregator::former()
    .grammar(
    [
      Command::former()
      .phrase( "echo" )
      .subject( "Subject", Type::String )
      .property( "property", "simple property", Type::String )
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
    .with_help_command()
    .form();

    let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
    ca.perform( args.join( " " ) ).unwrap();
  }
}
