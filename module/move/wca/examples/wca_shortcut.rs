use wca::exposed::*;

fn echo( () : (), args : Args, _ : Props ) -> Result< (), () >
{
  let mut args = args.0.into_iter();
  wca::parse_args!( args, value: String );

  println!( "{value}" );

  Ok( () )
}

fn main()
{
  let args = std::env::args().skip( 1 ).collect::< Vec< _ > >().join( " " );
  let aggregator = wca::cui( () ).command( echo.arg( "string", Type::String ) ).build();
  aggregator.perform( args ).unwrap();
}
