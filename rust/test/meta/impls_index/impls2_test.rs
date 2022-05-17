use test_tools::*;

use TheModule::prelude::impls2;

//

fn impls_basic_test()
{

  // test.case( "impls2 basic" );
  {

    impls2!
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

  // test.case( "impls2 as" );
  {

    impls2!
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

  // test.case( "impls2 as index" );
  {

    impls2!
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

  // test.case( "macro" );
  {

    impls2!
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
  // fns,
  impls_basic,
}
