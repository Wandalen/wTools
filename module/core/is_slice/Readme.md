<!-- {{# generate.module_header{} #}} -->

# Module :: is_slice
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_is_slice_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_is_slice_push.yml) [![docs.rs](https://img.shields.io/docsrs/is_slice?color=e3e8f0&logo=docs.rs)](https://docs.rs/is_slice) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fis_slice%2Fexamples%2Fis_slice_trivial.rs,RUN_POSTFIX=--example%20is_slice_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Macro to answer the question: is it a slice?

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
use is_slice::*;

dbg!( is_slice!( Box::new( true ) ) );
// < is_slice!(Box :: new(true)) = false
dbg!( is_slice!( &[ 1, 2, 3 ] ) );
// < is_slice!(& [1, 2, 3]) = false
dbg!( is_slice!( &[ 1, 2, 3 ][ .. ] ) );
// < is_slice!(& [1, 2, 3] [..]) = true
```

### To add to your project

```sh
cargo add is_slice
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/is_slice_trivial
cargo run
```
