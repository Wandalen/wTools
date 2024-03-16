{

  // #[ derive( Debug, PartialEq ) ]
  // struct StructNamedFields
  // {
  //   a : i32,
  //   b : i32,
  //   c : i32,
  //   d : i32,
  // }

  let got : StructNamedFields = the_module::from!();
  let exp = StructNamedFields{ a : 0, b : 0, c : 0, d : 0 };
  a_id!( got, exp );

  let got : StructNamedFields = the_module::from!( 13 );
  let exp = StructNamedFields{ a : 13, b : 13, c : 13, d : 13 };
  a_id!( got, exp );

//   let got : StructNamedFields = the_module::from!( 0, 1 );
//   let exp = StructNamedFields{ a : 0, b : 1, c : 1, d : 1 };
//   a_id!( got, exp );
//
//   let got : StructNamedFields = the_module::from!( 0, 1, 2 );
//   let exp = StructNamedFields{ a : 0, b : 1, c : 2, d : 2 };
//   a_id!( got, exp );

  // let got : StructNamedFields = the_module::from!( 0, 1, 2, 3 );
  // let exp = StructNamedFields{ a : 0, b : 1, c : 2, d : 3 };
  // a_id!( got, exp );

}