// use test_tools::exposed::*;
use super::*;
use the_module::prelude::impls1;

//

tests_impls!
{

  fn impls_basic()
  {

    // test.case( "impls1 basic" );
    {

      impls1!
      {
        fn f1()
        {
          println!( "f1" );
        }
        pub fn f2()
        {
          println!( "f2" );
        }
      };

      // trace_macros!( true );
      f1!();
      f2!();
      // trace_macros!( false );

      f1();
      f2();

    }

  //   // test.case( "impls1 as" );
  //   {
  //
  //     impls1!
  //     {
  //       fn f1()
  //       {
  //         println!( "f1" );
  //       }
  //       pub fn f2()
  //       {
  //         println!( "f2" );
  //       }
  //     };
  //
  //     // trace_macros!( true );
  //     f1!( as f1b );
  //     f2!( as f2b );
  //     // trace_macros!( false );
  //
  //     f1b();
  //     f2b();
  //
  //   }
  //
  //   // test.case( "impls1 as index" );
  //   {
  //
  //     impls1!
  //     {
  //       fn f1()
  //       {
  //         println!( "f1" );
  //       }
  //       pub fn f2()
  //       {
  //         println!( "f2" );
  //       }
  //     };
  //
  //     // trace_macros!( true );
  //     index!
  //     {
  //       f1,
  //       f2 as f2b,
  //     }
  //     // trace_macros!( false );
  //
  //     f1();
  //     f2b();
  //
  //   }

    // test.case( "macro" );
    {

      impls1!
      {
        fn f1()
        {
          macro_rules! macro1
          {
            ( $( $Arg : tt )* ) => { };
          }
          macro1!();
        }
      }

      // trace_macros!( true );
      f1!();
      // trace_macros!( false );

    }

  }
}

//

tests_index!
{
  impls_basic,
}
