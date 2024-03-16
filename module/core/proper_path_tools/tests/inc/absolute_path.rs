#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn basic()
{
  use the_module::AbsolutePath;

  let path1 = "/some/absolute/path";
  let got : AbsolutePath = path1.try_into().unwrap();
  println!( "got : {}", &got );
  println!( "path1 : {}", &path1 );
  a_id!( &got.to_string(), path1 );

}
