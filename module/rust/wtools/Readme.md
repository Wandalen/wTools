# module::wtools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/RustPublish.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/RustPublish.yml)

Collection of general purpose tools for solving problems. Fundamentally extend the language without spoiling, so may be used solely or in conjunction with another module of such kind.

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

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/wtools_trivial
cargo run
```
