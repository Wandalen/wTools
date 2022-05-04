# module::typing_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

Collection of general purpose tools for type checking.

### Sample

```rust
use typing_tools::*;

fn main()
{
  let src = Box::new( true );
  assert_eq!( implements!( src => Copy ), false );
  assert_eq!( implements!( src => Clone ), true );
}
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
cd sample/rust/typing_tools_trivial
cargo run
```
