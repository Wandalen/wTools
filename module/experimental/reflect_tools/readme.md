# Module :: reflect_tools

Runtime type reflection system providing traits, descriptors, and utilities for dynamic inspection of Rust types. Enables introspection of type names, sizes, elements, and structure at runtime without compile-time knowledge of the concrete type.

<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_reflect_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_reflect_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/reflect_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/reflect_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Freflect_tools%2Fexamples%2Freflect_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Freflect_tools%2Fexamples%2Freflect_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use reflect_tools::*;

// Derive `From` and `InnerFrom` for custom types
#[ derive( From, InnerFrom ) ]
struct Wrapper( i32 );

let w : Wrapper = 42.into();
```

### To add to your project

```sh
cargo add reflect_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/reflect_tools_trivial
cargo run
```
