<!-- {{# generate.module_header{} #}} -->

# Module :: proc_macro_tools

[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/proc_macro_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/proc_macro_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Tools for writing procedural macros.

### Basic use-case

<!-- {{# generate.module_sample{} #}} -->

```rust
use proc_macro_tools::*;
use proc_macro_tools::dependency::*;
use quote::quote;

fn main()
{
  let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  let got = type_parameters( &tree_type, 0..=2 );
  got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
  // < i8
  // < i16
  // < i32
}
```

### To add to your project

```sh
cargo add proc_macro_tools
```
