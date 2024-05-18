#[ test ]
fn from4_tuple()
{

  // #[ derive( Debug, PartialEq ) ]
  // struct Struct1( i32, i32, i32, i32 );

  let got : Struct1 = the_module::from!();
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( 13 );
  let exp = Struct1( 13, 13, 13, 13 );
  a_id!( got, exp );

  // - from unit

  let got : Struct1 = the_module::from!( () );
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : Struct1 = the_module::from!( ( (), ) );
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : Struct1 = ().to();
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : Struct1 = ( (), ).to();
  let exp = Struct1( 0, 0, 0, 0 );
  a_id!( got, exp );

  // - negative

//   let got : Struct1 = the_module::from!( 0, 1 );
//   let exp = Struct1( 0, 1, 1, 1 );
//   a_id!( got, exp );
//
//   let got : Struct1 = the_module::from!( 0, 1, 2 );
//   let exp = Struct1( 0, 1, 2, 2 );
//   a_id!( got, exp );
//
//   let got : Struct1 = the_module::from!( 0, 1, 2, 3 );
//   let exp = Struct1( 0, 1, 2, 3 );
//   a_id!( got, exp );

  // qqq : write negative test

}
