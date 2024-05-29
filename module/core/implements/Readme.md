<!-- {{# generate.module_header{} #}} -->

# Module :: implements
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_implements_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_implements_push.yml) [![docs.rs](https://img.shields.io/docsrs/implements?color=e3e8f0&logo=docs.rs)](https://docs.rs/implements) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fimplements%2Fexamples%2Fimplements_trivial.rs,RUN_POSTFIX=--example%20implements_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case entity is a function and trait is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

### Basic use-case

<!-- {{# generate.module{} #}} -->

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
cargo run --example implements_trivial
```
