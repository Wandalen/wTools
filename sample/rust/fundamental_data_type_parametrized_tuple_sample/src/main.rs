use fundamental_data_type::*;

fn main()
{

  single!( MySingle : i32 );
  let x = MySingle( 13 );
  println!( "x : {}", x.0 );

}

