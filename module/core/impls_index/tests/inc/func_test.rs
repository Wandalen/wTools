#![ deny( unused_imports ) ]

use super::*;
#[ allow ( unused_imports ) ]
use the_module::exposed::*;
// use test_tools::exposed::*;

//

#[ test ]
fn fn_name()
{
  let f1 = 13;

  let f2 = fn_name!
  {
    fn f1()
    {
    }
  };

  dbg!( f2 );
  a_id!( f2, 13 );
}

//

#[ test ]
fn fn_rename()
{

  fn_rename!
  {
    @Name { f2 }
    @Fn
    {
      fn f1() -> i32
      {
        13
      }
    }
  };

  a_id!( f2(), 13 );

}

//

#[ test ]
fn fns()
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
//     a_id!( counter, 2 );
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

    a_id!( counter, 2 );
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

    a_id!( counter, 2 );
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

    a_id!( counter, 1 );
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

    a_id!( counter, 1 );
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

    a_id!( counter, 1 );
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

    a_id!( counter, 1 );
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
//     a_id!( counter, 1 );
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

    a_id!( counter, 2 );
    f1( 1 );
    f2( 2 );
  }

}
