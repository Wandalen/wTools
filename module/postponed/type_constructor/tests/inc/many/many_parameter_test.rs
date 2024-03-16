#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn parameter_complex()
  {

    the_module::types!
    {
      #[ derive( Debug, Clone ) ]
      #[ derive( PartialEq ) ]
      many Many : < T : core::cmp::PartialEq + core::clone::Clone >;
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : Many< f32 > = core::iter::once( 13.0 ).into();
    let instance2 = Many::< f32 >::from( core::iter::once( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Many< f32 > = ( Many::from( core::iter::once( 13.0 ) ) ).into();
    let instance2 = Many::< f32 >::from( Many::from( core::iter::once( 13.0 ) ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : Many< f32 > = core::iter::once( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "deref" ) */
    let mut got : Many< f32 > = core::iter::once( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );
    a_id!( got.0, std::vec::Vec::< f32 >::new() );

  }

  //

  fn parameter_no_derives()
  {

    mod mod1
    {
      pub struct Floats< T1, T2 >
      (
        pub T1,
        pub T2,
      );
    }

    // trace_macros!( true );
    the_module::types!
    {
      many Many : < T >;
    }
    // trace_macros!( false );

    /* test.case( "smoke test" ) */
    let instance1 = Many( vec![ mod1::Floats( 13.0, 31.0 ) ] );

  }

  //

  fn struct_basic()
  {

    /* test.case( "from f32 into Many" ) */
    let instance1 : the_module::Many< f32 > = core::iter::once( 13.0 ).into();
    let instance2 = the_module::Many::< f32 >::from( core::iter::once( 13.0 ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : the_module::Many< f32 > = ( the_module::Many::from( core::iter::once( 13.0 ) ) ).into();
    let instance2 = the_module::Many::< f32 >::from( the_module::Many::from( core::iter::once( 13.0 ) ) );
    a_id!( instance1.0, vec![ 13.0 ] );
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "clone / eq" ) */
    let instance1 : the_module::Many< f32 > = core::iter::once( 13.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, vec![ 13.0 ] );
    a_id!( instance1, instance2 );

    /* test.case( "default" ) */
    let instance1 : the_module::Many< f32 > = Default::default();
    a_id!( instance1.0, std::vec::Vec::< f32 >::new() );

    /* test.case( "deref" ) */
    let mut got : the_module::Many< f32 > = core::iter::once( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

    /* test.case( "iterate" ) */
    // let mut got : the_module::Many< f32 > = [ 1.0, 2.0, 3.0 ].into();
    // a_id!( got.len(), 3 );
    // for e in got
    // {
    //   dbg!( e );
    // }
    // a_id!( got.len(), 3 );

    // zzz

  }

  //

  fn struct_no_derives()
  {

    macro_rules! mk
    {
      (
        $( $Rest : tt )*
      )
      =>
      {
        mod1::Floats( $( $Rest )* )
      };
    }

    mod mod1
    {
      pub struct Floats< T >( pub T );
      impl< T > Floats< T >
      {
        pub fn new( src : T ) -> Self
        { Self( src ) }
      }
    }

    /* test.case( "from f32 into Many" ) */
    let instance1 : the_module::Many< mod1::Floats< f32 > > = core::iter::once( mk!( 13.0 ) ).into();
    let instance2 = the_module::Many::< mod1::Floats< f32 > >::from( core::iter::once( mk!( 13.0 ) ) );
    a_id!( instance1.0[ 0 ].0, 13.0 );
    a_id!( instance1.len(), 1 );
    a_id!( instance2.0[ 0 ].0, 13.0 );
    a_id!( instance2.len(), 1 );

    /* test.case( "deref" ) */
    let mut got : the_module::Many< f32 > = core::iter::once( 13.0 ).into();
    a_id!( got.len(), 1 );
    a_id!( got.pop(), Some( 13.0 ) );

  }

}

//

tests_index!
{
  parameter_complex,
  parameter_no_derives,
  struct_basic,
  struct_no_derives,
}
