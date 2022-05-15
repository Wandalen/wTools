use test_tools::*;

// #[ allow( unused_imports ) ]
#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
// #[ allow( unused_imports ) ]
#[cfg( not( feature = "in_wtools" ) )]
use meta_tools as TheModule;
// #[ allow( unused_imports ) ]
// use TheModule::prelude::*;
use TheModule::prelude::impls1;

//

fn impls_basic_test()
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

//

test_suite!
{
  impls_basic,
}
