# Module :: wtools [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulewToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulewToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/wtools?color=e3e8f0&logo=docs.rs)](https://docs.rs/wtools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

### Sample

```rust
use wtools::*;

fn main()
{
  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );
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
cd sample/rust/wtools_trivial
cargo run
```
