# Module :: typing_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTypingToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTypingToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/typing_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/typing_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

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
