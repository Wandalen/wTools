//! Basic usage of `IterableInterval` trait with standard range types.
//!
//! Demonstrates how to write generic functions that accept any iterable interval type,
//! allowing callers to use either half-open `Range` (0..4) or closed `RangeInclusive` (0..=3)
//! interchangeably. Both produce identical iteration results.
//!
//! Expected output: Prints 0,1,2,3 twice (once per function call).
fn main() 
{
  use interval_adapter ::IterableInterval;

  //
  // Let's assume you have a function which should accept Interval.
  // But you don't want to limit caller of the function to use either half-open interval `core :: ops :: Range` or closed one `core :: ops :: RangeInclusive`.
  // To make that work smoothly use `IterableInterval`.
  // Both `core :: ops :: Range` and `core :: ops :: RangeInclusive` implement the trait.
  //
  fn f1( interval : impl IterableInterval )
  {
    for i in interval
    {
      println!( "{i}" );
    }
  }

  // Calling the function either with half-open interval `core ::ops ::Range`.
  f1(0..=3);
  // Or closed one `core ::ops ::RangeInclusive`.
  f1(0..4);
}
