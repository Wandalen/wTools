#[ allow( unused_imports, clippy::wildcard_imports ) ]
use super::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use alloc::collections::btree_set::*;

/// Creates a `BTreeSet` from a list of elements.
///
/// The `bset` macro allows for convenient creation of a `BTreeSet` with initial elements.
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
/// # use collection_tools::{ BTreeSet, bset };
/// // BTreeSet of &str
/// let set1 = bset!( "a", "b", "c" );
///
/// // With trailing comma
/// let set3 = bset!( 1, 2, 3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `BTreeSet`.
///   Each element can be of any type that implements the `Into<T>` trait, where `T` is the
///   type stored in the `BTreeSet`.
///
/// # Returns
///
/// Returns a `BTreeSet` containing all the specified elements. The capacity of the set is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices:
///
/// ```rust
/// # use collection_tools::{ BTreeSet, bset };
/// let set = bset!( "one", "two", "three" );
/// assert!( set.contains( "one" ) );
/// assert!( set.contains( "two" ) );
/// assert!( set.contains( "three" ) );
/// assert_eq!( set.len(), 3 );
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! bset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let mut _set = $crate::collection::BTreeSet::new();
    $(
      _set.insert( $key );
    )*
    _set
  }};
}

/// Creates a `BTreeSet` from a list of elements.
///
/// The `into_bset` macro allows for convenient creation of a `BTreeSet` with initial elements.
/// Elements passed to the macro are automatically converted into the set's element type
/// using `.into()`, facilitating the use of literals or values of different, but convertible types.
///
/// Note: The `into_bset` macro relies on the `.into()` method to convert each element into the target type
/// of the `BTreeSet`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `BTreeSet`. Also, this means that sometimes you must specify the type of collection's items.
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
/// # use collection_tools::{ BTreeSet, into_bset };
/// // BTreeSet of &str
/// let set1 : BTreeSet< &str > = into_bset!( "a", "b", "c" );
///
/// // BTreeSet of String
/// let set2 : BTreeSet< String > = into_bset!{ "a".to_string(), "b", "c" };
///
/// // With trailing comma
/// let set3 : BTreeSet< i32 > = into_bset!( 1, 2, 3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `BTreeSet`.
///   Each element can be of any type that implements the `Into<T>` trait, where `T` is the
///   type stored in the `BTreeSet`.
///
/// # Returns
///
/// Returns a `BTreeSet` containing all the specified elements. The capacity of the set is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices:
///
/// ```rust
/// # use collection_tools::{ BTreeSet, into_bset };
/// let set  : BTreeSet< &str > = into_bset!( "one", "two", "three" );
/// assert!( set.contains( "one" ) );
/// assert!( set.contains( "two" ) );
/// assert!( set.contains( "three" ) );
/// assert_eq!( set.len(), 3 );
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{ BTreeSet, into_bset };
/// let numbers : BTreeSet< i32 > = into_bset!( 1, 2, 3 );
/// assert!( numbers.contains( &1 ) );
/// assert!( numbers.contains( &2 ) );
/// assert!( numbers.contains( &3 ) );
/// ```
///
/// # Example
///
/// Creating a `BTreeSet` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ BTreeSet, into_bset };
/// let s : BTreeSet< String > = into_bset!{ "value" };
/// assert!( s.contains( "value" ) );
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_bset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let mut _set = $crate::collection::BTreeSet::new();
    $(
      _set.insert( Into::into( $key ) );
    )*
    _set
  }};
}
