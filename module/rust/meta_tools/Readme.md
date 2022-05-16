# Module :: meta_tools [![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/MetaToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/MetaToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/meta_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/meta_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Collection of general purpose meta tools.

### Sample

```rust
use meta_tools::*;

fn main()
{
  let meta_map = hmap! { 3 => 13 };
  let mut std_map = std::collections::HashMap::new();
  std_map.insert( 3, 13 );
  assert_eq!( meta_map, std_map );
}
```

<!-- qqq for Rust dev : please write --> <!-- aaa : done -->

### To add to your project

```sh
cargo add meta_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/meta_tools_trivial
cargo run
```
