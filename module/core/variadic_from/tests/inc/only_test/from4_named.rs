#[ test ]
fn from4_named_fields()
{

  let got : Struct1 = the_module::from!();
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( 13 );
  let exp = Struct1{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

  // - from unit

  let got : Struct1 = the_module::from!( () );
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( (), ) );
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : Struct1 = ().to();
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : Struct1 = ( (), ).to();
  let exp = Struct1{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  // - negative

//   let got : Struct1 = the_module::from!( 0, 1 );
//   let exp = Struct1{ a : 0, b : 1, c : 1, d : 1 };
//   a_id!( got, exp );
//
//   let got : Struct1 = the_module::from!( 0, 1, 2 );
//   let exp = Struct1{ a : 0, b : 1, c : 2, d : 2 };
//   a_id!( got, exp );
//
//   let got : Struct1 = the_module::from!( 0, 1, 2, 3 );
//   let exp = Struct1{ a : 0, b : 1, c : 2, d : 3 };
//   a_id!( got, exp );

  // qqq : write negative test

}
