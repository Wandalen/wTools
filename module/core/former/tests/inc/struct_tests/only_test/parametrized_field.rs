#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]

#[ test ]
fn basic()
{
  let got = Child::< 'static, str >::former().name( "abc" ).arg( "arg1" ).end();
  let exp = Child::< 'static, str >{ name : "abc".into(), arg : "arg1" };
  a_id!( got, exp );
}
