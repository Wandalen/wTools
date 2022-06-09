# Module :: graphs_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml/badge.svg?branch=alpha)](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml) [![docs.rs](../../.https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/graphs_tools)

Graphs tools.

## Sample  :: trivial

```rust
#[ cfg( all( feature = "cell_factory", feature = "use_std" ) ) ]
{
  use graphs_tools::prelude::*;
  use wtools::prelude::*;
  let node : graphs_tools::canonical::Node = make!( 13 );
  assert_eq!( node.id(), 13.into() );
  println!( "{:?}", node );
  /* print : node::13 */
}
```

### To add to your project

```bash
cargo add graphs_tools
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/graphs_tools_trivial_sample
cargo run
```
