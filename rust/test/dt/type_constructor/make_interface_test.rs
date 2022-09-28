#[ allow( unused_imports ) ]
use super::*;
// use test_tools::exposed::*;
// use TheModule::*;

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
    }

    impl TheModule::Make2< i32, i32 > for Struct1
    {
      fn make_2( val0 : i32, val1 : i32 ) -> Self
      {
        Self { _0 : val0, _1 : val1, _2 : val1 }
      }
    }

    impl TheModule::Make3< i32, i32, i32 > for Struct1
    {
      fn make_3( val0 : i32, val1 : i32, val2 : i32 ) -> Self
      {
        Self { _0 : val0, _1 : val1, _2 : val2 }
      }
    }

    let got : Struct1 = TheModule::make!();
    let exp = Struct1{ _0 : 0, _1 : 0, _2 : 0 };
    a_id!( got, exp );

    let got : Struct1 = TheModule::make!( 13 );
    let exp = Struct1{ _0 : 13, _1 : 13, _2 : 13 };
    a_id!( got, exp );

    let got : Struct1 = TheModule::make!( 0, 1 );
    let exp = Struct1{ _0 : 0, _1 : 1, _2 : 1 };
    a_id!( got, exp );

    let got : Struct1 = TheModule::make!( 0, 1, 2 );
    let exp = Struct1{ _0 : 0, _1 : 1, _2 : 2 };
    a_id!( got, exp );
    let exp = Struct1{ _0 : 0, _1 : 1, _2 : 2 };
    a_id!( got, exp );

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

    impl TheModule::Make2< i32, i32 > for Struct1
    {
      fn make_2( val1 : i32, val2 : i32 ) -> Self
      {
        Self { a : val1, b : val2 }
      }
    }

    let got : Struct1 = TheModule::make!();
    let exp = Struct1{ a : 0, b : 0 };
    a_id!( got, exp );

    let got : Struct1 = TheModule::make!( 13 );
    let exp = Struct1{ a : 13, b : 13 };
    a_id!( got, exp );

    let got : Struct1 = TheModule::make!( 1, 3 );
    let exp = Struct1{ a : 1, b : 3 };
    a_id!( got, exp );

  }

}

//

tests_index!
{
  max,
  sample,
}
