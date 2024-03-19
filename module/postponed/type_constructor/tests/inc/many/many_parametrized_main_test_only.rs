// #[ derive( PartialEq, Debug ) ]
// struct MySingle
// (
//   pub f32,
// );
// impl From< MySingle >
// for f32
// {
//   fn from( src : MySingle ) -> Self
//   {
//     src.0
//   }
// }

tests_impls!
{
  fn main()
  {

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : Many< f32, f64 > = the_module::from!();
      let exp = Many::< f32, f64 >( std::vec::Vec::new() );
      a_id!( got, exp );

      /* test.case( "make1" ) */
      let got : Many< f32, f64 > = the_module::from!( mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Many< f32, f64 > = the_module::from!( mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make3" ) */
      let got : Many< f32, f64 > = the_module::from!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many< f32, f64 > = [ mk!( 13.0 ) ].into();
    let instance2 = Many::< f32, f64 >::from([ mk!( 13.0 ) ]);
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    // /* test.case( "from &f32 into Many" ) */
    // let instance1 : Many< f32, f64 > = ( &mk!( 13.0 ) ).into();
    // let instance2 = Many::< f32, f64 >::from( &mk!( 13.0 ) );
    // a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    // a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    // a_id!( instance1, instance2 );
    // yyy

    /* test.case( "from itself into itself" ) */
    let instance1 : Many< f32, f64 > = ( Many::from([ mk!( 13.0 ) ]) ).into();
    let instance2 = Many::< f32, f64 >::from( Many::from([ mk!( 13.0 ) ]) );
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    // /* test.case( "from tuple" ) */
    // let got : Many< f32, f64 > = ( mk!( 13.0 ), ).into();
    // let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    // a_id!( got, exp );
    // let got = Many::< f32, f64 >::from( ( mk!( 13.0 ), ) );
    // let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    // a_id!( got, exp );
    // yyy

    /* test.case( "from array" ) */
    let got : Many< f32, f64 > = [ mk!( 13.0 ), ].into();
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( [ mk!( 13.0 ), ] );
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );

    /* test.case( "from array" ) */
    let got : Many< f32, f64 > = [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ), ].into();
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );

    /* test.case( "from array of singles" ) */
    let got : Many< f32, f64 > = [ 1.0, 3.0 ].into();
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );
    let got = Many::< f32, f64 >::from( [ 1.0, 3.0 ] );
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );

    /* test.case( "from list" ) */
    let got : Many< f32, f64 > = vec![ mk!( 1.0 ), mk!( 3.0 ) ].into();
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );
    let got = Many::< f32, f64 >::from( vec![ mk!( 1.0 ), mk!( 3.0 ) ] );
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );

    /* test.case( "from list of singles" ) */
    let got : Many< f32, f64 > = vec![ 1.0, 3.0 ].into();
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );
    let got = Many::< f32, f64 >::from( vec![ 1.0, 3.0 ] );
    a_id!( got, Many( vec![ mk!( 1.0 ), mk!( 3.0 ) ] ) );

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( ( &[ mk!( 13.0 ), ][ .. ] ).iter().cloned() ).into();
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( ( &[ mk!( 13.0 ), ][ .. ] ).iter().cloned() );
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( &[ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ][ .. ] ).iter().cloned().into();
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( ( &[ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ][ .. ] ).iter().cloned() );
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32, f64 > = [ mk!( 13.0 ) ].into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32, f64 > = [ mk!( 13.0 ) ].into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( mk!( 13.0 ) ) );

    /* test.case( "as_slice" ) */
    let src : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );
    let got = &src[ .. ];
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );

  }

}

//

tests_index!
{
  main,
}
