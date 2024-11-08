#[ allow( unused_imports ) ]
use super::*;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use alloc::collections::binary_heap::*;

/// Creates a `BinaryHeap` from a list of elements.
///
/// The `into_heap` macro simplifies the creation of a `BinaryHeap` with initial elements.
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
/// # use collection_tools::{ BinaryHeap, heap };
/// // BinaryHeap of i32
/// let heap1 = heap!( 3, 1, 4, 1, 5, 9 );
///
/// // BinaryHeap of &str
/// let heap2 = heap!{ "pear", "apple", "banana" };
///
/// // With trailing comma
/// let heap3 = heap!( 2, 7, 1, 8, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `BinaryHeap`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `BinaryHeap`.
///
/// # Returns
///
/// Returns a `BinaryHeap` containing all the specified elements. The capacity of the heap is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, heap };
/// let heap = heap!( 5, 3, 7, 1 );
/// assert_eq!( heap.peek(), Some( &7 ) ); // The largest value is at the top of the heap
/// ```
///
#[ cfg( feature = "collection_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! heap
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _heap = $crate::collection::BinaryHeap::with_capacity( _cap );
    $(
      _heap.push( $key );
    )*
    _heap
  }};
}

/// Creates a `BinaryHeap` from a list of elements.
///
/// The `into_heap` macro simplifies the creation of a `BinaryHeap` with initial elements.
/// Elements passed to the macro are automatically converted into the heap's element type
/// using `.into()`, allowing for the use of literals or values of different, but convertible types.
///
/// Note: The `into_heap` macro utilizes the `.into()` method to convert each element into the target type
/// of the `BinaryHeap`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `BinaryHeap`. Also, this means that sometimes you must specify the type of collection's items.
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
/// # use collection_tools::{ BinaryHeap, into_heap };
/// // BinaryHeap of i32
/// let heap1 : BinaryHeap< i32 > = into_heap!( 3, 1, 4, 1, 5, 9 );
///
/// // BinaryHeap of String
/// let heap2 : BinaryHeap< String > = into_heap!{ "pear".to_string(), "apple", "banana" };
///
/// // With trailing comma
/// let heap3 : BinaryHeap< i32 > = into_heap!( 2, 7, 1, 8, );
/// ```
///
/// # Parameters
///
/// - `$( $key:expr ),* $( , )?`: A comma-separated list of elements to insert into the `BinaryHeap`.
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `BinaryHeap`.
///
/// # Returns
///
/// Returns a `BinaryHeap` containing all the specified elements. The capacity of the heap is
/// automatically determined based on the number of elements provided.
///
/// # Example
///
/// Basic usage with integers:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, into_heap };
/// let heap : BinaryHeap< i32 > = into_heap!( 5, 3, 7, 1 );
/// assert_eq!( heap.peek(), Some( &7 ) ); // The largest value is at the top of the heap
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, into_heap };
/// let chars : BinaryHeap< char > = into_heap!( 'a', 'b', 'c' );
/// assert_eq!( chars.peek(), Some( &'c' ) ); // Characters are ordered by their ASCII value
/// ```
///
/// # Example
///
/// Creating a `BinaryHeap` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, into_heap };
/// let fruits : BinaryHeap< String > = into_heap!{ "cherry", "apple", "banana" };
/// assert_eq!( fruits.peek(), Some( &"cherry".to_string() ) ); // The lexicographically largest value is at the top
/// ```
///
#[ cfg( feature = "collection_into_constructors" ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! into_heap
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _heap = $crate::collection::BinaryHeap::with_capacity( _cap );
    $(
      _heap.push( Into::into( $key ) );
    )*
    _heap
  }};
}
