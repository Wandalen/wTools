#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn basic()
{
  use TheModule::AbsolutePath;

  let path1 = "/some/absolute/path";
  let got : AbsolutePath = path1.into();
  let exp = AbsolutePath::new( "/some/absolute/path" );
  a_id!( got, exp );

}
