#[ allow( unused_imports ) ]
use super::*;

tests_impls!
{

  //

  fn info_from()
  {
    use TheModule::*;
    let exp = Interval::new( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) );

    let got : Interval< _ > = ( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ).into();
    a_id!( got, exp );
    let got = ( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ).into_interval();
    a_id!( got, exp );

    let got : Interval< _ > = ( 0, 3 ).into();
    a_id!( got, exp );
    let got = ( 0, 3 ).into_interval();
    a_id!( got, exp );

    let got : Interval< _ > = [ TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ].into();
    a_id!( got, exp );
    let got = [ TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ].into_interval();
    a_id!( got, exp );

    let got : Interval< _ > = [ 0, 3 ].into();
    a_id!( got, exp );
    let got = [ 0, 3 ].into_interval();
    a_id!( got, exp );

  }

  //

  fn from_std()
  {
    use TheModule::*;

    let exp = Interval::new( TheModule::Bound::Included( 0 ), TheModule::Bound::Excluded( 4 ) );
    let got = ( 0..4 ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::Included( 0 ), TheModule::Bound::Excluded( 4 ) );
    let got = ( 0..4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 4 ) );
    let got = ( 0..=4 ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 4 ) );
    let got = ( 0..=4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( TheModule::Bound::Unbounded, TheModule::Bound::Excluded( 4 ) );
    let got = ( ..4 ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::Unbounded, TheModule::Bound::Excluded( 4 ) );
    let got = ( ..4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( TheModule::Bound::Unbounded, TheModule::Bound::Included( 4 ) );
    let got = ( ..=4 ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::Unbounded, TheModule::Bound::Included( 4 ) );
    let got = ( ..=4 ).bounds();
    a_id!( got, exp );

    let exp = Interval::new( TheModule::Bound::Included( 4 ), TheModule::Bound::Unbounded );
    let got = ( 4.. ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::Included( 4 ), TheModule::Bound::Unbounded );
    let got = ( 4.. ).bounds();
    a_id!( got, exp );

    let exp = Interval::< isize >::new( TheModule::Bound::Unbounded, TheModule::Bound::Unbounded );
    let got = ( .. ).into_interval();
    a_id!( got, exp );
    let exp = ( TheModule::Bound::< isize >::Unbounded, TheModule::Bound::< isize >::Unbounded );
    let got = ( .. ).bounds();
    a_id!( got, exp );

  }

  //

  // #[ cfg( not( feature = "no_std" ) ) ]
  fn adapter_basic()
  {
    use TheModule::*;
    let src = Interval::new( TheModule::Bound::Included( 2 ), TheModule::Bound::Included( 4 ) );

    a_id!( NonIterableInterval::left( &src ), TheModule::Bound::Included( 2 ) );
    a_id!( NonIterableInterval::right( &src ), TheModule::Bound::Included( 4 ) );
    a_id!( NonIterableInterval::bounds( &src ), ( TheModule::Bound::Included( 2 ), TheModule::Bound::Included( 4 ) ) );
    a_id!( NonIterableInterval::closed_left( &src ), 2 );
    a_id!( NonIterableInterval::closed_right( &src ), 4 );
    a_id!( NonIterableInterval::closed_len( &src ), 3 );
    a_id!( NonIterableInterval::closed( &src ), ( 2, 4 ) );

    a_id!( src.left(), TheModule::Bound::Included( 2 ) );
    a_id!( src.right(), TheModule::Bound::Included( 4 ) );
    a_id!( src.bounds(), ( TheModule::Bound::Included( 2 ), TheModule::Bound::Included( 4 ) ) );
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

    a_id!( src.left(), TheModule::Bound::Included( 2 ) );
    a_id!( src.right(), TheModule::Bound::Excluded( 5 ) );
    a_id!( src.bounds(), ( TheModule::Bound::Included( 2 ), TheModule::Bound::Excluded( 5 ) ) );
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

    a_id!( src.left(), TheModule::Bound::Included( 2 ) );
    a_id!( src.right(), TheModule::Bound::Included( 4 ) );
    a_id!( src.bounds(), ( TheModule::Bound::Included( 2 ), TheModule::Bound::Included( 4 ) ) );
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
    use TheModule::{ NonIterableInterval, IterableInterval, IntoInterval, Bound };

    //
    // Let's assume you have a function which should accept Interval.
    // But you don't want to limit caller of the function to use either half-open interval `core::ops::Range` or closed one `core::ops::RangeInclusive`.
    // To make that work smoothly use `IterableInterval`.
    // Both `core::ops::Range` and `core::ops::RangeInclusive` implement the trait.
    //
    fn f1( interval : impl IterableInterval )
    {
      for i in interval
      {
        println!( "{i}" );
      }
    }

    // Calling the function either with
    // half-open interval `core::ops::Range`.
    f1( 0..=3 );
    // Or closed one `core::ops::RangeInclusive`.
    f1( 0..4 );
    // Alternatively you construct your custom interval from a tuple.
    f1( ( 0, 3 ).into_interval() );
    f1( ( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ).into_interval() );
    // All the calls to the function `f1`` perform the same task,
    // and the output is exactly identical.

  }

  fn non_interable_smoke()
  {
    use TheModule::{ NonIterableInterval, IntoInterval };

    fn f1( interval : impl NonIterableInterval )
    {
      println!( "Do something with this {:?} .. {:?} interval", interval.left(), interval.right() );
    }

    // Iterable/bound interval from tuple.
    f1( ( TheModule::Bound::Included( 0 ), TheModule::Bound::Included( 3 ) ).into_interval() );
    // Non-iterable/unbound interval from tuple.
    f1( ( TheModule::Bound::Included( 0 ), TheModule::Bound::Unbounded ).into_interval() );
    // Non-iterable/unbound interval from `core::ops::RangeFrom`.
    f1( 0.. );
    // Non-iterable/unbound interval from `core::ops::RangeFull`
    // what is ( -Infinity .. +Infinity ).
    f1( .. );
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
  non_interable_smoke,
}
// zzz : qqq : fix problem of wrong line
