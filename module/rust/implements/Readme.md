# Module :: implements [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case entity is a function and trait is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

### Sample

``` rust
use implements::*;

dbg!( implements!( 13_i32 => Copy ) );
// < implements!( 13_i32 => Copy ) : true
dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
// < implements!( 13_i32 => Copy ) : false
```

### To add to your project

```sh
cargo add implements
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/implements_trivial
cargo run
```
