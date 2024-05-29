
use rand::Rng;
use deterministic_rand::Hrng;

#[ test ]
fn assumption_gen()
{
  let rng = Hrng::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let _got : u64 = rng.gen();
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got, 6165676721551962567 );
  let _got : u64 = rng.gen();
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got, 15862033778988354993 );

  let rng = Hrng::master().rng_ref();
  let mut rng = rng.lock().unwrap();
  let _got : u64 = rng.gen();
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got, 6165676721551962567 );
  let _got : u64 = rng.gen();
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  assert_eq!( _got, 15862033778988354993 );
}

#[ test ]
fn assumption_choose()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  {
    use rand::seq::IteratorRandom;
    let rng = Hrng::master().rng_ref();
    let mut rng = rng.lock().unwrap();
    let got = ( 1..1000 ).choose( &mut *rng ).unwrap();
    assert_eq!( got, 334 );
    let got = ( 1..1000 ).choose( &mut *rng ).unwrap();
    assert_eq!( got, 421 );
    let got : u64 = rng.gen();
    assert_eq!( got, 11385630238607229870 );
  }
}

#[ test ]
fn assumption_choose_stable()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  {
    use rand::seq::IteratorRandom;
    let rng = Hrng::master().rng_ref();
    let mut rng = rng.lock().unwrap();
    let got = ( 1..1000 ).choose_stable( &mut *rng ).unwrap();
    assert_eq!( got, 704 );
    let got = ( 1..1000 ).choose_stable( &mut *rng ).unwrap();
    assert_eq!( got, 511 );
    let got : u64 = rng.gen();
    assert_eq!( got, 18025856250180898108 );
  }
}

#[ test ]
fn assumption_choose_multiple()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  {
    use rand::seq::{ IteratorRandom, SliceRandom };
    let rng = Hrng::master().rng_ref();
    let mut rng = rng.lock().unwrap();
    let got = ( 1..1000 ).choose_multiple( &mut *rng, 10 );
    assert_eq!( got, vec![ 704, 2, 359, 578, 198, 219, 884, 649, 696, 532 ] );

    let got = ( 1..1000 ).choose_multiple( &mut *rng, 10 );
    assert_eq!( got, vec![ 511, 470, 835, 820, 26, 776, 261, 278, 828, 765 ] );

    let got = ( 1..1000 )
    .collect::< Vec< _ > >()
    .choose_multiple( &mut *rng, 10 )
    .copied()
    .collect::< Vec< _ > >();
    assert_eq!( got, vec![ 141, 969, 122, 311, 926, 11, 987, 184, 888, 423 ] );

    let got = ( 1..1000 )
    .collect::< Vec< _ > >()
    .choose_multiple( &mut *rng, 10 )
    .copied()
    .collect::< Vec< _ > >();
    assert_eq!( got, vec![ 637, 798, 886, 412, 652, 688, 71, 854, 639, 282 ] );
  }
}

#[ test ]
fn assumption_choose_weighted()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  {
     use deterministic_rand::seq::SliceRandom;
    let rng = Hrng::master().rng_ref();
    let mut rng = rng.lock().unwrap();
    let got = ( 1..1000 )
    .zip( ( 1..1000 ).rev() )
    .into_iter()
    .collect::< Vec< _ > >()
    .choose_weighted( &mut *rng, |w| w.0 )
    .map( |( i, j )| ( *i, *j ) )
    .unwrap();
    assert_eq!( got, ( 800, 200 ) );

    let got = ( 1..1000 )
    .zip( ( 1..1000 ).rev() )
    .into_iter()
    .collect::< Vec< _ > >()
    .choose_weighted( &mut *rng, |w| w.0 )
    .map( |( i, j )| ( *i, *j ) )
    .unwrap();
    assert_eq!( got, ( 578, 422 ) );
  }
}

#[ test ]
fn assumption_choose_multiple_weighted()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  #[ cfg( feature = "determinism" ) ]
  {
     use deterministic_rand::seq::SliceRandom;
    let rng = Hrng::master().rng_ref();
    let mut rng = rng.lock().unwrap();
    let got = ( 1..10 )
    .zip( ( 1..10 ).rev() )
    .into_iter()
    .collect::< Vec< _ > >()
    .choose_multiple_weighted( &mut *rng, 10, |w| w.0 )
    .unwrap()
    .map( |( i, j )| ( *i, *j ) )
    .collect::< Vec< _ > >();
    assert_eq!
    (
      got,
      vec!
      [
        ( 8, 2 ),
        ( 7, 3 ),
        ( 9, 1 ),
        ( 5, 5 ),
        ( 2, 8 ),
        ( 3, 7 ),
        ( 4, 6 ),
        ( 6, 4 ),
        ( 1, 9 )
      ]
    );

    let got = ( 1..10 )
    .zip( ( 1..10 ).rev() )
    .into_iter()
    .collect::< Vec< _ > >()
    .choose_multiple_weighted( &mut *rng, 10, |w| w.0 )
    .unwrap()
    .map( |( i, j )| ( *i, *j ) )
    .collect::< Vec< _ > >();
    assert_eq!
    (
      got,
      vec!
      [
        ( 5, 5 ),
        ( 6, 4 ),
        ( 8, 2 ),
        ( 7, 3 ),
        ( 2, 8 ),
        ( 3, 7 ),
        ( 9, 1 ),
        ( 4, 6 ),
        ( 1, 9 )
      ]
    );
  }
}

#[ cfg( feature = "determinism" ) ]
#[ test ]
fn assumption_streams_switching()
{
  use rand::{ RngCore, SeedableRng };
  use rand_chacha::ChaCha8Rng;

  let a = 6234031553773679537;
  let b = 5421492469564588225;

  let mut master = ChaCha8Rng::seed_from_u64( 13 );
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_eq!( got, a );
  master.set_stream( 1 );
  let _got = master.next_u64();
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_eq!( got, b );

  let mut master = ChaCha8Rng::seed_from_u64( 13 );
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_eq!( got, a );
  master.set_stream( 0 );
  let _got = master.next_u64();
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_eq!( got, b );
}

#[ cfg( feature = "determinism" ) ]
#[ test ]
fn assumption_streams_same_source()
{
  use rand::{ RngCore, SeedableRng };
  use rand_chacha::ChaCha8Rng;

  let a = 6234031553773679537;
  let b = 2305422516838604614;

  let mut master = ChaCha8Rng::seed_from_u64( 13 );
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_eq!( got, a );
  master.set_stream( 1 );
  let got = master.next_u64();
  assert_eq!( got, b );

  let mut master = ChaCha8Rng::seed_from_u64( 13 );
  master.set_stream( 1 );
  let got = master.next_u64();
  assert_ne!( got, a );
  assert_ne!( got, b );
  master.set_stream( 0 );
  let got = master.next_u64();
  assert_ne!( got, a );
  assert_ne!( got, b );
}
