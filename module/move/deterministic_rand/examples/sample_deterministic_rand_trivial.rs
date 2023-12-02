//!
//! The most trivial use case. Just generating a random number.
//!

// `Rng`` is re-exported from `rand` and `Hrng` stands for hierarchical random number generators.
use deterministic_rand::{ Rng, Hrng };

fn main()
{
  // Make master random number generator with a seed.
  let hrng = Hrng::master_with_seed( "master1".into() );
  // Get a reference to the current random number generator using a reference counter and mutex.
  let rng_ref = hrng.rng_ref();
  // Lock it producing a guard.
  let mut rng = rng_ref.lock().unwrap();
  // Generate a number.
  let got : u64 = rng.gen();
  // If determinism is enabled then sequence of generated rundom numbers will be the same.
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( got, 6165676721551962567 );
}
