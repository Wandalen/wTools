<!-- {{# generate.module_header{} #}} -->

# Module :: former

[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleFormerPush.yml) [![docs.rs](https://img.shields.io/docsrs/former?color=e3e8f0&logo=docs.rs)](https://docs.rs/former) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fformer_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20former_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

A flexible and extensible implementation of the builder pattern.

It offers specialized subformers for common Rust collections like `Vec`, `HashMap`, and `HashSet`, enabling the construction of complex data structures in a fluent and intuitive manner.

## How Former Works

- **Trait Derivation** : By deriving `Former` on a struct, you automatically generate builder methods for each field.
- **Fluent Interface** : Each field's builder method allows for setting the value of that field and returns a mutable reference to the builder,
  enabling method chaining.
- **Optional Fields** : Optional fields can be easily handled without needing to explicitly set them to `None`.
- **Finalization** : The `.form()` method finalizes the building process and returns the constructed struct instance.

This approach abstracts away the need for manually implementing a builder for each struct, making code more readable and maintainable.

### Basic use-case

The provided code snippet illustrates a basic use-case of the Former crate in Rust, which is used to apply the builder pattern for structured and flexible object creation. Below is a detailed explanation of each part of the markdown chapter, aimed at clarifying how the Former trait simplifies struct instantiation.

```rust
use former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub struct UserProfile
{
  age : i32,
  username : String,
  bio_optional : Option<String>, // Fields could be optional
}

let profile = UserProfile::former()
.age(30)
.username("JohnDoe".to_string())
.bio_optional("Software Developer".to_string()) // Optionally provide a bio
.form();

dbg!( &profile );
// Expected output:
// &profile = UserProfile {
//   age: 30,
//   username: "JohnDoe",
//   bio_optional: Some("Software Developer"),
// }

```

### Concept of subformer

Subformers are specialized builders used within the `Former` framework to construct nested or collection-based data structures like vectors, hash maps, and hash sets. They simplify the process of adding elements to these structures by providing a fluent interface that can be seamlessly integrated into the overall builder pattern of a parent struct. This approach allows for clean and intuitive initialization of complex data structures, enhancing code readability and maintainability.

### Example: Building a Vector

The following example illustrates how to use a `VectorSubformer` to construct a `Vec` field within a struct. The subformer enables adding elements to the vector with a fluent interface, streamlining the process of populating collection fields within structs.

```rust
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithVec
{
  #[ subformer( former::runtime::VectorSubformer ) ]
  vec : Vec< &'static str >,
}

let instance = StructWithVec::former()
.vec()
  .push( "apple" )
  .push( "banana" )
  .end()
.form();

assert_eq!( instance, StructWithVec { vec: vec![ "apple", "banana" ] } );
```

### Example: Building a Hashmap

This example demonstrates the use of a `HashMapSubformer` to build a hash map within a struct. The subformer provides a concise way to insert key-value pairs into the map, making it easier to manage and construct hash map fields.

```rust
use test_tools::exposed::*;

#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithMap
{
  #[ subformer( former::runtime::HashMapSubformer ) ]
  map : std::collections::HashMap< &'static str, &'static str >,
}

let struct1 = StructWithMap::former()
.map()
  .insert( "a", "b" )
  .insert( "c", "d" )
  .end()
.form()
;
assert_eq!( struct1, StructWithMap { map : hmap!{ "a" => "b", "c" => "d" } } );
```

### Example: Building a Hashset

In the following example, a `HashSetSubformer` is utilized to construct a hash set within a struct. This illustrates the convenience of adding elements to a set using the builder pattern facilitated by subformers.

```rust
use test_tools::exposed::*;

#[ derive( Debug, PartialEq, former::Former ) ]
pub struct StructWithSet
{
  #[ subformer( former::runtime::HashSetSubformer ) ]
  set : std::collections::HashSet< &'static str >,
}

let instance = StructWithSet::former()
.set()
  .insert("apple")
  .insert("banana")
  .end()
.form();

assert_eq!(instance, StructWithSet { set : hset![ "apple", "banana" ] });
```

### To add to your project

```sh
cargo add former
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/former_trivial
cargo run
```
