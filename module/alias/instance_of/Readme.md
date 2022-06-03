# Module :: instance_of
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleInstanceOfPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleInstanceOfPush.yml) [![docs.rs](https://img.shields.io/docsrs/instance_of?color=e3e8f0&logo=docs.rs)](https://docs.rs/instance_of) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/JwTG6d2b)

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
