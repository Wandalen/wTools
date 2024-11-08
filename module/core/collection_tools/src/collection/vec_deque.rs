#[ allow( unused_imports ) ]
use super::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use alloc::collections::vec_deque::*;

/// Creates a `VecDeque` from a list of elements.
///
/// The `deque` macro allows for the convenient creation of a `VecDeque` with initial elements.
/// Elements passed to the macro are automatically converted into the deque's element type
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `deque` macro relies on the `.into()` method to convert each element into the target type
/// of the `VecDeque`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `VecDeque`.
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
/// # use collection_tools::{ VecDeque, deque };
/// // VecDeque of i32
/// let vd1 = deque!( 1, 2, 3, 4, 5 );
///
/// // VecDeque of String
/// let vd2 = deque!{ "hello", "world", "rust" };
///
/// // With trailing comma
/// let vd3 = deque!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `VecDeque`.
/// Each element can be of any type that implements the `Into< T >` trait, where `T` is the
/// type stored in the `VecDeque`.
///
/// # Returns
///
/// Returns a `VecDeque` containing all the specified elements. The capacity of the deque is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ VecDeque, deque };
/// let vd : VecDeque< i32 > = deque!( 1, 2, 3 );
/// assert_eq!( vd.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( vd.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Creating a `VecDeque` of `&str` from string literals:
///
/// ```rust
/// # use collection_tools::{ VecDeque, deque };
/// let fruits = deque!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple" ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry" ) ); // The last element
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! deque
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vecd = $crate::collection::VecDeque::with_capacity( _cap );
    $(
      _vecd.push_back( $key );
    )*
    _vecd
  }};
}

/// Creates a `VecDeque` from a list of elements.
///
/// The `into_vecd` macro allows for the convenient creation of a `VecDeque` with initial elements.
/// Elements passed to the macro are automatically converted into the deque's element type
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `into_vecd` macro relies on the `.into()` method to convert each element into the target type
/// of the `VecDeque`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `VecDeque`.
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
/// # use collection_tools::{ VecDeque, into_vecd };
/// // VecDeque of i32
/// let vd1 : VecDeque< i32 > = into_vecd!( 1, 2, 3, 4, 5 );
///
/// // VecDeque of String
/// let vd2 : VecDeque< String > = into_vecd!{ "hello".to_string(), "world", "rust" };
///
/// // With trailing comma
/// let vd3 : VecDeque< f64 > = into_vecd!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `VecDeque`.
/// Each element can be of any type that implements the `Into< T >` trait, where `T` is the
/// type stored in the `VecDeque`.
///
/// # Returns
///
/// Returns a `VecDeque` containing all the specified elements. The capacity of the deque is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ VecDeque, into_vecd };
/// let vd : VecDeque< i32 > = into_vecd!( 1, 2, 3 );
/// assert_eq!( vd.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( vd.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Using with different types that implement `Into< T >`:
///
/// ```rust
/// # use collection_tools::{ VecDeque, into_vecd };
/// let chars : VecDeque< char > = into_vecd!( 'a', 'b', 'c' );
/// assert!( chars.contains( &'a' ) );
/// assert!( chars.contains( &'b' ) );
/// assert!( chars.contains( &'c' ) );
/// ```
///
/// # Example
///
/// Creating a `VecDeque` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ VecDeque, into_vecd };
/// let fruits : VecDeque< String > = into_vecd!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple".to_string() ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry".to_string() ) ); // The last element
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_vecd
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vecd = $crate::collection::VecDeque::with_capacity( _cap );
    $(
      _vecd.push_back( Into::into( $key ) );
    )*
    _vecd
  }};
}
