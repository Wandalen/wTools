#[ derive( PartialEq, Debug ) ]
struct MySingle
(
  pub f32,
);
impl From< MySingle >
for f32
{
  fn from( src : MySingle ) -> Self
  {
    src.0
  }
}

tests_impls!
{
  fn main()
  {
    use core::fmt;

    #[ allow( unused_macros ) ]
    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        $( $Rest )*
      };
    }

    /* test.case( "basic" ) */
    let instance1 = Many::< f32 >::from( core::iter::once( 13.0_f32 ) );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( implements!( instance1 => Default ) );
    assert!( !implements!( instance1 => fmt::Display ) );

    #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
    {
      /* test.case( "make0" ) */
      let got : Many< f32 > = the_module::from!();
      let exp = Many::< f32 >( std::vec::Vec::new() );
      a_id!( got, exp );

      /* test.case( "make1" ) */
      let got : Many< f32 > = the_module::from!( mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make2" ) */
      let got : Many< f32 > = the_module::from!( mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );

      /* test.case( "make3" ) */
      let got : Many< f32 > = the_module::from!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) );
      let exp = Many::< f32 >( vec!( mk!( 1.0 ), mk!( 1.0 ), mk!( 1.0 ) ) );
      a_id!( got, exp );
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many< f32 > = ( core::iter::once( 13.0 ) ).into();
    let instance2 = Many::< f32 >::from( core::iter::once( 13.0 ) );
    a_id!( instance1.0, vec!( 13.0 ) );
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    // /* test.case( "from &f32 into Many" ) */
    // let instance1 : Many< f32 > = ( &13.0 ).into();
    // let instance2 = Many::< f32 >::from( &13.0 );
    // a_id!( instance1.0, vec!( 13.0 ) );
    // a_id!( instance2.0, vec!( 13.0 ) );
    // a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( core::iter::once( 13.0 ) ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( core::iter::once( 13.0 ) ) );
    a_id!( instance1.0, vec!( 13.0 ) );
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    // /* test.case( "from tuple" ) */
    // let got : Many< f32 > = ( 13.0, ).into();
    // a_id!( got, Many::from( core::iter::once( 13.0 ) ) );
    // let got = Many::< f32 >::from( ( 13.0, ) );
    // a_id!( got, Many::from( core::iter::once( 13.0 ) ) );

    /* test.case( "from array" ) */
    let got : Many< f32 > = [ 1.0, 3.0 ].into();
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );
    let got = Many::< f32 >::from( [ 1.0, 3.0 ] );
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );

    /* test.case( "from array of singles" ) */
    let got : Many< f32 > = [ MySingle( 1.0 ), MySingle( 3.0 ) ].into();
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );
    let got = Many::< f32 >::from( [ MySingle( 1.0 ), MySingle( 3.0 ) ] );
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );

    /* test.case( "from list" ) */
    let got : Many< f32 > = vec![ 1.0, 3.0 ].into();
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );
    let got = Many::< f32 >::from( vec![ 1.0, 3.0 ] );
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );

    /* test.case( "from list of singles" ) */
    let got : Many< f32 > = vec![ MySingle( 1.0 ), MySingle( 3.0 ) ].into();
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );
    let got = Many::< f32 >::from( vec![ MySingle( 1.0 ), MySingle( 3.0 ) ] );
    a_id!( got, Many( vec![ 1.0, 3.0 ] ) );

    /* test.case( "from slice" ) */
    let got : Many< f32 > = (&[ 13.0 ][ .. ]).iter().cloned().into();
    a_id!( got, Many::from( core::iter::once( 13.0 ) ) );
    let got = Many::< f32 >::from( (&[ 13.0 ][ .. ]).iter().cloned() );
    a_id!( got, Many::from( core::iter::once( 13.0 ) ) );

    /* test.case( "from slice" ) */
    let got : Many< f32 > = (&[ 1.0, 2.0, 3.0 ][ .. ]).iter().cloned().into();
    a_id!( got, Many::from( [ 1.0, 2.0, 3.0 ] ) );
    let got = Many::< f32 >::from( (&[ 1.0, 2.0, 3.0 ][ .. ]).iter().cloned() );
    a_id!( got, Many::from( [ 1.0, 2.0, 3.0 ] ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = core::iter::once( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec!( 13.0 ) );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = core::iter::once( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );
    a_id!( got.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "as_slice" ) */
    let src : Many< f32 > = core::iter::once( 13.0 ).into();
    let got = src.as_slice();
    a_id!( got, &[ 13.0, ][ .. ] );
    assert!( !mem::same_ptr( &src, got ) );
    let got = &src[ .. ];
    a_id!( got, &[ 13.0, ][ .. ] );
    assert!( !mem::same_ptr( &src, got ) );

  }
}

//

tests_index!
{
  main,
}
