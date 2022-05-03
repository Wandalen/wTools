# module::wTools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

It's alias of module::[wtools](https://github.com/Wandalen/wTools/tree/master/module/rust/wtools).

### Sample

``` rust test
use wtools::*;

fn main()
{

  println!( "implements!( 13_i32 => Copy ) : {}", implements!( 13_i32 => Copy ) );
  println!( "implements!( Box::new( 13_i32 ) => Copy ) : {}", implements!( Box::new( 13_i32 ) => Copy ) );

}
```

### To add to your project

``` shell
cargo add wtools
```
