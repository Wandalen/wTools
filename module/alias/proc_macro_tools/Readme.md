<!-- {{# generate.module_header{} #}} -->

# Module :: proc_macro_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_proc_macro_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_proc_macro_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/proc_macro_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/proc_macro_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Falias%2Fproc_macro_tools%2Fexamples%2Fproc_macro_tools_trivial.rs,RUN_POSTFIX=--example%20proc_macro_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Tools for writing procedural macros.

### Basic use-case

<!-- {{# generate.module{} #}} -->

```rust ignore
use proc_macro_tools::*;

fn main()
{
  let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = type_parameters( &tree_type, 0..=2 );
  got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  // < i8
  // < i16
  // < i32
}
```

### To add to your project

```sh
cargo add proc_macro_tools
```
