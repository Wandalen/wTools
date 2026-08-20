#![allow(missing_docs)]

use rand ::RngExt;
use deterministic_rand ::Hrng;

#[ test ]
#[ allow(clippy ::used_underscore_binding) ] // `_got` is unused when `determinism` is off; the cfg-gated asserts use it when on
fn assumption_gen()
{
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let _got: u64 = rng.random();
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  assert_eq!(_got, 6_165_676_721_551_962_567);
  let _got: u64 = rng.random();
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  assert_eq!(_got, 15_862_033_778_988_354_993);

  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let _got: u64 = rng.random();
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  assert_eq!(_got, 6_165_676_721_551_962_567);
  let _got: u64 = rng.random();
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  assert_eq!(_got, 15_862_033_778_988_354_993);
}

#[ test ]
fn assumption_choose() 
{
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  {
  use rand ::seq ::IteratorRandom;
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let got = (1..1000).choose(&mut *rng).unwrap();
  assert_eq!(got, 640);
  let got = (1..1000).choose(&mut *rng).unwrap();
  assert_eq!(got, 334);
  let got: u64 = rng.random();
  assert_eq!(got, 15_862_033_778_988_354_993);
 }
}

#[ test ]
fn assumption_choose_stable() 
{
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  {
  use rand ::seq ::IteratorRandom;
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let got = (1..1000).choose_stable(&mut *rng).unwrap();
  assert_eq!(got, 256);
  let got = (1..1000).choose_stable(&mut *rng).unwrap();
  assert_eq!(got, 598);
  let got: u64 = rng.random();
  assert_eq!(got, 16_297_902_690_204_926_191);
 }
}

#[ test ]
fn assumption_choose_multiple() 
{
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  {
  use rand ::seq :: { IteratorRandom, IndexedRandom };
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let got = (1..1000).sample(&mut *rng, 10);
  assert_eq!(got, vec![552, 39, 715, 532, 832, 46, 388, 308, 104, 911]);

  let got = (1..1000).sample(&mut *rng, 10);
  assert_eq!(got, vec![764, 464, 96, 16, 181, 142, 302, 824, 453, 341]);

  let got = (1..1000)
   .collect :: < Vec<_ >>()
   .sample(&mut *rng, 10)
   .copied()
   .collect :: < Vec<_ >>();
  assert_eq!(got, vec![590, 157, 722, 863, 902, 790, 159, 749, 416, 314]);

  let got = (1..1000)
   .collect :: < Vec<_ >>()
   .sample(&mut *rng, 10)
   .copied()
   .collect :: < Vec<_ >>();
  assert_eq!(got, vec![187, 273, 778, 456, 513, 154, 294, 118, 965, 471]);
 }
}

#[ test ]
fn assumption_choose_weighted() 
{
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  {
  use rand ::seq ::IndexedRandom;
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let got = (1..1000)
   .zip((1..1000).rev())
   .collect :: < Vec<_ >>()
   .choose_weighted(&mut *rng, |w| w.0)
   .map(|(i, j)| (*i, *j))
   .unwrap();
  assert_eq!(got, (800, 200));

  let got = (1..1000)
   .zip((1..1000).rev())
   .collect :: < Vec<_ >>()
   .choose_weighted(&mut *rng, |w| w.0)
   .map(|(i, j)| (*i, *j))
   .unwrap();
  assert_eq!(got, (578, 422));
 }
}

#[ test ]
fn assumption_choose_multiple_weighted() 
{
  #[ cfg(not(feature = "no_std")) ]
  #[ cfg(feature = "determinism") ]
  {
  use rand ::seq ::IndexedRandom;
  let rng = Hrng ::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let got = (1..10)
   .zip((1..10).rev())
   .collect :: < Vec<_ >>()
   .sample_weighted(&mut *rng, 10, |w| w.0)
   .unwrap()
   .map(|(i, j)| (*i, *j))
   .collect :: < Vec<_ >>();
  assert_eq!(
   got,
   vec![(1, 9), (4, 6), (6, 4), (2, 8), (5, 5), (3, 7), (7, 3), (8, 2), (9, 1)]
 );

  let got = (1..10)
   .zip((1..10).rev())
   .collect :: < Vec<_ >>()
   .sample_weighted(&mut *rng, 10, |w| w.0)
   .unwrap()
   .map(|(i, j)| (*i, *j))
   .collect :: < Vec<_ >>();
  assert_eq!(
   got,
   vec![(1, 9), (4, 6), (3, 7), (9, 1), (5, 5), (6, 4), (7, 3), (8, 2), (2, 8)]
 );
 }
}

#[ cfg(feature = "determinism") ]
#[ test ]
fn assumption_streams_switching() 
{
  use rand :: { Rng, SeedableRng };
  use rand_chacha ::ChaCha8Rng;

  let a = 6_234_031_553_773_679_537;
  let b = 5_421_492_469_564_588_225;

  let mut master = ChaCha8Rng ::seed_from_u64(13);
  master.set_stream(0);
  let got = master.next_u64();
  assert_eq!(got, a);
  master.set_stream(1);
  let _got = master.next_u64();
  master.set_stream(0);
  let got = master.next_u64();
  assert_eq!(got, b);

  let mut master = ChaCha8Rng ::seed_from_u64(13);
  master.set_stream(0);
  let got = master.next_u64();
  assert_eq!(got, a);
  master.set_stream(0);
  let _got = master.next_u64();
  master.set_stream(0);
  let got = master.next_u64();
  assert_eq!(got, b);
}

#[ cfg(feature = "determinism") ]
#[ test ]
fn assumption_streams_same_source() 
{
  use rand :: { Rng, SeedableRng };
  use rand_chacha ::ChaCha8Rng;

  let a = 6_234_031_553_773_679_537;
  let b = 2_305_422_516_838_604_614;

  let mut master = ChaCha8Rng ::seed_from_u64(13);
  master.set_stream(0);
  let got = master.next_u64();
  assert_eq!(got, a);
  master.set_stream(1);
  let got = master.next_u64();
  assert_eq!(got, b);

  let mut master = ChaCha8Rng ::seed_from_u64(13);
  master.set_stream(1);
  let got = master.next_u64();
  assert_ne!(got, a);
  assert_ne!(got, b);
  master.set_stream(0);
  let got = master.next_u64();
  assert_ne!(got, a);
  assert_ne!(got, b);
}
