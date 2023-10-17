<!-- {{# generate.module_header{} #}} -->

# Module :: winterval
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewIntervalPush.yml) [![docs.rs](https://img.shields.io/docsrs/winterval?color=e3e8f0&logo=docs.rs)](https://docs.rs/winterval) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwinterval_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20winterval_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Integer interval adapter for both Range and RangeInclusive.

Let's assume you have a function which should accept Interval. But you don't want to limit caller of the function to use either half-open interval `core::ops::Range` or closed one `core::ops::RangeInclusive`. To make that work smoothly use `IntervalAdapter`. Both `core::ops::Range` and `core::ops::RangeInclusive` implement the trait.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust

use interval_adapter::IntervalAdapter;

fn f1( interval : impl IntervalAdapter )
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

```

### More flexibility

<!-- {{# generate.module_sample{} #}} -->

```rust

use interval_adapter::{ IntervalAdapter, IntoInterval, Bound };

fn f1( interval : impl IntervalAdapter )
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

```

### To add to your project

```sh
cargo add interval_adaptor
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example interval_adapter_trivial
```
<!-- zzz : test that too -->
