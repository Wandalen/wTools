# Module :: is_slice
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleIsSlicePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleIsSlicePush.yml) [![docs.rs](https://img.shields.io/docsrs/is_slice?color=e3e8f0&logo=docs.rs)](https://docs.rs/is_slice) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=discuss)](https://discord.gg/JwTG6d2b)

Macro to answer the question: is it a slice?

### Sample

```rust
use is_slice::*;

fn main()
{

  dbg!( is_slice!( Box::new( true ) ) );
  // < is_slice!(Box :: new(true)) = false
  dbg!( is_slice!( &[ 1, 2, 3 ] ) );
  // < is_slice!(& [1, 2, 3]) = false
  dbg!( is_slice!( &[ 1, 2, 3 ][ .. ] ) );
  // < is_slice!(& [1, 2, 3] [..]) = true

}
```

### To add to your project

```sh
cargo add is_slice
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/is_slice_trivial
cargo run
```
