tests_impls!
{

  fn main()
  {
    use core::fmt;
    use the_module::
    {
      CloneAsTuple,
      CloneAsArray,
      AsTuple,
      AsArray,
      AsSlice,
    };

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make1" ) */
      let got : Single< f32, f64 > = the_module::from!( mk!( 13.0 ) );
      let exp = Single::< f32, f64 >::from( mk!( 13.0 ) );
      a_id!( got, exp );
    }

    /* test.case( "traits" ) */
    let instance1 = Single::< f32, f64 >::from( mk!( 13.0 ) );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    /* test.case( "from f32 into Single" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = Single::< f32, f64 >::from( mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from &f32 into Single" ) */
    let instance1 : Single< f32, f64 > = ( &mk!( 13.0 ) ).into();
    let instance2 = Single::< f32, f64 >::from( &mk!( 13.0 ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Single< f32, f64 > = ( Single::from( mk!( 13.0 ) ) ).into();
    let instance2 = Single::< f32, f64 >::from( Single::from( mk!( 13.0 ) ) );
    a_id!( instance1.0.0, 13.0 );
    a_id!( instance2.0.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Single into f32" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let got : mod1::Floats< f32, f64 > = instance1.into();
    a_id!( got.0, 13.0 );
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let got = mod1::Floats::< f32, f64 >::from( instance1 );
    a_id!( got.0, 13.0 );

    /* test.case( "from tuple" ) */
    let got : Single< f32, f64 > = ( mk!( 13.0 ), ).into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( ( mk!( 13.0 ), ) );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from array" ) */
    let got : Single< f32, f64 > = [ mk!( 13.0 ), ].into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( [ mk!( 13.0 ), ] );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "from slice" ) */
    let got : Single< f32, f64 > = ( &[ mk!( 13.0 ), ][ .. ] ).into();
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );
    let got = Single::< f32, f64 >::from( &[ mk!( 13.0 ), ][ .. ] );
    let exp : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    a_id!( got, exp );

    /* test.case( "clone / eq" ) */
    let instance1 : Single< f32, f64 > = ( mk!( 13.0 ) ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, mk!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let got : Single< f32, f64 > = ( mk!( 13.5 ) ).into();
    a_id!( got.round(), 14.0 );

    /* test.case( "clone_as_tuple" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.clone_as_tuple();
    a_id!( got, ( mk!( 13.0 ), ) );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "clone_as_array" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.clone_as_array();
    a_id!( got, [ mk!( 13.0 ), ] );
    assert!( !mem::same_ptr( &src, &got ) );

    /* test.case( "as_tuple" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_tuple();
    a_id!( got, &( mk!( 13.0 ), ) );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_array" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_array();
    a_id!( got, &[ mk!( 13.0 ), ] );
    assert!( mem::same_region( &src, got ) );

    /* test.case( "as_slice" ) */
    let src : Single< f32, f64 > = Single::from( mk!( 13.0 ) );
    let got = src.as_slice();
    a_id!( got, &[ mk!( 13.0 ), ][ .. ] );
    assert!( mem::same_region( &src, got ) );

  }

}

//

tests_index!
{

  main,

}
