<!-- {{# generate.module_header{} #}} -->

# Module :: impls_index
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleImplsIndexPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleImplsIndexPush.yml) [![docs.rs](https://img.shields.io/docsrs/impls_index?color=e3e8f0&logo=docs.rs)](https://docs.rs/impls_index) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fimpls_index_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20impls_index_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Several of macros to put each function under a named macro to index every function in a class.

It encourages writing better code, having index of components stripped of details of implementation is very important for comprehension of the code and ability to see the big picture.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
use ::impls_index::*;

impls1!
{
  fn f1() -> i32
  {
    println!( "f1() : 13" );
    13
  }
};

index!
{
  f1,
}

assert_eq!( f1(), 13 );
/* print : f1() : 13 */
```

### To add to your project

```sh
cargo add impls_index_meta
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/impls_index_trivial_sample
cargo run
```
