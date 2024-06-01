# Module :: deterministic_rand

<!-- {{# generate.module_header{} #}} -->
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_deterministic_rand_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_deterministic_rand_push.yml) [![docs.rs](https://img.shields.io/docsrs/deterministic_rand?color=e3e8f0&logo=docs.rs)](https://docs.rs/deterministic_rand) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Fdeterministic_rand%2Fexamples%2Fdeterministic_rand_trivial.rs,RUN_POSTFIX=--example%20deterministic_rand_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Hierarchical random number generators for concurrent simulations with switchable determinism.

This library introduces hierarchical random number generators designed for concurrent simulations, offering the flexibility of switchable determinism.

### Rationale

Deterministic randomness, also known as pseudo-randomness, finds its application in various fields beyond algorithmic solutions for NP-hard problems and multiplayer gaming. Here are some other notable applications:

*Cryptography*: In cryptography, deterministic randomness is essential for generating secure keys, cryptographic nonces, and for various encryption algorithms. Pseudo-random number generators (PRNGs) need to produce output that is indistinguishable from true randomness to ensure security.

*Simulation and Modeling*: In scientific simulations, such as those in physics, biology, or economics, deterministic randomness is used to model complex systems with inherent uncertainties. This allows for reproducibility of results, which is crucial for verification and validation of models.

*Computer Graphics*: In procedural generation, such as terrain or texture generation in computer graphics, deterministic randomness can create diverse yet consistent visuals. This is widely used in video games and simulations.

*Load Testing*: In software engineering, deterministic randomness is used in load and stress testing of systems. By simulating user behavior or system inputs in a controlled, repeatable manner, developers can identify and rectify potential performance issues.

*Machine Learning*: Some machine learning algorithms, especially those involving stochastic processes like stochastic gradient descent, use deterministic randomness to ensure reproducibility of results while still benefiting from randomness in the training process.

*Statistical Sampling*: In statistics, pseudo-random number generators are used for random sampling and other statistical methods where reproducibility is essential.

*Quantum Computing Simulation*: Simulating quantum computers on classical machines often requires deterministic randomness to emulate the probabilistic nature of quantum mechanics.

*Algorithmic Art*: In generative art, deterministic randomness helps in creating complex and appealing patterns and images that are reproducible.

*Financial Modeling*: In finance, deterministic randomness is used in Monte Carlo simulations for risk assessment and option pricing, where numerous scenarios are generated to model the behavior of financial markets.

*Educational Tools and Demonstrations*: In teaching concepts of probability and randomness, deterministic algorithms allow educators to demonstrate principles using repeatable experiments.

These applications leverage the balance that deterministic randomness provides between unpredictability and reproducibility, making it a versatile tool in many fields.

### Sources of non-determinism

A random number generator is the most obvious source of randomness, but it's not the only one. Among the sources of randomness in programs are:

- Random Number Generator (e.g., `rand`)
- Parallelism (e.g., `rayon`)
- Standard Library (e.g., keys of HashMap and HashSet)
- System Time
- Memory Addresses
- Database Query Results
- Quantum Randomness

The `deterministic_rand` provides means to address the first three sources of randomness.

### Basic use-case

The most trivial use case. Just generating a random number.

```rust
#[ cfg( not( feature = "no_std" ) ) ]
{
  // `Rng`` is re-exported from `rand` and `Hrng` stands for hierarchical random number generators.
  use deterministic_rand::{ Rng, Hrng };
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
  assert_eq!( got, 8185996568056992464 );
}
```

### How to deal with parallelism-caused non-determinism

To address the non-determinism caused by parallelism, HRNG create a child random number generator for each dataflow lane. The key is to tie the generator not to the thread ID but to the batch ID. This ensures that no matter which thread handles the job, the sequence of random numbers remains consistent and is determined solely by the batch ID.

Internally, a hierarchical random number generator employs a dedicated RNG to produce offspring. This ensures consistent outcomes regardless of when a child generator is created. Additionally, this approach enhances performance by minimizing concurrent clashes between the parent and child generators over shared resources.

If you don't have batch ID consider enumerating your items to and use key as batch ID.

```rust
// Import necessary traits and modules from the `rayon` and `deterministic_rand` crates.
use rayon::prelude::*;
use deterministic_rand::{ distributions::Uniform, Rng, Hrng };

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
#[ cfg( not( feature = "no_std" ) ) ]
#[ cfg( feature = "determinism" ) ]
assert_eq!( got_pi, 3.1410448 );

// Print the calculated value of Pi.
println!( "PI = {got_pi}" );
```

### How to deal with STD non-determinism

In the standard library, randomness can also be a factor; for instance, iterating over the keys of a hashmap or hashset is non-deterministic. To achieve deterministic enumeration, you can use the `deterministic_rand::IfDeterminismIteratorExt` extension for iterators. By applying `if_determinism_then_sort` or `if_determinism_then_sort_by` before processing the keys, you can ensure a consistent order. The `if_determinism_then_sort_by` method acts as a no-op (no operation) when determinism is off, but it performs sorting when the determinism feature is on.

```rust
// Import the necessary modules from the standard library and the `deterministic_rand` crate.
use std::collections::HashMap;
use deterministic_rand::IfDeterminismIteratorExt;

// Create a HashMap with three key-value pairs.
let map: HashMap<_, _> = HashMap::from_iter( [ ( 1, "first" ), ( 2, "second" ), ( 3, "third" ) ] );

// Convert the HashMap into an iterator, apply deterministic sorting to the keys,
// and then map each (key, value) pair to just the value.
let keys: Vec< _ > = map
.into_iter()
.if_determinism_then_sort_by( | ( a, _ ), ( b, _ ) | a.cmp( &b ) )
.map( | e | e.1 )
.collect();

// If the 'determinism' feature is enabled, assert that the sorted keys match the expected order.
// This is a conditional compilation check that ensures the code block is compiled and run only
// if the 'determinism' feature is enabled.
#[ cfg( feature = "determinism" ) ]
assert_eq!( keys, vec![ "first", "second", "third" ] );
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
