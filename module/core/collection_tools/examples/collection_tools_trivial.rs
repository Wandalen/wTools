//! # Collection Tools Crate
//!
//! This module provides utilities and macros to simplify working with Rust's collection types,
//! aiming to enhance ergonomics and reduce boilerplate code. Among other features, it includes
//! the `hmap!` macro for concise `HashMap` creation.
//!
//! ## Features
//!
//! - `hmap!`: A macro to create `HashMap` instances with minimal syntax.
//!
//! ## Example Usage
//!
//! Here's a quick example to demonstrate how you can use the `hmap!` macro provided by this crate
//! to create a `HashMap` similar to how you might initialize a map in other languages. This example
//! also shows that the resulting map is equivalent to one created using the standard `HashMap::new`
//! and `.insert()` methods.
//!
//! ```rust
//! use collection_tools::*;
//!
//! fn main()
//! {
//!   // Create a HashMap using the `hmap!` macro for more ergonomic initialization.
//!   let meta_map = hmap! { 3 => 13 };
//!
//!   // For comparison, create a HashMap using the standard approach.
//!   let mut std_map = std::collections::HashMap::new();
//!   std_map.insert( 3, 13 );
//!
//!   // Verify that the maps created by the two methods are equivalent.
//!   assert_eq!( meta_map, std_map );
//! }
//! ```
//!
//! The `hmap!` macro significantly simplifies the syntax required to instantiate and populate
//! a `HashMap`, making your code cleaner and more concise. This is particularly useful in cases
//! where you need to define a map with a known set of key-value pairs upfront.

fn main()
{
  use collection_tools::*;
  let meta_map = hmap! { 3 => 13 };
  let mut std_map = std::collections::HashMap::new();
  std_map.insert( 3, 13 );
  assert_eq!( meta_map, std_map );
}
