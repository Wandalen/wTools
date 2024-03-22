{

  // #[ derive( Debug, PartialEq ) ]
  // struct StructTuple( i32, i32, i32, i32 );

  let got : StructTuple = the_module::from!();
  let exp = StructTuple( 0, 0, 0, 0 );
  a_id!( got, exp );

  let got : StructTuple = the_module::from!( 13 );
  let exp = StructTuple( 13, 13, 13, 13 );
  a_id!( got, exp );

//   let got : StructTuple = the_module::from!( 0, 1 );
//   let exp = StructTuple( 0, 1, 1, 1 );
//   a_id!( got, exp );
//
//   let got : StructTuple = the_module::from!( 0, 1, 2 );
//   let exp = StructTuple( 0, 1, 2, 2 );
//   a_id!( got, exp );

  // qqq : write negative test
  // let got : StructTuple = the_module::from!( 0, 1, 2, 3 );
  // let exp = StructTuple( 0, 1, 2, 3 );
  // a_id!( got, exp );

}
