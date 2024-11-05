#[ allow( unused_imports ) ]
use super::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use alloc::vec::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use core::slice::{ Iter, IterMut };

/// Creates a `Vec` from a list of elements.
///
/// The `vec` macro simplifies the creation of a `Vec` with initial elements.
///
/// # Origin
///
/// This collection is reexported from `alloc`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{Vec, vec};
/// // Vec of i32
/// let vec1 = vec!( 1, 2, 3, 4, 5 );
///
/// // Vec of &str
/// let vec2 = vec!{ "hello", "world", "rust" };
///
/// // With trailing comma
/// let vec3 = vec!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key : expr ),* $( , )?`: A comma-separated list of elements to insert into the `Vec`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `Vec`.
///
/// # Returns
///
/// Returns a `Vec` containing all the specified elements. The capacity of the vector is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{Vec, vec};
/// let vec = vec!( 1, 2, 3 );
/// assert_eq!( vec[ 0 ], 1 );
/// assert_eq!( vec[ 1 ], 2 );
/// assert_eq!( vec[ 2 ], 3 );
/// ```
///
/// # Example
///
/// Creating a `Vec` of `&str` from string literals:
///
/// ```rust
/// # use collection_tools::{Vec, vec};
/// let mixed = vec!{ "value", "another value" };
/// assert_eq!( mixed[ 0 ], "value" );
/// assert_eq!( mixed[ 1 ], "another value" );
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! vec
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vec = $crate::collection::Vec::with_capacity( _cap );
    $(
      _vec.push( $key );
    )*
    _vec
  }};
}

/// Creates a `Vec` from a list of elements.
///
/// The `into_vec!` macro simplifies the creation of a `Vec` with initial elements.
/// Elements passed to the macro are automatically converted into the vector's element type
/// using `.into()`, making it convenient to use literals or values of different, but convertible types.
///
/// Note: The `into_vec!` macro utilizes the `.into()` method to convert each element into the target type
/// of the `Vec`. Therefore, the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `Vec`. Also, this means that sometimes you must specify the type of collection's items.
///
/// # Origin
///
/// This collection is reexported from `alloc`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{Vec, into_vec};
/// // Vec of i32
/// let vec1 : Vec< i32 > = into_vec!( 1, 2, 3, 4, 5 );
///
/// // Vec of String
/// let vec2 : Vec< String > = into_vec!{ "hello", "world", "rust" };
///
/// // With trailing comma
/// let vec3 : Vec< f64 > = into_vec!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key : expr ),* $( , )?`: A comma-separated list of elements to insert into the `Vec`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `Vec`.
///
/// # Returns
///
/// Returns a `Vec` containing all the specified elements. The capacity of the vector is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{Vec, into_vec};
/// let vec : Vec< i32 > = into_vec!( 1, 2, 3 );
/// assert_eq!( vec[ 0 ], 1 );
/// assert_eq!( vec[ 1 ], 2 );
/// assert_eq!( vec[ 2 ], 3 );
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{Vec, into_vec};
/// let words : Vec< String > = into_vec!( "alpha", "beta", "gamma" );
/// assert_eq!( words[ 0 ], "alpha" );
/// assert_eq!( words[ 1 ], "beta" );
/// assert_eq!( words[ 2 ], "gamma" );
/// ```
///
/// # Example
///
/// Creating a `Vec` of `String` from string literals and String objects:
///
/// ```rust
/// # use collection_tools::{Vec, into_vec};
/// let mixed : Vec< String > = into_vec!{ "value", "another value".to_string() };
/// assert_eq!( mixed[ 0 ], "value" );
/// assert_eq!( mixed[ 1 ], "another value" );
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_vec
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vec = $crate::collection::Vec::with_capacity( _cap );
    $(
      _vec.push( Into::into( $key ) );
    )*
    _vec
  }};
}
