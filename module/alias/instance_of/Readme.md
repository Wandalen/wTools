<!-- {{# generate.module_header{} #}} -->

# Module :: instance_of
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) |[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleInstanceOfPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleInstanceOfPush.yml)[![docs.rs](https://img.shields.io/docsrs/instance_of?color=e3e8f0&logo=docs.rs)](https://docs.rs/instance_of)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Finstance_of_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20instance_of_trivial/https://github.com/Wandalen/wTools)
<!--{ generate.module_header.end }-->

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case entity is a function and trait is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

This is alias for [module::implements](https://github.com/Wandalen/wTools/tree/master/module/core/implements).

### Basic use-case

<!-- {{# generate.module{} #}} -->

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
cd examples/implements_trivial
cargo run
```
