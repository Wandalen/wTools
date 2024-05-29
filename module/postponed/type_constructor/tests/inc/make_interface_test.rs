#[ allow( unused_imports ) ]
use super::*;
// use test_tools::exposed::*;
// use the_module::*;

tests_impls!
{

  fn max()
  {

    #[ derive( Debug, PartialEq, Make ) ]
    struct Struct1
    {
      _0 : i32,
      _1 : i32,
      _2 : i32,
      _3 : i32,
    }

    let got : Struct1 = the_module::from!();
    let exp = Struct1{ _0 : 0, _1 : 0, _2 : 0, _3 : 0 };
    a_id!( got, exp );

    let got : Struct1 = the_module::from!( 13 );
    let exp = Struct1{ _0 : 13, _1 : 13, _2 : 13, _3 : 13 };
    a_id!( got, exp );

//     let got : Struct1 = the_module::from!( 0, 1 );
//     let exp = Struct1{ _0 : 0, _1 : 1, _2 : 1, _3 : 1 };
//     a_id!( got, exp );
//
//     let got : Struct1 = the_module::from!( 0, 1, 2 );
//     let exp = Struct1{ _0 : 0, _1 : 1, _2 : 2, _3 : 2 };
//     a_id!( got, exp );
//
//     let got : Struct1 = the_module::from!( 0, 1, 2, 3 );
//     let exp = Struct1{ _0 : 0, _1 : 1, _2 : 2, _3 : 3 };
//     a_id!( got, exp );

  }

  //

  fn sample()
  {

    #[ derive( Debug, PartialEq, Make ) ]
    struct Struct1
    {
      a : i32,
      b : i32,
    }

    let got : Struct1 = the_module::from!();
    let exp = Struct1{ a : 0, b : 0 };
    a_id!( got, exp );

    let got : Struct1 = the_module::from!( 13 );
    let exp = Struct1{ a : 13, b : 13 };
    a_id!( got, exp );

    // let got : Struct1 = the_module::from!( 1, 3 );
    // let exp = Struct1{ a : 1, b : 3 };
    // a_id!( got, exp );

  }

  //

  fn slice_like()
  {

    #[ derive( Debug, PartialEq, Make ) ]
    struct Struct1( i32, i32, i32, i32 );

    let got : Struct1 = the_module::from!();
    let exp = Struct1( 0, 0, 0, 0 );
    a_id!( got, exp );

    let got : Struct1 = the_module::from!( 13 );
    let exp = Struct1( 13, 13, 13, 13 );
    a_id!( got, exp );

//     let got : Struct1 = the_module::from!( 0, 1 );
//     let exp = Struct1( 0, 1, 1, 1 );
//     a_id!( got, exp );
//
//     let got : Struct1 = the_module::from!( 0, 1, 2 );
//     let exp = Struct1( 0, 1, 2, 2 );
//     a_id!( got, exp );

    // qqq : write negative test
    // let got : Struct1 = the_module::from!( 0, 1, 2, 3 );
    // let exp = Struct1( 0, 1, 2, 3 );
    // a_id!( got, exp );

  }
}

//

tests_index!
{
  max,
  sample,
  slice_like,
}
