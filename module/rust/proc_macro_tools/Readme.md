# Module :: proc_macro_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleProcMacroToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/proc_macro_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/proc_macro_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Tools for writing procedural macroses.

### Sample

```rust
use proc_macro_tools::*;

let code = qt!( core::option::Option< i8, i16, i32, i64 > );
let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
let got = type_parameters( &tree_type, 0..=2 );
got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
// < i8
// < i16
// < i32
```

### To add to your project

```sh
cargo add proc_macro_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/proc_macro_tools_trivial
cargo run
```

