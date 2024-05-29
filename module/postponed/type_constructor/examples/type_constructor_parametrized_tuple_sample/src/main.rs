use type_constructor::*;

fn main()
{

  types!
  {
    #[ derive( Debug ) ]
    single MySingle : < T : Copy >;
  }
  let x = MySingle( 13 );
  dbg!( x );

}

