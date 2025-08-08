#[ allow( unused_imports, clippy::wildcard_imports ) ]
use super::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use alloc::collections::btree_map::*;

/// Creates a `BTreeMap` from a list of key-value pairs.
///
/// The `bmap` macro facilitates the convenient creation of a `BTreeMap` with initial elements.
///
/// # Origin
///
/// This collection is reexported from `alloc`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ BTreeMap, bmap };
/// // BTreeMap of &str to i32
/// let map1 = bmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // BTreeMap of &str to &str
/// let map2 = bmap!{ "name" => "value" };
///
/// // With trailing comma
/// let map3 = bmap!( 1 => "one", 2 => "two", 3 => "three", );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr => $value:expr ),* $( , )?`: A comma-separated list of key-value pairs to insert into the `BTreeMap`.
///   Each key and value can be of any type that implements the `Into< K >` and `Into< V >` traits, where `K` and `V` are the
///   types stored in the `BTreeMap` as keys and values, respectively.
///
/// # Returns
///
/// Returns a `BTreeMap` containing all the specified key-value pairs. The map's capacity is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices and integer values:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, bmap };
/// let map = bmap!( "one" => 1, "two" => 2, "three" => 3 );
/// assert_eq!( map.get( "one" ), Some( &1 ) );
/// assert_eq!( map.get( "two" ), Some( &2 ) );
/// assert_eq!( map.get( "three" ), Some( &3 ) );
/// ```
///
/// # Example
///
/// Creating a `BTreeMap` of integers to string slices from literals:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, bmap };
/// let numbers = bmap!( 1 => "one", 2 => "two", 3 => "three" );
/// assert_eq!( numbers.get( &1 ), Some( &"one" ) );
/// assert_eq!( numbers.get( &2 ), Some( &"two" ) );
/// assert_eq!( numbers.get( &3 ), Some( &"three" ) );
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! bmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let mut _map = $crate::collection::BTreeMap::new();
    $(
      let _ = _map.insert( $key , $value );
    )*
    _map
  }};
}

/// Creates a `BTreeMap` from a list of key-value pairs.
///
/// The `into_bmap` macro facilitates the convenient creation of a `BTreeMap` with initial elements.
/// Keys and values passed to the macro are automatically converted into the map's key and value types
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `into_bmap` macro relies on the `.into()` method to convert each key and value into the target types
/// of the `BTreeMap`. This means that the keys and values must be compatible with the `Into< K >` and `Into< V >` traits
/// for the key type `K` and value type `V` used in the `BTreeMap`. Also, this means that sometimes you must specify the type of collection's items.
///
/// # Origin
///
/// This collection is reexported from `alloc`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ BTreeMap, into_bmap };
/// // BTreeMap of &str to i32
/// let map1 : BTreeMap< &str, i32 > = into_bmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // BTreeMap of String to String
/// let map2 : BTreeMap< String, String > = into_bmap!{ "name" => "value" };
///
/// // With trailing comma
/// let map3 : BTreeMap< i32, &str > = into_bmap!( 1 => "one", 2 => "two", 3 => "three", );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr => $value:expr ),* $( , )?`: A comma-separated list of key-value pairs to insert into the `BTreeMap`.
///   Each key and value can be of any type that implements the `Into< K >` and `Into< V >` traits, where `K` and `V` are the
///   types stored in the `BTreeMap` as keys and values, respectively.
///
/// # Returns
///
/// Returns a `BTreeMap` containing all the specified key-value pairs. The map's capacity is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices and integer values:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, into_bmap };
/// let map : BTreeMap< &str, i32 > = into_bmap!( "one" => 1, "two" => 2, "three" => 3 );
/// assert_eq!( map.get( "one" ), Some( &1 ) );
/// assert_eq!( map.get( "two" ), Some( &2 ) );
/// assert_eq!( map.get( "three" ), Some( &3 ) );
/// ```
///
/// # Example
///
/// Using with different types that implement `Into< K >` and `Into< V >`:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, into_bmap };
/// let months : BTreeMap< String, i32 > = into_bmap!( "January" => 1, "February" => 2, "March" => 3 );
/// assert_eq!( months.get( &"January".to_string() ), Some( &1 ) );
/// assert_eq!( months.get( &"February".to_string() ), Some( &2 ) );
/// ```
///
/// # Example
///
/// Creating a `BTreeMap` of integers to strings from literals:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, into_bmap };
/// let numbers : BTreeMap< i32, String > = into_bmap!( 1 => "one", 2 => "two", 3 => "three" );
/// assert_eq!( numbers.get( &1 ), Some( &"one".to_string() ) );
/// assert_eq!( numbers.get( &2 ), Some( &"two".to_string() ) );
/// assert_eq!( numbers.get( &3 ), Some( &"three".to_string() ) );
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_bmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let mut _map = $crate::collection::BTreeMap::new();
    $(
      let _ = _map.insert( Into::into( $key ), Into::into( $value ) );
    )*
    _map
  }};
}
