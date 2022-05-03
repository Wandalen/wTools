# module::instance_of [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case enity is a function and trat is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

This is alias for [module::implements](https://github.com/Wandalen/wTools/tree/master/module/rust/implements).

### Sample

```rust
use instance_of::*;

dbg!( instance_of!( 13_i32 => Copy ) );
// < instance_of!( 13_i32 => Copy ) : true
dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
// < instance_of!( 13_i32 => Copy ) : false
```

### To add to your project

```
cargo add implements
```

### Try out from the repository

```
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/implements_trivial
cargo run
```
