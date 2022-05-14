use test_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::meta as TheModule;
#[cfg( not( feature = "in_wtools" ) )]
use meta_tools as TheModule;
use TheModule::prelude::*;

//

fn fn_name_test()
{

  let f1 = 13;

  let f2 = fn_name!
  {
    fn f1()
    {
    }
  };

  dbg!( f2 );
  assert_eq!( f2, 13 );

}

//

fn fns_test()
{

//   // test.case( "several, trivial syntax" );
//   {
//     let mut counter = 0;
//
//     macro_rules! count
//     {
//       ( $( $Tts : tt )* ) =>
//       {
//         dbg!( stringify!( $( $Tts )* ) );
//         counter += 1;
//         $( $Tts )*
//       };
//     }
//
//     fns2!
//     {
//       @Callback { count }
//       @Fns
//       {
//         fn f1()
//         {
//           println!( "f1" );
//         }
//         fn f2()
//         {
//           println!( "f2" );
//         }
//       }
//     };
//
//     assert_eq!( counter, 2 );
//     f1();
//     f2();
//   }

  // test.case( "several, trivial syntax" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1()
        {
          println!( "f1" );
        }
        fn f2()
        {
          println!( "f2" );
        }
      }
    };

    assert_eq!( counter, 2 );
    f1();
    f2();
  }

  // test.case( "several, complex syntax" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1( src : i32 ) -> i32
        {
          println!( "f1" );
          src
        }
        fn f2( src : i32 ) -> i32
        {
          println!( "f2" );
          src
        }
      }
    };

    assert_eq!( counter, 2 );
    f1( 1 );
    f2( 2 );
  }

  // test.case( "several, parametrized syntax" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1< T : Copy >( src : T ) -> T
        {
          println!( "f1" );
          src
        }
      }
    };

    assert_eq!( counter, 1 );
    f1( 1 );
  }


  // test.case( "several, visibility" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        pub fn f1( src : i32 ) -> i32
        {
          println!( "f1" );
          src
        }
      }
    };

    assert_eq!( counter, 1 );
    f1( 1 );
  }

  // test.case( "several, where with comma" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1< T, >( src : T ) -> T
        where
          T : Copy,
        {
          println!( "f1" );
          src
        }
      }
    };

    assert_eq!( counter, 1 );
    f1( 1 );
  }

  // test.case( "several, where without comma" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1< T >( src : T ) -> T
        where
          T : Copy
        {
          println!( "f1" );
          src
        }
      }
    };

    assert_eq!( counter, 1 );
    f1( 1 );
  }

//   // test.case( "several, complex parameter" );
//   {
//     let mut counter = 0;
//
//     macro_rules! count
//     {
//       ( $( $Tts : tt )* ) =>
//       {
//         dbg!( stringify!( $( $Tts )* ) );
//         counter += 1;
//       };
//     }
//
//     fns!
//     {
//       @Callback { count }
//       @Fns
//       {
//         fn f1< T >( src : T ) -> T
//         where
//           T : < Self as From< X > >::Type
//         {
//           println!( "f1" );
//           src
//         }
//       }
//     };
//
//     assert_eq!( counter, 1 );
//   }

  // test.case( "several, complex syntax" );
  {
    let mut counter = 0;

    macro_rules! count
    {
      ( $( $Tts : tt )* ) =>
      {
        dbg!( stringify!( $( $Tts )* ) );
        counter += 1;
        $( $Tts )*
      };
    }

    // trace_macros!( true );
    fns!
    {
      @Callback { count }
      @Fns
      {
        fn f1< T >( src : T ) -> T
        where
          T : Copy,
        {
          println!( "f1" );
          src
        }
        fn f2< T : Copy >( src : T ) -> T
        {
          println!( "f2" );
          src
        }
      }
    };
    // trace_macros!( false );

    assert_eq!( counter, 2 );
    f1( 1 );
    f2( 2 );
  }

}

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

}

//

test_suite!
{
  fn_name,
  fns,
  impls_basic,
}
