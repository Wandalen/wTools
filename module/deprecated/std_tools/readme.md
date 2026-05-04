# DEPRECATED: std_tools
<!-- {{# generate.module_header{} #}} -->

# Module :: std_tools
<!--{ generate.module_header.start() }-->
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleStdToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleStdToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/std_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/std_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

### Basic Use Case :: implements

<!-- {{# generate.module{} #}} -->

```rust ignore
use std_tools::prelude::*;

fn main()
{
  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
}
```

### Basic Use Case :: type constructors

In Rust, you often need to wrap a given type into a new one.
The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.

Macro [types](https://docs.rs/type_constructor/latest/type_constructor/types/macro.types.html) is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once:

<!-- {{# generate.module{} #}} -->

```rust ignore
use std_tools::prelude::*;

// types!
// {
//
//   single MySingle : f32;
//   single SingleWithParametrized : std::sync::Arc< T : Copy >;
//   single SingleWithParameter : < T >;
//
//   pair MyPair : f32;
//   pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
//   pair PairWithParameter : < T1, T2 >;
//
//   pair MyHomoPair : f32;
//   pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
//   pair HomoPairWithParameter : < T >;
//
//   many MyMany : f32;
//   many ManyWithParametrized : std::sync::Arc< T : Copy >;
//   many ManyWithParameter : < T >;
//
// }
```

### To add to your project

```sh
cargo add std_tools
```
