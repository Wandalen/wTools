#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "use_alloc" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use crate::dependency::hashbrown::hash_set::*;

#[ cfg( not( feature = "no_std" ) ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use std::collections::hash_set::*;

/// Creates a `HashSet` from a list of elements.
///
/// The `hset` macro allows for convenient creation of a `HashSet` with initial elements.
///
/// # Origin
///
/// This collection can be reexported from different crates:
/// - from `std`, if `use_std` is on ( `no_std` flag if off )
/// - from `hashbrown`, if `use_alloc` flag if on
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashSet, hset };
/// // HashSet of &str
/// let set1 = hset!( "a", "b", "c" );
///
/// // HashSet of &str
/// let set2 = hset!{ "a", "b", "c" };
///
/// // With trailing comma
/// let set3 = hset!( 1, 2, 3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `HashSet`.
/// Each element can be of any type that implements the `Into< T >` trait, where `T` is the
/// type stored in the `HashSet`.
///
/// # Returns
///
/// Returns a `HashSet` containing all the specified elements. The capacity of the set is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices:
///
/// ```rust
/// # use collection_tools::{ HashSet, hset };
/// let set = hset!( "one", "two", "three" );
/// assert!( set.contains( "one" ) );
/// assert!( set.contains( "two" ) );
/// assert!( set.contains( "three" ) );
/// assert_eq!( set.len(), 3 );
/// ```
///
/// # Example
///
/// Creating a `HashSet` of `&str` from string literals:
///
/// ```rust
/// # use collection_tools::{ HashSet, hset };
/// let s = hset!{ "value" };
/// assert_eq!( s.get( "value" ), Some( &"value" ) );
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! hset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _set = $crate::collection::HashSet::with_capacity( _cap );
    $(
      let _ = _set.insert( $key );
    )*
    _set
  }};
}

/// Creates a `HashSet` from a list of elements.
///
/// The `into_hset` macro allows for convenient creation of a `HashSet` with initial elements.
/// Elements passed to the macro are automatically converted into the set's element type
/// using `.into()`, facilitating the use of literals or values of different, but convertible types.
///
/// Note: The `into_hset` macro relies on the `.into()` method to convert each element into the target type
/// of the `HashSet`. This means that the elements must be compatible with the `Into< T >` trait for the
/// type `T` used in the `HashSet`. Also, this means that sometimes you must specify the type of collection's items.
///
/// # Origin
///
/// This collection can be reexported from different crates:
/// - from `std`, if `use_std` is on ( `no_std` flag if off )
/// - from `hashbrown`, if `use_alloc` flag if on
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashSet, into_hset };
/// // HashSet of &str
/// let set1 : HashSet< &str > = into_hset!( "a", "b", "c" );
///
/// // HashSet of String
/// let set2 : HashSet< String > = into_hset!{ "a".to_string(), "b", "c" };
///
/// // With trailing comma
/// let set3 : HashSet< i32 > = into_hset!( 1, 2, 3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `HashSet`.
/// Each element can be of any type that implements the `Into< T >` trait, where `T` is the
/// type stored in the `HashSet`.
///
/// # Returns
///
/// Returns a `HashSet` containing all the specified elements. The capacity of the set is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices:
///
/// ```rust
/// # use collection_tools::{ HashSet, into_hset };
/// let set : HashSet< &str > = into_hset!( "one", "two", "three" );
/// assert!( set.contains( "one" ) );
/// assert!( set.contains( "two" ) );
/// assert!( set.contains( "three" ) );
/// assert_eq!( set.len(), 3 );
/// ```
///
/// # Example
///
/// Using with different types that implement `Into< T >`:
///
/// ```rust
/// # use collection_tools::{ HashSet, into_hset };
/// let numbers : HashSet< i32 > = into_hset!( 1, 2, 3 );
/// assert!( numbers.contains( &1 ) );
/// assert!( numbers.contains( &2 ) );
/// assert!( numbers.contains( &3 ) );
/// ```
///
/// # Example
///
/// Creating a `HashSet` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ HashSet, into_hset };
/// let s : HashSet< String > = into_hset!{ "value" };
/// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) );
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_hset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _set = $crate::collection::HashSet::with_capacity( _cap );
    $(
      let _ = _set.insert( Into::into( $key ) );
    )*
    _set
  }};
}
