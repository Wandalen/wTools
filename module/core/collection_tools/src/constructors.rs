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
/// Each key and value can be of any type that implements the `Into< K >` and `Into< V >` traits, where `K` and `V` are the
/// types stored in the `BTreeMap` as keys and values, respectively.
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
#[ macro_export( local_inner_macros ) ]
macro_rules! bmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let mut _map = collection_tools::BTreeMap::new();
    $(
      let _ = _map.insert( $key , $value );
    )*
    _map
  }};
}

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
/// Each element can be of any type that implements the `Into<T>` trait, where `T` is the
/// type stored in the `BTreeSet`.
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
#[ macro_export( local_inner_macros ) ]
macro_rules! bset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let mut _set = collection_tools::BTreeSet::new();
    $(
      _set.insert( $key );
    )*
    _set
  }};
}

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
#[ macro_export( local_inner_macros ) ]
macro_rules! heap
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _heap = collection_tools::BinaryHeap::with_capacity( _cap );
    $(
      _heap.push( $key );
    )*
    _heap
  }};
}

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
#[macro_export(local_inner_macros)]
macro_rules! hmap
{
  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _map = collection_tools::HashMap::with_capacity( _cap );
    $(
      let _ = _map.insert( $key, $value );
    )*
    _map
  }};
}

/// Creates a `HashSet` from a list of elements.
///
/// The `hset` macro allows for convenient creation of a `HashSet` with initial elements.
/// 
/// # Origin
/// 
/// This collection can be reexported from different crates:
/// - from `std`, if `no_std` flag if off
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
#[ macro_export( local_inner_macros ) ]
macro_rules! hset
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _set = collection_tools::HashSet::with_capacity( _cap );
    $(
      let _ = _set.insert( $key );
    )*
    _set
  }};
}

/// Creates a `LinkedList` from a list of elements.
///
/// The `list` macro facilitates the creation of a `LinkedList` with initial elements.
///
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
    let mut _lst = collection_tools::LinkedList::new();
    $(
      _lst.push_back( $key );
    )*
    _lst
  }};
}

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
#[ macro_export( local_inner_macros ) ]
macro_rules! vec
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vec = collection_tools::Vec::with_capacity( _cap );
    $(
      _vec.push( $key );
    )*
    _vec
  }};
}

/// Creates a `VecDeque` from a list of elements.
///
/// The `vecd` macro allows for the convenient creation of a `VecDeque` with initial elements.
/// Elements passed to the macro are automatically converted into the deque's element type
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `vecd` macro relies on the `.into()` method to convert each element into the target type
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
/// # use collection_tools::{ VecDeque, vecd };
/// // VecDeque of i32
/// let vd1 = vecd!( 1, 2, 3, 4, 5 );
///
/// // VecDeque of String
/// let vd2 = vecd!{ "hello", "world", "rust" };
///
/// // With trailing comma
/// let vd3 = vecd!( 1.1, 2.2, 3.3, );
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
/// # use collection_tools::{ VecDeque, vecd };
/// let vd : VecDeque< i32 > = vecd!( 1, 2, 3 );
/// assert_eq!( vd.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( vd.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Creating a `VecDeque` of `&str` from string literals:
///
/// ```rust
/// # use collection_tools::{ VecDeque, vecd };
/// let fruits = vecd!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple" ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry" ) ); // The last element
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! vecd
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = count!( @count $( $key ),* );
    let mut _vecd = collection_tools::VecDeque::with_capacity( _cap );
    $(
      _vecd.push_back( $key );
    )*
    _vecd
  }};
}
