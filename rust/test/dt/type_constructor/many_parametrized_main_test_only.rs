tests_impls!
{

  //

  #[ test ]
  fn main()
  {

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : Many< f32, f64 > = TheModule::make!();
      let exp = Many::< f32, f64 >( std::vec::Vec::new() );
      a_id!( got, exp );

      /* test.case( "make1" ) */
      let got : Many< f32, f64 > = TheModule::make!( mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Many< f32, f64 > = TheModule::make!( mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make3" ) */
      let got : Many< f32, f64 > = TheModule::make!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32, f64 >( vec!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many< f32, f64 > = [ mk!( 13.0 ) ].into();
    let instance2 = Many::< f32, f64 >::from([ mk!( 13.0 ) ]);
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    /* test.case( "from &f32 into Many" ) */
    let instance1 : Many< f32, f64 > = ( &mk!( 13.0 ) ).into();
    let instance2 = Many::< f32, f64 >::from( &mk!( 13.0 ) );
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );
    // yyy

    /* test.case( "from itself into itself" ) */
    let instance1 : Many< f32, f64 > = ( Many::from([ mk!( 13.0 ) ]) ).into();
    let instance2 = Many::< f32, f64 >::from( Many::from([ mk!( 13.0 ) ]) );
    a_id!( instance1.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance2.0, vec![ mk!( 13.0 ) ] );
    a_id!( instance1, instance2 );

    // /* test.case( "from vector" ) */
    // let got : Many< f32, f64 > = vec![ mk!( 13.0 ), mk!( 31.0 ) ].into();
    // let exp : Many< f32, f64 > = Many::from( vec![ mk!( 13.0 ), mk!( 31.0 ) ] );
    // a_id!( got, exp );
    // xxx

    /* test.case( "from tuple" ) */
    let got : Many< f32, f64 > = ( mk!( 13.0 ), ).into();
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( ( mk!( 13.0 ), ) );
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );
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

    // // xxx
    // let slice = &[ mk!( 13.0 ), ][ .. ];
    // for e in slice
    // {
    //   inspect_type::inspect_type_of!( e )
    //   // dbg!( e );
    // }

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( &[ mk!( 13.0 ), ][ .. ] ).into();
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( &[ mk!( 13.0 ), ][ .. ] );
    let exp : Many< f32, f64 > = Many::from([ mk!( 13.0 ) ]);
    a_id!( got, exp );

    /* test.case( "from slice" ) */
    let got : Many< f32, f64 > = ( &[ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ][ .. ] ).into();
    let exp : Many< f32, f64 > = Many::from( [ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ] );
    a_id!( got, exp );
    let got = Many::< f32, f64 >::from( &[ mk!( 1.0 ), mk!( 2.0 ), mk!( 3.0 ) ][ .. ] );
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
