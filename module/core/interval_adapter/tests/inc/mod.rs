#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  //

  fn info_from()
  {
    use TheModule::*;
    let exp = Interval::new( Bound::Included( 0 ), Bound::Included( 3 ) );

    let got : Interval< _ > = ( Bound::Included( 0 ), Bound::Included( 3 ) ).into();
    a_id!( got, exp );
    let got = ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval();
    a_id!( got, exp );

    let got : Interval< _ > = ( 0, 3 ).into();
    a_id!( got, exp );
    let got = ( 0, 3 ).into_interval();
    a_id!( got, exp );

    // let got : Interval< _ > = [ Bound::Included( 0 ), Bound::Included( 3 ) ].into();
    // a_id!( got, exp );
    let got = [ Bound::Included( 0 ), Bound::Included( 3 ) ].into_interval();
    a_id!( got, exp );

    // zzz : why does not work?
    // let got : Interval< _ > = [ 0, 3 ].into();
    // a_id!( got, exp );
    let got = [ 0, 3 ].into_interval();
    a_id!( got, exp );

  }

  //

  fn from_std()
  {
    use TheModule::*;

    let exp = Interval::new( Bound::Included( 0 ), Bound::Excluded( 4 ) );
    let got = ( 0..4 ).into_interval();
    a_id!( got, exp );
    let exp = ( Bound::Included( 0 ), Bound::Excluded( 4 ) );
    let got = ( 0..4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( Bound::Included( 0 ), Bound::Included( 4 ) );
    let got = ( 0..=4 ).into_interval();
    a_id!( got, exp );
    let exp = ( Bound::Included( 0 ), Bound::Included( 4 ) );
    let got = ( 0..=4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( Bound::Unbounded, Bound::Included( 4 ) );
    let got = ( ..4 ).into_interval();
    a_id!( got, exp );
    // let exp = ( Bound::Unbounded, Bound::Excluded( 4 ) );
    // let got = ( ..4 ).bounds();
    // a_id!( got, exp );
    // xxx : !

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn adapter_basic()
  {
    use TheModule::*;
    let src = Interval::new( Bound::Included( 2 ), Bound::Included( 4 ) );

    a_id!( IntervalAdapter::left( &src ), Bound::Included( 2 ) );
    a_id!( IntervalAdapter::right( &src ), Bound::Included( 4 ) );
    a_id!( IntervalAdapter::bounds( &src ), ( Bound::Included( 2 ), Bound::Included( 4 ) ) );
    a_id!( IntervalAdapter::closed_left( &src ), 2 );
    a_id!( IntervalAdapter::closed_right( &src ), 4 );
    a_id!( IntervalAdapter::closed_len( &src ), 3 );
    a_id!( IntervalAdapter::closed( &src ), ( 2, 4 ) );

    a_id!( src.left(), Bound::Included( 2 ) );
    a_id!( src.right(), Bound::Included( 4 ) );
    a_id!( src.bounds(), ( Bound::Included( 2 ), Bound::Included( 4 ) ) );
    a_id!( src.closed_left(), 2 );
    a_id!( src.closed_right(), 4 );
    a_id!( src.closed_len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn adapter_std_closed_open()
  {
    use TheModule::*;

    // test.case( "basic" );

    let src = 2..5;

    a_id!( src.left(), Bound::Included( 2 ) );
    a_id!( src.right(), Bound::Excluded( 5 ) );
    a_id!( src.bounds(), ( Bound::Included( 2 ), Bound::Excluded( 5 ) ) );
    a_id!( src.closed_left(), 2 );
    a_id!( src.closed_right(), 4 );
    a_id!( src.closed_len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn adapter_std_closed()
  {
    use TheModule::*;

    // test.case( "basic" );

    let src = 2..=4;

    a_id!( src.left(), Bound::Included( 2 ) );
    a_id!( src.right(), Bound::Included( 4 ) );
    a_id!( src.bounds(), ( Bound::Included( 2 ), Bound::Included( 4 ) ) );
    a_id!( src.closed_left(), 2 );
    a_id!( src.closed_right(), 4 );
    a_id!( src.closed_len(), 3 );
    a_id!( src.closed(), ( 2, 4 ) );

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn into_interval()
  {
    use TheModule::*;

    // test.case( "from closed open std interval" );

    let src : Interval = ( 2..5 ).into();
    a_id!( src.closed(), ( 2, 4 ) );
    let src = Interval::from( 2..5 );
    a_id!( src.closed(), ( 2, 4 ) );

    // test.case( "from closed std interval" );

    let src : Interval = ( 2..=4 ).into();
    a_id!( src.closed(), ( 2, 4 ) );
    let src = Interval::from( 2..=4 );
    a_id!( src.closed(), ( 2, 4 ) );

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn impl_interval()
  {
    use TheModule::{ IntervalAdapter, IntoInterval, Bound };

    //
    // Let's assume you have a function which should accept Interval.
    // But you don't want to limit caller of the function to use either half-open interval `core::ops::Range` or closed one `core::ops::RangeInclusive`.
    // To make that work smoothly use `IntervalAdapter`.
    // Both `core::ops::Range` and `core::ops::RangeInclusive` implement the trait.
    //
    fn f1( interval : impl IntervalAdapter )
    {
      for i in interval
      {
        println!( "{i}" );
      }
    }

    // Calling the function either with half-open interval `core::ops::Range`.
    f1( 0..=3 );
    // Or closed one `core::ops::RangeInclusive`.
    f1( 0..4 );
    // Alternatively you construct your custom interval from a tuple.
    f1( ( 0, 3 ).into_interval() );
    f1( ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval() );
    // All the calls to the function `f1`` perform the same task, and the output is exactly identical.

  }

}

//

tests_index!
{
  info_from,
  from_std,
  adapter_basic,
  adapter_std_closed,
  adapter_std_closed_open,
  into_interval,
  impl_interval,
}
