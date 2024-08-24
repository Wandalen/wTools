<!-- {{# generate.module_header{} #}} -->

# Module :: collection_tools
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_collection_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_collection_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/collection_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/collection_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fcollection_tools%2Fexamples%2Fcollection_tools_trivial.rs,RUN_POSTFIX=--example%20collection_tools_trivial/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Collection of general purpose tools to manipulate collections( containers like Vec/HashMap/HashSet... ).

### Basic Use Case :: Variadic Constructors for Collections

This module encompasses a suite of meta-tools designed to enhance Rust's collection handling, most notably through the inclusion of variadic constructors. A prime example is the `hmap!` macro, which facilitates the ergonomic construction of `HashMap` instances. These constructors allow for the intuitive and concise initialization of collections, mirroring the simplicity found in other programming languages.

Consider the following example, which demonstrates the use of the `hmap!` macro to effortlessly create a `HashMap`:

```rust
# #[ cfg( all( feature = "enabled", feature = "collection_constructors" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {
use collection_tools::*;

let meta_map = hmap! { 3 => 13 };

// it is identical to `hashbrown::HashMap` if `use_alloc` feature is on, otherwise `std::collections::HashMap`
let mut std_map = collection_tools::HashMap::new();

std_map.insert( 3, 13 );
assert_eq!( meta_map, std_map );
# }
```

Another example, this time, `into_bset!`, providing you a `BTreeSet`:

```rust
# #[ cfg( all( feature = "enabled", feature = "collection_constructors" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {
use collection_tools::*;

let meta_set = bset! { 3, 13 };

// this `BTreeSet` is just a reexport from `alloc`,
// so it can be used in the same places as `alloc/std::BTreeSet`
let mut std_set = collection_tools::BTreeSet::new();

std_set.insert( 13 );
std_set.insert( 3 );
assert_eq!( meta_set, std_set );
# }
```

Another example with `llist!`:

```rust
# #[ cfg( all( feature = "enabled", feature = "collection_constructors" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {
use collection_tools::*;

let meta_list : LinkedList< i32 > = llist! { 3, 13 };

// this `LinkedList` is just a reexport from `alloc`,
// so it can be used in the same places as `alloc/std::LinkedList`
let mut meta_list = collection_tools::LinkedList::new();

meta_list.push_front( 13 );
meta_list.push_front( 3 );
assert_eq!( meta_list, meta_list );
# }
```

### Basic Use Case :: `no_std` `HashSet` / `HashMap`

When implementing a `no_std` environment with the `use_alloc` feature in your Rust project, you'll encounter a challenge: collections like `Vec` are imported differently depending on the availability of the `std` library. Moreover, to use data structures such as `HashSet` or `HashMap` in a `no_std` context, it's necessary to depend on third-party crates, as these are not provided by the `alloc` crate directly. This crate aims to simplify the process of designing Rust libraries or applications that require these collections in a `no_std` environment, offering a more streamlined approach to working with dynamic data structures without the standard library.

You can do

<!-- // zzz : aaa : rid of `#[ cfg( not( feature = "use_alloc" ) ) ]` -- Rid of by not relying on std -->
```rust
# #[ cfg( feature = "enabled" ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {
use collection_tools::HashSet;

let mut vec : HashSet< i32 > = HashSet::new();
vec.insert( 1 );
assert_eq!( vec.contains( &1 ), true );
# }
```

Instead of

<details>
<summary>Click to see</summary>

```rust
# #[ cfg( all( feature = "enabled", feature = "collection_std" ) ) ]
# #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
# {

#[ cfg( feature = "use_alloc" ) ]
use hashbrown::HashSet; // a `no_std` replacement for `HashSet`
#[ cfg( not( feature = "no_std" ) ) ]
use std::collections::HashSet;

let mut vec : HashSet< i32 > = HashSet::new();
vec.insert( 1 );
assert_eq!( vec.contains( &1 ), true );

# }
```

</details>

### Basic Use Case :: `no_std` `HashSet` / `HashMap`

The crate has two classes of macros: strict macros (the one we covered), which require that all collection members are of the same type; and more "relaxed" macros, that use under the hood `Into` trait to cast to a certain type. They can be accessed by prepending `into_` to name of a macro (`into_vec`, `into_bmap`, etc).

While strict macros require you to have all members of the same type, more relaxed macros often require you to specify the desired type. So there's no a clear winner. Choose the right one for each situation separately.

For example:
```rust
# #[ cfg( all( feature = "enabled", feature = "collection_into_constructors", any( not( feature = "no_std" ), feature = "use_alloc" ) ) ) ]
# {
use std::borrow::Cow;
let vec : Vec< String > = collection_tools::into_vec!( "&str", "String".to_string(), Cow::from( "Cow" ) );
# }
```

Each strict macro has its relaxed counterpart.

### Collections being used

So what's the deal with `collection_tools::<collection>`?

Nothing really fancy. We just reuse collections from `alloc` (same as `std`).

But not all collections are available in `alloc` crate. For now, the exceptions are `HashMap` and `HashSet`. This leads to the fact that we can't use them in `no_std` environment. How did we solve this? By using those collections from `hashbrown` crate whenever `no_std` feature is enabled. You can found more details on origin of a collection on its documentation page.

### MORE Examples

If you are feeling confused about the syntax you should use for a macro, you can visit its documentation. It is saturated with different examples, so hopefully you'll not be stuck.

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
