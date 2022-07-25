<!-- {{# generate.module_header{} #}} -->

# Module :: mem_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleMemToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleMemToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/mem_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/mem_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=&message=try&color=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fmem_tools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20mem_tools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Collection of tools to manipulate memory.

Performant size / pointer / region / data comparing.

### Sample

<!-- {{# generate.module_sample{} #}} -->

```rust

use mem_tools as mem;

// Are two pointers are the same, not taking into accoint type.
// Unlike `std::ptr::eq()` does not require arguments to have the same type.
let src1 = ( 1, );
let src2 = ( 1, );
assert!( !mem::same_ptr( &src1, &src2 ) );

// Are two pointers points on data of the same size.
let src1 = "abc";
let src2 = "cba";
assert!( mem::same_size( src1, src2 ) );

// Are two pointers points on the same region, ie same size and same pointer.
// Does not require arguments to have the same type.
let src1 = "abc";
let src2 = "abc";
assert!( mem::same_region( src1, src2 ) );

```

### To add to your project

```sh
cargo add mem_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/mem_tools_trivial
cargo run
```
