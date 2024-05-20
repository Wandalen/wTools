<!-- {{# generate.module_header{} #}} -->

# Module :: mem_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_mem_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_mem_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/mem_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/mem_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fmem_tools%2Fexamples%2Fmem_tools_trivial.rs,RUN_POSTFIX=--example%20mem_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of tools to manipulate memory.

Performant size / pointer / region / data comparing.

### Basic use-case

<!-- {{# generate.module{} #}} -->

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
cd examples/mem_tools_trivial
cargo run
```

# Sample

[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=sample%2Frust%2Fmeta_tools_trivial,SAMPLE_FILE=.%2Fsrc%2Fmain.rs/https://github.com/Wandalen/wTools)
[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/meta_tools)
![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/meta_tools)
