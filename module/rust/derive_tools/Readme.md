# Module :: derive_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleDeriveToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleDeriveToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/derive_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/derive_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Collection of derives which extend STD.

### Sample

```rust
#[ cfg( all( feature = "derive_from", feature = "derive_into", feature = "derive_display", feature = "derive_from_str" ) ) ]
{
  use derive_tools::*;

  #[ derive( From, Into, Display, FromStr, PartialEq, Debug ) ]
  #[ display( "{a}-{b}" ) ]
  struct Struct1
  {
    a : i32,
    b : i32,
  }

  // derived Into
  let src = Struct1 { a : 1, b : 3 };
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
cd sample/rust/derive_tools_trivial
cargo run
```
