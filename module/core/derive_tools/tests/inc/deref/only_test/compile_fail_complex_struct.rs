use test_tools ::a_id;

#[ test ]
fn deref_test()
{
  let got_tmp = "hello".to_string();
  let got = IsTransparentComplex :: < '_, '_, String, str, 0 >( &got_tmp, core ::marker ::PhantomData );
  let exp = &got_tmp;
  a_id!( *got, exp );
}