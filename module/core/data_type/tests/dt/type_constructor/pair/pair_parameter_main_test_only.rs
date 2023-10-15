#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{
  fn main()
  {
    use core::fmt;

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair1 = ( 13.0, 31.0 ).into();
    let instance2 = Pair1::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    assert!( !implements!( instance1 => PartialEq ) );
    assert!( !implements!( instance1 => Clone ) );
    assert!( !implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );
    assert!( implements!( instance1 => PartialEq ) );
    assert!( implements!( instance1 => Clone ) );
    assert!( implements!( instance1 => fmt::Debug ) );
    assert!( !implements!( instance1 => Default ) );

    /* test.case( "from tuple into Pair2" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = Pair2::from( ( 13.0, 31.0 ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from itself into itself" ) */
    let instance1 : Pair2 = ( Pair2::from( ( 13.0, 31.0 ) ) ).into();
    let instance2 = Pair2::from( Pair2::from( ( 13.0, 31.0 ) ) );
    a_id!( instance1.0, 13.0 );
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    /* test.case( "from Pair2 into tuple" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got : ( _, _ ) = instance1.into();
    a_id!( got, ( 13.0, 31.0 ) );
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let got = <( f32, f64 )>::from( instance1 );
    a_id!( got, ( 13.0, 31.0 ) );

    /* test.case( "clone / eq" ) */
    let instance1 : Pair2 = ( 13.0, 31.0 ).into();
    let instance2 = instance1.clone();
    a_id!( instance2.0, 13.0 );
    a_id!( instance1, instance2 );

    // /* test.case( "deref" ) */
    // let got : Pair2 = ( 13.5, 15.5 ).into();
    // a_id!( got.round(), 14.0 );

  }
}

//

tests_index!
{
  main,
}
