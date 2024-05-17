#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use alloc::collections::linked_list::*;

/// Creates a `LinkedList` from a list of elements.
///
/// The `list` macro facilitates the creation of a `LinkedList` with initial elements.
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
/// # use collection_tools::{ LinkedList, list };
/// // LinkedList of i32
/// let lst1 = list!( 1, 2, 3, 4, 5 );
///
/// // LinkedList of &str
/// let lst2 = list!{ "hello", "world", "rust" };
///
/// // With trailing comma
/// let lst3 = list!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `LinkedList`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `LinkedList`.
///
/// # Returns
///
/// Returns a `LinkedList` containing all the specified elements. The capacity of the list is
/// dynamically adjusted based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ LinkedList, list };
/// let lst = list!( 1, 2, 3 );
/// assert_eq!( lst.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( lst.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Creating a `LinkedList` of `&str` from string literals:
///
/// ```rust
/// # use collection_tools::{ LinkedList, list };
/// let fruits = list!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple" ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry" ) ); // The last element
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! list
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    // "The LinkedList allows pushing and popping elements at either end in constant time."
    // So no `with_capacity`
    let mut _lst = $crate::list::LinkedList::new();
    $(
      _lst.push_back( $key );
    )*
    _lst
  }};
}

/// Creates a `LinkedList` from a list of elements.
///
/// The `into_list` macro facilitates the creation of a `LinkedList` with initial elements.
/// Elements passed to the macro are automatically converted into the list's element type
/// using `.into()`, making it convenient to use literals or values of different, but convertible types.
///
/// Note: The `into_list` macro leverages the `.into()` method to convert each element into the target type
/// of the `LinkedList`. Therefore, the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `LinkedList`. Also, this means that sometimes you must specify the type of collection's items.
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
/// # use collection_tools::{ LinkedList, into_list };
/// // LinkedList of i32
/// let lst1 : LinkedList< i32 > = into_list!( 1, 2, 3, 4, 5 );
///
/// // LinkedList of String
/// let lst2 : LinkedList< String > = into_list!{ "hello".to_string(), "world", "rust" };
///
/// // With trailing comma
/// let lst3 : LinkedList< f64 > = into_list!( 1.1, 2.2, 3.3, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `LinkedList`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `LinkedList`.
///
/// # Returns
///
/// Returns a `LinkedList` containing all the specified elements. The capacity of the list is
/// dynamically adjusted based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ LinkedList, into_list };
/// let lst: LinkedList< i32 > = into_list!( 1, 2, 3 );
/// assert_eq!( lst.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( lst.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{ LinkedList, into_list };
/// let chars : LinkedList< String > = into_list!( "a", "b", "c" );
/// assert!( chars.contains( &"a".to_string() ) );
/// assert!( chars.contains( &"b".to_string() ) );
/// assert!( chars.contains( &"c".to_string() ) );
/// ```
///
/// # Example
///
/// Creating a `LinkedList` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ LinkedList, into_list };
/// let fruits : LinkedList< String > = into_list!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple".to_string() ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry".to_string() ) ); // The last element
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_list
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    // "The LinkedList allows pushing and popping elements at either end in constant time."
    // So no `with_capacity`
    let mut _lst = $crate::list::LinkedList::new();
    $(
      _lst.push_back( Into::into( $key ) );
    )*
    _lst
  }};
}
