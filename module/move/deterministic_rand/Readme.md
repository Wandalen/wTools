# Module :: deterministic_rand

<!-- {{# generate.module_header{} #}} -->
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewLangPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewLangPush.yml) [![docs.rs](https://img.shields.io/docsrs/deterministic_rand?color=e3e8f0&logo=docs.rs)](https://docs.rs/deterministic_rand) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwlang_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wlang_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Hierarchical random number generators for concurrent simulations with switchable determinism.

This library introduces hierarchical random number generators designed for concurrent simulations, offering the flexibility of switchable determinism.

## Addressing Deterministic Challenges

### Non-Reproducible Rand

While the standard StdRng in the `rand` crate is deterministic, its reproducibility is compromised by
configuration dependencies and the potential for future library updates. Our solution leverages the
`rand_chacha` crate internally, presenting a hierarchical random number generator that ensures reproducibility.

### Ensuring Determinism in Parallel Computation

In parallel computation scenarios, utilizing the same random number generator across threads can result in
non-deterministic outcomes due to racing conditions. This crate resolves this issue by assigning each
thread its dedicated generator, guaranteeing deterministic results in parallel computations.

### Random Hash{Map,Set} Ordering

The default randomized ordering of values in HashMap and HashSet can introduce non-deterministic behavior.
This crate offers an optional sorting feature, empowering users to stabilize the order of iterated values.
This functionality ensures determinism in situations where the order of collection elements holds significance.

## Use-cases

### Basic use-case

```rust
use rand::Rng;
use deterministic_rand::Hrng;

let hrng = Hrng::master();
let rng_ref = hrng.rng();
let mut rng = rng.lock().unwrap();
let got : u64 = rng.gen();
assert_eq!( got, 6165676721551962567 );
```

### Paralleled use-case

```rust
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;
use deterministic_rand::Hrng;

let range = Uniform::new( -1.0f64, 1.0 );

let manager = Hrng::master();
let got = ( 0..1000 )
  .into_par_iter()
  .map
  (
    |i|
    {
      let child = manager.child( i );
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
assert_eq!( got_pi, 3.1410448 );
println!("PI = {got_pi}");
```

### `Hash{Map,Set}` use-case

```rust
use std::collections::HashMap;
use deterministic_rand::IfDeterminismIteratorExt;

let map: HashMap<_, _> = HashMap::from_iter( [ ( 1, "first" ), ( 2, "second" ), ( 3, "third" ) ] );
let first = map.into_iter().if_determinism_then_sort_by( | ( i, _ ), ( j, _ ) | i.cmp(&j) ).next();
assert_eq!( first, Some( ( 1, "first" ) ) );
```

### To add to your project

```bash
cargo add deterministic_rand
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example sample_deterministic_rand_trivial
```
