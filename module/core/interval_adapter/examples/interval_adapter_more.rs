//! qqq : write proper description
fn main()
{
  use interval_adapter::{ IterableInterval, IntoInterval, Bound };

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

  // Calling the function either with half-open interval `core::ops::Range`.
  f1( 0..=3 );
  // Or closed one `core::ops::RangeInclusive`.
  f1( 0..4 );
  // Alternatively you construct your custom interval from a tuple.
  f1( ( 0, 3 ).into_interval() );
  f1( ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval() );
  // All the calls to the function `f1`` perform the same task, and the output is exactly identical.

}
