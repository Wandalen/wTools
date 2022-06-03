# Module :: inspect_type
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleInspectTypePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleInspectTypePush.yml) [![docs.rs](https://img.shields.io/docsrs/inspect_type?color=e3e8f0&logo=docs.rs)](https://docs.rs/inspect_type) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=discuss)](https://discord.gg/JwTG6d2b)

Diagnostic-purpose tools to inspect type of a variable and its size.

### Sample

```rust
#![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
pub use inspect_type::*;

#[ cfg( feature = "nightly" ) ]
fn main()
{
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  inspect_type_of!( &[ 1, 2, 3 ] );
  // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
}

#[ cfg( not( feature = "nightly" ) ) ]
fn main()
{
}
```

### To add to your project

```sh
cargo add implements
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/inspect_type_trivial
cargo run
```
