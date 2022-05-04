# module::inspect_type [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Diagnostic-purpose tools to inspect type of a variable and its size.

### Sample

```rust
#![ feature( type_name_of_val ) ]

pub use inspect_type::*;

fn main()
{
  inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
  // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
  inspect_type_of!( &[ 1, 2, 3 ] );
  // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
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
