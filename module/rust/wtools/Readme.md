# Module :: wtools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/wtools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fwtools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20wtools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

### Sample :: implements

```rust,editable
#[ cfg( feature = "typing_default" ) ]
{
  use wtools::prelude::*;
  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
}
```

### Sample :: type constructors

In Rust, you often need to wrap a given type into a new one.
The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.

Macro [types](https://docs.rs/type_constructor/latest/type_constructor/types/macro.types.html) is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once.

```rust,editable
#[ cfg( feature = "dt_default" ) ]
{
  use wtools::prelude::*;

  types!
  {

    single MySingle : f32;
    single SingleWithParametrized : std::sync::Arc< T : Copy >;
    single SingleWithParameter : < T >;

    pair MyPair : f32;
    pair PairWithParametrized : std::sync::Arc< T1 : Copy >, std::sync::Arc< T2 : Copy >;
    pair PairWithParameter : < T1, T2 >;

    pair MyHomoPair : f32;
    pair HomoPairWithParametrized : std::sync::Arc< T : Copy >;
    pair HomoPairWithParameter : < T >;

    many MyMany : f32;
    many ManyWithParametrized : std::sync::Arc< T : Copy >;
    many ManyWithParameter : < T >;

  }
}
```

### Sample :: make - variadic constructor

Implement traits [Make0], [Make1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
- Constructor without arguments fills fields with zero.
- Constructor with a single argument sets both fields to the value of the argument.
- Constructor with 2 arguments set individual values of each field.

```rust
#[ cfg( feature = "dt_default" ) ]
{
  use wtools::prelude::*;

  #[ derive( Debug, PartialEq ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
  }

  impl Make0 for Struct1
  {
    fn make_0() -> Self
    {
      Self { a : 0, b : 0 }
    }
  }

  impl Make1< i32 > for Struct1
  {
    fn make_1( val : i32 ) -> Self
    {
      Self { a : val, b : val }
    }
  }

  impl Make2< i32, i32 > for Struct1
  {
    fn make_2( val1 : i32, val2 : i32 ) -> Self
    {
      Self { a : val1, b : val2 }
    }
  }

  let got : Struct1 = make!();
  let exp = Struct1{ a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : Struct1 = make!( 13 );
  let exp = Struct1{ a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : Struct1 = make!( 1, 3 );
  let exp = Struct1{ a : 1, b : 3 };
  assert_eq!( got, exp );
}
```

### To add to your project

```sh
cargo add wtools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wtools_trivial_sample
cargo run
```
