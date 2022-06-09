
fn main()
{
  use type_constructor::*;

  types!( single MySingle : i32 );
  let x = MySingle( 13 );
  println!( "x : {}", x.0 );
  /* print : x : 13 */
}
