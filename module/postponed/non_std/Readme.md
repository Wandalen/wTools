<!-- {{# generate.module_header{} #}} -->

# Module :: non_std
<!--{ generate.module_header.start() }-->
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleNonStdPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleNonStdPush.yml) [![docs.rs](https://img.shields.io/docsrs/non_std?color=e3e8f0&logo=docs.rs)](https://docs.rs/non_std) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->


Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

### Basic Use Case :: implements

<!-- {{# generate.module{} #}} -->

```rust ignore
use non_std::prelude::*;

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
use non_std::prelude::*;

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
```

### Basic Use Case :: make - variadic constructor

Implement traits [From_0], [From_1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
- Constructor without arguments fills fields with zero.
- Constructor with a single argument sets both fields to the value of the argument.
- Constructor with 2 arguments set individual values of each field.

<!-- {{# generate.module{} #}} -->

```rust ignore
use non_std::prelude::*;

#[ derive( Debug, PartialEq ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

impl From_0 for Struct1
{
  fn from_0() -> Self
  {
    Self { a : 0, b : 0 }
  }
}

impl From_1< i32 > for Struct1
{
  fn from_1( val : i32 ) -> Self
  {
    Self { a : val, b : val }
  }
}

impl From_2< i32, i32 > for Struct1
{
  fn from_2( val1 : i32, val2 : i32 ) -> Self
  {
    Self { a : val1, b : val2 }
  }
}

let got : Struct1 = from!();
let exp = Struct1{ a : 0, b : 0 };
assert_eq!( got, exp );

let got : Struct1 = from!( 13 );
let exp = Struct1{ a : 13, b : 13 };
assert_eq!( got, exp );

let got : Struct1 = from!( 1, 3 );
let exp = Struct1{ a : 1, b : 3 };
assert_eq!( got, exp );
```

### To add to your project

```sh
cargo add non_std
```

