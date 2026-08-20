//!
//! The most trivial use case. Just generating a random number.
//!

// `Rng`` is re-exported from `rand` and `Hrng` stands for hierarchical random number generators.
#[ cfg(not(feature = "no_std")) ]
use deterministic_rand :: { RngExt, Hrng };

#[ allow(clippy ::used_underscore_binding) ] // `_got` is unused when `determinism` is off; the cfg-gated assert uses it when on
fn main()
{
  #[ cfg(not(feature = "no_std")) ]
  {
  // Make master random number generator with a seed.
  let hrng = Hrng ::master_with_seed("master1".into());
  // Get a reference to the current random number generator using a reference counter and mutex.
  let rng_ref = hrng.rng_ref();
  // Lock it producing a guard.
  let mut rng = rng_ref.lock().unwrap();
  // Generate a number.
  let _got: u64 = rng.random();
  // If determinism is enabled then sequence of generated rundom numbers will be the same.
  #[ cfg(feature = "determinism") ]
  assert_eq!(_got, 6_165_676_721_551_962_567);
 }
}
