# Module :: meta_tools
[![experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleMetaToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleMetaToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/meta_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/meta_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=e3e8f0&logo=discord&logoColor=e3e8f0)](https://discord.gg/JwTG6d2b)

Collection of general purpose meta tools.

### Sample  :: variadic constructor of collections

Among other useful meta tools the module aggregates variadtic constructors of collections. For example macro `hmap!` for constructing a hash map.

```rust
use meta_tools::*;

let meta_map = hmap! { 3 => 13 };
let mut std_map = std::collections::HashMap::new();
std_map.insert( 3, 13 );
assert_eq!( meta_map, std_map );
```

### Sample :: function-style call

Apply a macro for each element of a list.

Macro `for_each` may be called either in function-style way or in map-style way.
Pass name of macro to apply to elements as the first arguments and elements after the macro name.
Use comma as delimiter.

```rust
use for_each::for_each;
for_each!( dbg, "a", "b", "c" );

// generates
dbg!( "a" );
dbg!( "b" );
dbg!( "c" );
```

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
