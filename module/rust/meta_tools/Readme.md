# Module :: meta_tools [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ToolsRustPush.yml) [![stable](https://img.shields.io/badge/stability-stable-brightgreen.svg)](https://github.com/emersion/stability-badges#stable)

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
