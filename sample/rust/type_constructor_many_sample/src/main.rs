use type_constructor::prelude::*;

fn main()
{
  types!( many MyMany : i32 );
  let x = MyMany::from( [ 1, 2, 3 ] );
  println!( "x : {:?}", x.0 );
}
