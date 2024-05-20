# Module :: derive_tools

<!--{ generate.module_header{} }-->
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_derive_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_derive_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/derive_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/derive_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fderive_tools%2Fexamples%2Fderive_tools_trivial.rs,RUN_POSTFIX=--example%20derive_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
# #[ cfg( all( feature = "derive_from", feature = "derive_inner_from", feature = "derive_display", feature = "derive_from_str" ) ) ]
{
  use derive_tools::*;

  #[ derive( From, InnerFrom, Display, FromStr, PartialEq, Debug ) ]
  #[ display( "{a}-{b}" ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
  }

  // derived InnerFrom
  let src = Struct1 { a : 1, b : 3 };
  let got : ( i32, i32 ) = src.into();
  let exp = ( 1, 3 );
  assert_eq!( got, exp );

  // derived From
  let src : Struct1 = ( 1, 3 ).into();
  let got : ( i32, i32 ) = src.into();
  let exp = ( 1, 3 );
  assert_eq!( got, exp );

  // derived Display
  let src = Struct1 { a : 1, b : 3 };
  let got = format!( "{}", src );
  let exp = "1-3";
  println!( "{}", got );
  assert_eq!( got, exp );

  // derived FromStr
  use std::str::FromStr;
  let src = Struct1::from_str( "1-3" );
  let exp = Ok( Struct1 { a : 1, b : 3 } );
  assert_eq!( src, exp );

}
```

### To add to your project

```sh
cargo add derive_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/derive_tools_trivial
cargo run
```
