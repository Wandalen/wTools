
#[ test ]
fn simple()
{

  // Deref

  let got = IsTransparentSimple( true );
  let exp = true;
  a_id!( *got, exp );

}

#[ test ]
fn complex()
{

  // Deref

  let got_tmp = "start".to_string();
  let got = IsTransparentComplex::< '_, '_, String, str, 0 >( &got_tmp, core::marker::PhantomData );
  let exp_tmp = "start".to_string();
  let exp = &exp_tmp;
  assert_eq!( *got, exp );

}
