<!-- {{# generate.module_header{} #}} -->

# Module :: implements
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleImplementsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleImplementsPush.yml) [![docs.rs](https://img.shields.io/docsrs/implements?color=e3e8f0&logo=docs.rs)](https://docs.rs/implements) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fimplements_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20implements_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case entity is a function and trait is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

### Basic use-case.

<!-- {{# generate.module_sample{} #}} -->

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
cd examples/implements_trivial
cargo run
```
