//! Usage of `NonIterableInterval` trait with unbounded intervals.
//!
//! Demonstrates how `NonIterableInterval` trait accepts both bounded and unbounded intervals,
//! including `RangeFrom` (0..), `RangeFull` (..), and custom `Bound` tuples. Shows that unbounded
//! intervals cannot be iterated but can be inspected via `left()` and `right()` methods.
//!
//! Expected output: Prints debug representation of 4 different interval types including
//! bounded (0..3), semi-unbounded (0..∞), and fully unbounded (-∞..+∞) intervals.
fn main() 
{
  use interval_adapter :: { NonIterableInterval, IntoInterval, Bound };

  fn f1(interval: &impl NonIterableInterval) 
  {
  println!(
   "Do something with this {:?} .. {:?} interval",
   interval.left(),
   interval.right()
 );
 }

  // Iterable/bound interval from tuple.
  f1(&(Bound ::Included(0), Bound ::Included(3)).into_interval());
  // Non-iterable/unbound interval from tuple.
  f1(&(Bound ::Included(0), Bound ::Unbounded).into_interval());
  // Non-iterable/unbound interval from `core ::ops ::RangeFrom`.
  f1(&(0..));
  // Non-iterable/unbound interval from `core ::ops ::RangeFull`
  // what is ( -Infinity .. +Infinity ).
  f1(&(..));
}
