#[ cfg( feature = "use_alloc" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use crate::dependency::hashbrown::hash_map::*;
#[ cfg( not( feature = "no_std" ) ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use std::collections::hash_map::*;

/// Creates a `HashMap` from a list of key-value pairs.
///
/// The `hmap` macro allows for convenient creation of a `HashMap` with initial elements.
///
/// # Origin
///
/// This collection can be reexported from different crates:
/// - from `std`, if `no_std` flag if off
/// - from `hashbrown`, if `use_alloc` flag if on
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// // HashMap of &str to i32
/// let map1 = hmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // HashMap of &str to &str
/// let map2 = hmap!{ "name" => "value", "type" => "example" };
///
/// // With trailing comma
/// let map3 = hmap!( 1 => "one", 2 => "two", 3 => "three", );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr => $value:expr ),* $( , )?`: A comma-separated list of key-value pairs to insert into the `HashMap`.
/// Each key and value can be of any type that implements the `Into<K>` and `Into<V>` traits, where `K` and `V` are the
/// types stored in the `HashMap` as keys and values, respectively.
///
/// # Returns
///
/// Returns a `HashMap` containing all the specified key-value pairs. The capacity of the map is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices and integer values:
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// let map : HashMap< &str, i32 > = hmap!( "one" => 1, "two" => 2, "three" => 3 );
/// assert_eq!( map.get( "one" ), Some( &1 ) );
/// assert_eq!( map.get( "two" ), Some( &2 ) );
/// assert_eq!( map.get( "three" ), Some( &3 ) );
/// ```
///
/// # Example
///
/// Creating a `HashMap` of integers to strings from literals:
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// let pairs = hmap!( 1 => "apple", 2 => "banana" );
/// assert_eq!( pairs.get( &1 ), Some( &"apple" ) );
/// assert_eq!( pairs.get( &2 ), Some( &"banana" ) );
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! hmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _map = $crate::hmap::HashMap::with_capacity( _cap );
    $(
      let _ = _map.insert( $key, $value );
    )*
    _map
  }};
}

/// Creates a `HashMap` from a list of key-value pairs.
///
/// The `into_hmap` macro allows for convenient creation of a `HashMap` with initial elements.
/// Keys and values passed to the macro are automatically converted into the map's key and value types
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `into_hmap` macro relies on the `.into()` method to convert each key and value into the target types
/// of the `HashMap`. This means that the keys and values must be compatible with the `Into<K>` and `Into<V>` traits
/// for the key type `K` and value type `V` used in the `HashMap`. Also, this means that sometimes you must specify the type of collection's items.
///
/// # Origin
///
/// This collection can be reexported from different crates:
/// - from `std`, if `no_std` flag if off
/// - from `hashbrown`, if `use_alloc` flag if on
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashMap, into_hmap };
/// // HashMap of &str to i32
/// let map1 : HashMap< &str, i32 > = into_hmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // HashMap of String to String
/// let map2 : HashMap< String, String > = into_hmap!{ "name".to_string() => "value".to_string(), "type" => "example" };
///
/// // With trailing comma
/// let map3 : HashMap< i32, &str > = into_hmap!( 1 => "one", 2 => "two", 3 => "three", );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr => $value:expr ),* $( , )?`: A comma-separated list of key-value pairs to insert into the `HashMap`.
/// Each key and value can be of any type that implements the `Into<K>` and `Into<V>` traits, where `K` and `V` are the
/// types stored in the `HashMap` as keys and values, respectively.
///
/// # Returns
///
/// Returns a `HashMap` containing all the specified key-value pairs. The capacity of the map is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with string slices and integer values:
///
/// ```rust
/// # use collection_tools::{ HashMap, into_hmap };
/// let map : HashMap< &str, i32 > = into_hmap!( "one" => 1, "two" => 2, "three" => 3 );
/// assert_eq!( map.get( "one" ), Some( &1 ) );
/// assert_eq!( map.get( "two" ), Some( &2 ) );
/// assert_eq!( map.get( "three" ), Some( &3 ) );
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<K>` and `Into<V>`:
///
/// ```rust
/// # use collection_tools::{ HashMap, into_hmap };
/// let items : HashMap< String, i32 > = into_hmap!( "pen" => 10, "book" => 45, "eraser" => 5 );
/// assert_eq!( items.get( &"pen".to_string() ), Some(&10 ) );
/// assert_eq!( items.get( &"book".to_string() ), Some(&45 ) );
/// ```
///
/// # Example
///
/// Creating a `HashMap` of integers to strings from literals:
///
/// ```rust
/// # use collection_tools::{ HashMap, into_hmap };
/// let pairs : HashMap< i32, String > = into_hmap!( 1 => "apple", 2 => "banana" );
/// assert_eq!( pairs.get( &1 ), Some( &"apple".to_string() ) );
/// assert_eq!( pairs.get( &2 ), Some( &"banana".to_string() ) );
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_hmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _map = $crate::hmap::HashMap::with_capacity( _cap );
    $(
      let _ = _map.insert( Into::into( $key ), Into::into( $value ) );
    )*
    _map
  }};
}
