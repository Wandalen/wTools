use test_tools::*;
use super::TheModule as TheModule;
use TheModule::prelude::impls3;

//

fn impls_basic_test()
{

  // test.case( "impls3 basic" );
  {

    impls3!
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

  // test.case( "impls3 as" );
  {

    impls3!
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
    f1!( as f1b );
    f2!( as f2b );
    // trace_macros!( false );

    f1b();
    f2b();

  }

  // test.case( "impls3 as index" );
  {

    impls3!
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
    index!
    {
      f1,
      f2 as f2b,
    }
    // trace_macros!( false );

    f1();
    f2b();

  }

//   // test.case( "macro" );
//   {
//
//     impls3!
//     {
//       fn f1()
//       {
//         macro_rules! macro1
//         {
//           ( $( $Arg : tt )* ) => { };
//         }
//         macro1!();
//       }
//     }
//
//     // trace_macros!( true );
//     f1!();
//     // trace_macros!( false );
//
//   }

}

// macro_rules! closure
// {
//   () =>
//   {
//     macro_rules! macro1
//     {
//       ( $( $Arg : tt )* ) => { };
//     }
//   }
// }
//
// closure!();

//

test_suite!
{
  impls_basic,
}
