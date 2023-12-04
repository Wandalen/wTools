//! Example usage of deterministic rand with parallel iterators.
//!
//! Monte Carlo method for approximate value of PI.
//!
//! To address the non-determinism caused by parallelism, HRNG create a child random number generator for each dataflow lane. The key is to tie the generator not to the thread ID but to the batch ID. This ensures that no matter which thread handles the job, the sequence of random numbers remains consistent and is determined solely by the batch ID.
//!
//! Internally, a hierarchical random number generator employs a dedicated RNG to produce offspring. This ensures consistent outcomes regardless of when a child generator is created. Additionally, this approach enhances performance by minimizing concurrent clashes between the parent and child generators over shared resources.
//!
//! If you don't have batch ID consider enumerating your items to and use key as batch ID.

// Import necessary traits and modules from the `rayon` and `deterministic_rand` crates.
use rayon::prelude::*;
use deterministic_rand::{ distributions::Uniform, Rng, Hrng };

fn main()
{

  // Define a range for random number generation between -1.0 and 1.0.
  let range = Uniform::new( -1.0f64, 1.0 );

  // Create a master hierarchical random number generator (HRNG).
  let manager = Hrng::master();

  // Launch a parallel iteration over a range of numbers (0 to 999).
  let got = ( 0..1000 )
  .into_par_iter()
  .map
  (
    | i |
    {
      // For each barch, create a child HRNG tied to the current batch ID.
      let child = manager.child( i );
      // Get a reference to current RNG.
      let rng = child.rng_ref();
      // Lock the RNG to ensure safe access in the concurrent context.
      let mut rng = rng.lock().unwrap();

      // Initialize a counter for each iteration.
      let mut count = 0;
      // Perform 10,000 random draws.
      for _ in 0..10_000
      {
        // Sample two numbers from the range and calculate their positions.
        let a = rng.sample( &range );
        let b = rng.sample( &range );

        // If the point (a, b) lies within a unit circle, increment the count.
        if a * a + b * b <= 1.0
        {
          count += 1;
        }
      }

      // Return the count for this iteration.
      count
    }
  )
  // Sum the counts from all iterations.
  .sum::< u64 >();

  // Calculate an approximation of Pi using the Monte Carlo method.
  let got_pi = 4. * ( got as f64 ) / ( ( 10_000 * 1000 ) as f64 );

  // If determinism is enabled, assert that the calculated value of Pi matches the expected result.
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( got_pi, 3.1410448 );

  // Print the calculated value of Pi.
  println!( "PI = {got_pi}" );

}
