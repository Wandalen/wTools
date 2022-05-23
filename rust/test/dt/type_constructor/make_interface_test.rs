#[ allow( unused_imports ) ]
use super::TheModule;
use test_tools::*;
use TheModule::*;

tests_impls!
{

  #[ test ]
  fn max_test()
  {

    #[ derive( Debug, PartialEq ) ]
    struct Struct1
    {
      _0 : i32,
      _1 : i32,
      _2 : i32,
    }

    impl Make0 for Struct1
    {
      fn make_0() -> Self
      {
        Self { _0 : 0, _1 : 0, _2 : 0 }
      }
    }

    impl Make1< i32 > for Struct1
    {
      fn make_1( val : i32 ) -> Self
      {
        Self { _0 : val, _1 : val, _2 : val }
      }
    }

    impl Make2< i32, i32 > for Struct1
    {
      fn make_2( val0 : i32, val1 : i32 ) -> Self
      {
        Self { _0 : val0, _1 : val1, _2 : val1 }
      }
    }

    impl Make3< i32, i32, i32 > for Struct1
    {
      fn make_3( val0 : i32, val1 : i32, val2 : i32 ) -> Self
      {
        Self { _0 : val0, _1 : val1, _2 : val2 }
      }
    }

    let got : Struct1 = make!();
    let exp = Struct1{ _0 : 0, _1 : 0, _2 : 0 };
    assert_eq!( got, exp );

    let got : Struct1 = make!( 13 );
    let exp = Struct1{ _0 : 13, _1 : 13, _2 : 13 };
    assert_eq!( got, exp );

    let got : Struct1 = make!( 0, 1 );
    let exp = Struct1{ _0 : 0, _1 : 1, _2 : 1 };
    assert_eq!( got, exp );

    let got : Struct1 = make!( 0, 1, 2 );
    let exp = Struct1{ _0 : 0, _1 : 1, _2 : 2 };
    assert_eq!( got, exp );

  }

  //

  #[ test ]
  fn sample_test()
  {

    #[ derive( Debug, PartialEq ) ]
    struct Struct1
    {
      a : i32,
      b : i32,
    }

    impl Make0 for Struct1
    {
      fn make_0() -> Self
      {
        Self { a : 0, b : 0 }
      }
    }

    impl Make1< i32 > for Struct1
    {
      fn make_1( val : i32 ) -> Self
      {
        Self { a : val, b : val }
      }
    }

    impl Make2< i32, i32 > for Struct1
    {
      fn make_2( val1 : i32, val2 : i32 ) -> Self
      {
        Self { a : val1, b : val2 }
      }
    }

    let got : Struct1 = make!();
    let exp = Struct1{ a : 0, b : 0 };
    assert_eq!( got, exp );

    let got : Struct1 = make!( 13 );
    let exp = Struct1{ a : 13, b : 13 };
    assert_eq!( got, exp );

    let got : Struct1 = make!( 1, 3 );
    let exp = Struct1{ a : 1, b : 3 };
    assert_eq!( got, exp );

  }

}

//

tests_index!
{

  max_test,
  sample_test,

}
