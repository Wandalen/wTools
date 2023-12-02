//! Example usage of deterministic rand with parallel iterators
//!
//! Output is deterministic and is an approximate value of PI

use rand::{ distributions::Uniform, Rng };
use rayon::prelude::*;
use deterministic_rand::Hrng;

fn main() {
  let range = Uniform::new( -1.0f64, 1.0 );

  let hrng = Hrng::master();
  let rng_ref = hrng.rng();
  let mut rng = rng_ref.lock().unwrap();
  let _got : u64 = rng.gen();
  // Random if feature determinism is off.
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got, 6165676721551962567 );
  let got = ( 0..1000 )
  .into_par_iter()
  .map
  (
    |i|
    {
      let child = hrng.child( i );
      let rng = child.rng();
      let mut rng = rng.lock().unwrap();
      let mut count = 0;
      for _ in 0..10_000
      {
        let a = rng.sample( &range );
        let b = rng.sample( &range );
        if a * a + b * b <= 1.0
        {
          count += 1;
        }
      }
      count
    }
  )
  .sum::< u64 >();
  let got_pi = 4. * ( got as f64 ) / ( ( 10_000 * 1000 ) as f64 );
  // Random if feature determinism is off.
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( got_pi, 3.1410448 );
  println!( "PI = {got_pi}" );
}
