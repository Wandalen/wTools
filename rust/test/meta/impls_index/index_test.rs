use test_tools::*;
use super::TheModule;
use TheModule::prelude::impls1;

//

tests_impls!
{

  #[ test ]
  fn empty_with_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!();
      index!();

    }

  }

  #[ test ]
  fn empty_without_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
      };

      index!
      {
      }

    }

  }

  #[ test ]
  fn with_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
        fn f1() -> i32
        {
          println!( "f1" );
          13
        }
      };

      index!
      {
        f1,
      }

      a_id!( f1(), 13 );
    }

  }

  #[ test ]
  fn without_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
        fn f1() -> i32
        {
          println!( "f1" );
          13
        }
      };

      index!
      {
        f1
      }

      a_id!( f1(), 13 );
    }

  }

  #[ test ]
  fn parentheses_with_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
        fn f1() -> i32
        {
          println!( "f1" );
          13
        }
      };

      index!( f1, );

      a_id!( f1(), 13 );
    }

  }

  #[ test ]
  fn parentheses_without_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
        fn f1() -> i32
        {
          println!( "f1" );
          13
        }
      };

      index!( f1 );

      a_id!( f1(), 13 );
    }

  }

}

//

tests_index!
{

  empty_with_comma,
  empty_without_comma,
  with_comma,
  without_comma,
  parentheses_with_comma,
  parentheses_without_comma,

}
