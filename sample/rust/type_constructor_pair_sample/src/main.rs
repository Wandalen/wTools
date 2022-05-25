use type_constructor::prelude::*;

fn main()
{

  use core::fmt;
  types!
  {
    #[ derive( Debug ) ]
    pair MyPair : < T1 : fmt::Debug, T2 : fmt::Debug >;
  }
  let x = MyPair( 13, 13.0 );
  dbg!( x );
  // prints : x = MyPair( 13, 13.0 )

}
