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
//! The `hmap!` macro significantly simplifies the syntax required to instantiate and populate
//! a `HashMap`, making your code cleaner and more concise. This is particularly useful in cases
//! where you need to define a map with a known set of key-value pairs upfront.

#[ cfg( not( all
(
//   not( feature = "use_alloc" ) ) ],
  all( feature = "enabled", feature = "collection_constructors" ),
  any( not( feature = "no_std" ), feature = "use_alloc" )
)))]
fn main(){}

// zzz : aaa : rid of `#[ cfg( not( feature = "use_alloc" ) ) ]` -- Rid of by not relying on std
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( all( feature = "enabled", feature = "collection_constructors" ) ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn main()
{
  use collection_tools::*;
  let map = hmap! { 3 => 13 };
  let mut expected = collection_tools::HashMap::new();
  expected.insert( 3, 13 );
  assert_eq!( map, expected );
}
