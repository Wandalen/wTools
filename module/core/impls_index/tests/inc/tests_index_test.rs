// use test_tools::exposed::*;
use super::*;
use the_module::prelude::impls1;

//

tests_impls!
{


  fn empty_with_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!();
      tests_index!();

    }

  }


  fn empty_without_comma()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
      };

      tests_index!
      {
      }

    }

  }


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

      tests_index!
      {
        f1,
      }

      a_id!( f1(), 13 );
    }

  }


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

      tests_index!
      {
        f1
      }

      a_id!( f1(), 13 );
    }

  }


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

      tests_index!( f1, );

      a_id!( f1(), 13 );
    }

  }


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

      tests_index!( f1 );

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
