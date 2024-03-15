
use rand::distributions::Uniform;
use rayon::prelude::*;

#[test]
fn test_rng_manager()
{
  use deterministic_rand::{ Hrng, Rng };
  let range = Uniform::new( -1.0f64, 1.0 );

  let hrng = Hrng::master();
  let got = ( 0..100 )
  .into_par_iter()
  .map( |i|
  {
    let child = hrng.child( i );
    let rng_ref = child.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let mut count = 0;
    for _ in 0..1000
    {
      let a = rng.sample( &range );
      let b = rng.sample( &range );
      if a * a + b * b <= 1.0
      {
        count += 1;
      }
    }
    count
  } )
  .sum::< u64 >();
  let _got_pi = 4. * ( got as f64 ) / ( ( 100 * 1000 ) as f64 );
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got_pi, 3.1438 )
}

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
#[test]
fn test_reusability()
{
  use deterministic_rand::{ Hrng, Rng };
  let mut expected: [u64; 4] = [0; 4];

  let hrng = Hrng::master();
  {
    let child1 = hrng.child( 0 );
    let child1_ref = child1.rng_ref();
    let mut rng1 = child1_ref.lock().unwrap();
    let got = rng1.gen::< u64 >();
    expected[0] = got;
    let got = rng1.gen::< u64 >();
    expected[1] = got;
  }
  {
    let child1 = hrng.child( 0 );
    let child1_ref = child1.rng_ref();
    let mut rng1 = child1_ref.lock().unwrap();
    let got = rng1.gen::< u64 >();
    expected[2] = got;
    let got = rng1.gen::< u64 >();
    expected[3] = got;
  }
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( hrng._children_len(), 1 );
  #[ cfg( not( feature = "determinism" ) ) ]
  assert_eq!( hrng._children_len(), 0 );

  let hrng = Hrng::master();
  {
    let child1 = hrng.child( 0 );
    let child1_ref = child1.rng_ref();
    let mut rng1 = child1_ref.lock().unwrap();
    let got = rng1.gen::< u64 >();
    assert_eq!( got, expected[0] );
    let got = rng1.gen::< u64 >();
    assert_eq!( got, expected[1] );
  }
  {
    let child1 = hrng.child( 0 );
    let child1_ref = child1.rng_ref();
    let mut rng1 = child1_ref.lock().unwrap();
    let got = rng1.gen::< u64 >();
    assert_eq!( got, expected[2] );
    let got = rng1.gen::< u64 >();
    assert_eq!( got, expected[3] );
  }
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( hrng._children_len(), 1 );
  #[ cfg( not( feature = "determinism" ) ) ]
  assert_eq!( hrng._children_len(), 0 );
}

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
#[test]
fn test_par()
{
  use std::sync::{ Arc, Mutex };
  use deterministic_rand::{ Hrng, Rng };
  let expected: ( Arc<Mutex<( u64, u64 )>>, Arc<Mutex<( u64, u64 )>> ) =
  ( Arc::new( Mutex::new( ( 0, 0 ) ) ), Arc::new( Mutex::new( ( 0, 0 ) ) ) );

  let hrng = Hrng::master();
  ( 1..=2 )
  .into_par_iter()
  .map( |i| ( i, hrng.child( i ) ) )
  .for_each( |( i, child )|
  {
    let got1 = child.rng_ref().lock().unwrap().gen::< u64 >();
    let got2 = child.rng_ref().lock().unwrap().gen::< u64 >();
    match i {
      1 => *expected.0.lock().unwrap() = ( got1, got2 ),
      2 => *expected.1.lock().unwrap() = ( got1, got2 ),
      _ => unreachable!(),
    }
  } );

  let hrng = Hrng::master();
  ( 1..=2 )
  .into_par_iter()
  .map( |i| ( i, hrng.child( i ) ) )
  .for_each( |( i, child )|
  {
    let got1 = child.rng_ref().lock().unwrap().gen::< u64 >();
    let got2 = child.rng_ref().lock().unwrap().gen::< u64 >();
    match i
    {
      1 => assert_eq!( ( got1, got2 ), *expected.0.lock().unwrap() ),
      2 => assert_eq!( ( got1, got2 ), *expected.1.lock().unwrap() ),
      _ => unreachable!(),
    }
  } );
}

#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
#[test]
fn seed()
{
  use deterministic_rand::Seed;
  let seed = Seed::random();
  println!( "{seed:?}" );
  assert!( seed.into_inner().len() == 16 );
}
