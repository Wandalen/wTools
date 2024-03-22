<!-- {{# generate.module_header{} #}} -->

# Module :: collection_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_collection_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_collection_tools_push.yml)[![docs.rs](https://img.shields.io/docsrs/collection_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/collection_tools)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fcollection_tools_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20collection_tools_trivial/https://github.com/Wandalen/wTools)
[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools to manipulate collections( containers like Vec/HashMap/HashSet... ).

### Basic Use Case :: Variadic Constructors for Collections

This module encompasses a suite of meta-tools designed to enhance Rust's collection handling, most notably through the inclusion of variadic constructors. A prime example is the `hmap!` macro, which facilitates the ergonomic construction of `HashMap` instances. These constructors allow for the intuitive and concise initialization of collections, mirroring the simplicity found in other programming languages.

Consider the following example, which demonstrates the use of the `hmap!` macro to effortlessly create a `HashMap`:

<!-- // zzz : qqq : rid off `#[ cfg( not( feature = "use_alloc" ) ) ]` -->
```rust
# #[ cfg( not( feature = "use_alloc" ) ) ]
# #[ cfg( all( feature = "enabled", feature = "collection_constructors" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {

use collection_tools::*;

let meta_map = hmap! { 3 => 13 };
let mut std_map = std::collections::HashMap::new();
std_map.insert( 3, 13 );
assert_eq!( meta_map, std_map );

# }
```

### Basic Use Case :: `no_std` `HashSet` / `HashMap`

When implementing a `no_std` environment with the `use_alloc` feature in your Rust project, you'll encounter a challenge: collections like `Vec` are imported differently depending on the availability of the `std` library. Moreover, to use data structures such as `HashSet` or `HashMap` in a `no_std` context, it's necessary to depend on third-party crates, as these are not provided by the `alloc` crate directly. This crate aims to simplify the process of designing Rust libraries or applications that require these collections in a `no_std` environment, offering a more streamlined approach to working with dynamic data structures without the standard library.

You can do

<!-- // zzz : qqq : rid off `#[ cfg( not( feature = "use_alloc" ) ) ]` -->
```rust
# #[ cfg( not( feature = "use_alloc" ) ) ]
# #[ cfg( all( feature = "enabled", feature = "collection_std" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {

use collection_tools::Vec;

let mut map : Vec< i32 > = Vec::new();
map.push( 1 );
assert_eq!( map.first().unwrap().clone(), 1 );

# }
```

Instead of

<details>
<summary>The code above will be expanded to this</summary>

```rust
#[ cfg( feature = "use_alloc" ) ]
extern crate alloc;
#[ cfg( feature = "use_alloc" ) ]
use alloc::vec::Vec;
#[ cfg( not( feature = "no_std" ) ) ]
use std::vec::Vec;

let mut collection : Vec< i32 > = Vec::new();
collection.push( 1 );
assert_eq!( collection.first().unwrap().clone(), 1 );
```

</details>

### To add to your project

```sh
cargo add collection_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/container_tools_trivial
cargo run
```
