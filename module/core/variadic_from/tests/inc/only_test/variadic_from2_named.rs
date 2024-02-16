{

  //

  let got : StructNamedFields = from!( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_2( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = from!( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = StructNamedFields::from_1( ( ( 13, 14 ), ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).to();
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  let got : StructNamedFields = ( ( 13, 14 ), ).to();
  let exp = StructNamedFields{ a : 13, b : 14 };
  a_id!( got, exp );

  //

}