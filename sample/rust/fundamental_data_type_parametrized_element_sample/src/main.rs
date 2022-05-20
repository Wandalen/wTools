use fundamental_data_type::*;

fn main()
{

  single!
  {
    #[ derive( Debug ) ]
    MySingle : std::sync::Arc< T : Copy >;
  }
  let x = MySingle( std::sync::Arc::new( 13 ) );
  dbg!( x );

}
