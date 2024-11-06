<!-- {{# generate.module_header{} #}} -->

# Module :: `interval_adapter`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_interval_adapter_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_interval_adapter_push.yml) [![docs.rs](https://img.shields.io/docsrs/interval_adapter?color=e3e8f0&logo=docs.rs)](https://docs.rs/interval_adapter) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Finterval_adapter%2Fexamples%2Finterval_adapter_trivial.rs,RUN_POSTFIX=--example%20interval_adapter_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Integer interval adapter for both Range and `RangeInclusive`.

Let's assume you have a function which should accept Interval. But you don't want to limit caller of the function to either half-open interval `core::ops::Range` or closed one `core::ops::RangeInclusive` you want allow to use anyone of iterable interval. To make that work smoothly use `IterableInterval`. Both `core::ops::Range` and `core::ops::RangeInclusive` implement the trait, also it's possible to work with non-iterable intervals, like ( -Infinity .. +Infinity ).

### Basic use-case

```rust

use interval_adapter::IterableInterval;

fn f1( interval : impl IterableInterval )
{
  for i in interval
  {
    println!( "{i}" );
  }
}

// Calling the function either with
// half-open interval `core::ops::Range`.
f1( 0..=3 );
// Or closed one `core::ops::RangeInclusive`.
f1( 0..4 );

```

### More flexibility

If you need more flexibility in defining intervals, you can convert a tuple of endpoints to an interval.

```rust

use interval_adapter::{ IterableInterval, IntoInterval, Bound };

fn f1( interval : impl IterableInterval )
{
  for i in interval
  {
    println!( "{i}" );
  }
}

// Calling the function either with
// half-open interval `core::ops::Range`.
f1( 0..=3 );
// Or closed one `core::ops::RangeInclusive`.
f1( 0..4 );
// Alternatively you construct your custom interval from a tuple.
f1( ( 0, 3 ).into_interval() );
f1( ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval() );
// All the calls to the function `f1`` perform the same task,
// and the output is exactly identical.

```

### Non-iterable intervals

You may also use the crate to specify non-iterable intervals. Non-iterable intervals have either one or several unbound endpoints. For example, interval `core::ops::RangeFull` has no bounds and represents the range from minus infinity to plus infinity.

```rust

use interval_adapter::{ NonIterableInterval, IntoInterval, Bound };

fn f1( interval : impl NonIterableInterval )
{
  println!( "Do something with this {:?} .. {:?} interval", interval.left(), interval.right() );
}

// Iterable/bound interval from tuple.
f1( ( Bound::Included( 0 ), Bound::Included( 3 ) ).into_interval() );
// Non-iterable/unbound interval from tuple.
f1( ( Bound::Included( 0 ), Bound::Unbounded ).into_interval() );
// Non-iterable/unbound interval from `core::ops::RangeFrom`.
f1( 0.. );
// Non-iterable/unbound interval from `core::ops::RangeFull`
// what is ( -Infinity .. +Infinity ).
f1( .. );

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
