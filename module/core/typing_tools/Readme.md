<!-- {{# generate.module_header{} #}} -->

# Module :: typing_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_typing_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_typing_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/typing_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/typing_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ftyping_tools%2Fexamples%2Ftyping_tools_trivial.rs,RUN_POSTFIX=--example%20typing_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools for type checking.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use typing_tools::*;

let src = Box::new( true );
assert_eq!( implements!( src => Copy ), false );
assert_eq!( implements!( src => Clone ), true );
```

<!-- # qqq : for Rust dev : please add --> <!-- aaa : done -->

### To add to your project

```sh
cargo add typing_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/typing_tools_trivial
cargo run
```
