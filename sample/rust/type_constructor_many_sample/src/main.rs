#[ allow( unused_imports ) ]
use type_constructor::prelude::*;

fn main()
{
  #[ cfg( all( feature = "many", feature = "use_std" ) ) ]
  {
    types!( many MyMany : i32 );
    let x = MyMany::from( [ 1, 2, 3 ] );
    println!( "x : {:?}", x.0 );
  }
}
