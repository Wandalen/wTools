<!-- {{# generate.module_header{} #}} -->

# Module :: inspect_type
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_inspect_type_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_inspect_type_push.yml) [![docs.rs](https://img.shields.io/docsrs/inspect_type?color=e3e8f0&logo=docs.rs)](https://docs.rs/inspect_type) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Finspect_type%2Fexamples%2Finspect_type_trivial.rs,RUN_POSTFIX=--example%20inspect_type_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Diagnostic-purpose tools to inspect type of a variable and its size.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust
// #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
pub use inspect_type::*;

#[ cfg( feature = "nightly" ) ]
{
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  inspect_type_of!( &[ 1, 2, 3 ] );
  // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
}

```

### To add to your project

```sh
cargo add inspect_type
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example inspect_type_trivial
```
