<!-- {{# generate.module_header{} #}} -->

# Module :: variadic_from
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_variadic_from_push.yml)[![docs.rs](https://img.shields.io/docsrs/variadic_from?color=e3e8f0&logo=docs.rs)](https://docs.rs/variadic_from)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fvariadic_from_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20variadic_from_trivial/https://github.com/Wandalen/wTools)
<!--{ generate.module_header.end }-->

Variadic from

### Basic use-case.

<!-- {{# generate.module{} #}} -->

```rust
use variadic_from::exposed::*;

fn main()
{
  #[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
  }

  let got : StructNamedFields = From::from( ( 13, 14 ) );
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

  let got : StructNamedFields = from!( 13, 14 );
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

  let got : StructNamedFields = ( 13, 14 ).to();
  let exp = StructNamedFields{ a : 13, b : 14 };
  assert_eq!( got, exp );

}
```

### To add to your project

```sh
cargo add variadic_from
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example variadic_from_trivial
```
