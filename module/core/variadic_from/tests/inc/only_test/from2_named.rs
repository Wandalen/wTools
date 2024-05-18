#[ test ]
fn from2_named()
{

  // - from2

  let got : Struct1 = from!( 13, 14 );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = Struct1::from2( 13, 14 );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = from!( ( 13, 14 ) );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  // - from1

  let got : Struct1 = Struct1::from1( ( 13, 14 ) );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = from!( ( ( 13, 14 ), ) );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = Struct1::from1( ( ( 13, 14 ), ) );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  // - to

  let got : Struct1 = ( 13, 14 ).to();
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = ( ( 13, 14 ), ).to();
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  // - std

  let got : Struct1 = From::from( ( 13, 14 ) );
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : Struct1 = ( 13, 14 ).into();
  let exp = Struct1{ a : 13, b : 14 };
  a_id!( got, exp );

}
