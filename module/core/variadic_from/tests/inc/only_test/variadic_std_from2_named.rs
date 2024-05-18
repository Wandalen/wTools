#[ test ]
fn variadic_std_from2_named()
{

  //

  let got : StructNamedFields = From::from( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).into();
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  //

}
