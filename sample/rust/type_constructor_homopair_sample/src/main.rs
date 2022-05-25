use type_constructor::prelude::*;

fn main()
{
  use core::fmt;
  types!
  {
    #[ derive( Debug ) ]
    pair MyHomoPair : < T : fmt::Debug >;
  }
  let x = MyHomoPair( 13, 31 );
  dbg!( &x );
  // prints : &x = MyHomoPair( 13, 31 )
  let clone_as_array : [ i32 ; 2 ] = x.clone_as_array();
  dbg!( &clone_as_array );
  // prints : &clone_as_array = [ 13, 31 ]
  let clone_as_tuple : ( i32 , i32 ) = x.clone_as_tuple();
  dbg!( &clone_as_tuple );
  // prints : &clone_as_tuple = ( 13, 31 )
}
