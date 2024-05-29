<!-- {{# generate.module_header{} #}} -->

# Module :: meta_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_meta_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_meta_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/meta_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/meta_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fmeta_tools%2Fexamples%2Fmeta_tools_trivial.rs,RUN_POSTFIX=--example%20meta_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose meta tools.

### Basic use-case  :: variadic constructor of collections

Among other useful meta tools the module aggregates variadic constructors of collections. For example macro `hmap!` for constructing a hash map.

<!-- {{# generate.module{} #}} -->

```rust
use meta_tools::*;

let meta_map = hmap! { 3 => 13 };
let mut std_map = std::collections::HashMap::new();
std_map.insert( 3, 13 );
assert_eq!( meta_map, std_map );
```

### Basic Use Case :: function-style call

Apply a macro for each element of a list.

Macro `for_each` may be called either in function-style way or in map-style way.
Pass name of macro to apply to elements as the first arguments and elements after the macro name.
Use comma as delimiter.

<!-- {{# generate.module{} #}} -->

```rust
use meta_tools::*;
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
cd examples/meta_tools_trivial
cargo run
```
