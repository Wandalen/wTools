use type_constructor::*;

fn main()
{

  types!( single MySingle : i32 );
  let x = MySingle( 13 );
  println!( "x : {}", x.0 );

}
