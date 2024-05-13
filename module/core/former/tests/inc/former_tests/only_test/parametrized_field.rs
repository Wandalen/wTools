
#[ test ]
fn basic()
{
  let got = Child::< 'static, str >::former().name( "abc" ).arg( "arg1" ).end();
  let exp = Child::< 'static, str >{ name : "abc".into(), arg : "arg1" };
  a_id!( got, exp );
}
