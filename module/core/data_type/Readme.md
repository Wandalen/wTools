<!-- {{# generate.module_header{} #}} -->

# Module :: data_type
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_data_type_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_data_type_push.yml) [![docs.rs](https://img.shields.io/docsrs/data_type?color=e3e8f0&logo=docs.rs)](https://docs.rs/data_type) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fdata_type%2Fexamples%2Fdata_type_trivial.rs,RUN_POSTFIX=--example%20data_type_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of primal data types.

### Basic Use Case :: type constructors

In Rust, you often need to wrap a given type into a new one.
The role of the orphan rules in particular is basically to prevent you from implementing external traits for external types.
To overcome the restriction developer usually wrap the external type into a tuple introducing a new type.
Type constructor does exactly that and auto-implement traits From, Into, Deref and few more for the constructed type.

Macro [types](https://docs.rs/type_constructor/latest/type_constructor/types/macro.types.html) is responsible for generating code for Single, Pair, Homopair, Many. Each type constructor has its own keyword for that, but Pair and Homopair use the same keyword difference in a number of constituent types. It is possible to define all types at once:

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( feature = "enabled" ) ]
{
  use data_type::prelude::*;

  // qqq : xxx : write please

}
```

### Basic Use Case :: make - variadic constructor

Implement traits [From_0], [From1] up to MakeN to provide the interface to construct your structure with a different set of arguments.
In this example structure, Struct1 could be constructed either without arguments, with a single argument, or with two arguments.
- Constructor without arguments fills fields with zero.
- Constructor with a single argument sets both fields to the value of the argument.
- Constructor with 2 arguments set individual values of each field.

<!-- {{# generate.module{} #}} -->

```rust
#[ cfg( feature = "make" ) ]
{
  use type_constructor::prelude::*;

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

  impl From1< i32 > for Struct1
  {
    fn from1( val : i32 ) -> Self
    {
      Self { a : val, b : val }
    }
  }

  impl From2< i32, i32 > for Struct1
  {
    fn from2( val1 : i32, val2 : i32 ) -> Self
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
}
```

### To add to your project

``` shell
cargo add data_type
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/type_constructor_multiple
cargo run
```
