//! qqq : write proper description
fn main()
{
  use interval_adapter::{ NonIterableInterval, IntoInterval, Bound };

  fn f1( interval : impl NonIterableInterval )
  {
    println!( "Do something with this {:?} .. {:?} interval", interval.left(), interval.right() );
  }

  // Iterable/bound interval from tuple.
  f1( ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval() );
  // Non-iterable/unbound interval from tuple.
  f1( ( Bound::Included( 0 ), Bound::Unbounded ).into_interval() );
  // Non-iterable/unbound interval from `core::ops::RangeFrom`.
  f1( 0.. );
  // Non-iterable/unbound interval from `core::ops::RangeFull`
  // what is ( -Infinity .. +Infinity ).
  f1( .. );
}
