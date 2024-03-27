/// Creates a `BTreeMap` from a list of key-value pairs.
///
/// The `bmap` macro facilitates the convenient creation of a `BTreeMap` with initial elements.
/// Keys and values passed to the macro are automatically converted into the map's key and value types
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `bmap` macro relies on the `.into()` method to convert each key and value into the target types
/// of the `BTreeMap`. This means that the keys and values must be compatible with the `Into< K >` and `Into< V >` traits
/// for the key type `K` and value type `V` used in the `BTreeMap`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ BTreeMap, bmap };
/// // BTreeMap of &str to i32
/// let map1 : BTreeMap< &str, i32 > = bmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // BTreeMap of String to String
/// let map2 : BTreeMap< String, String > = bmap!{ "name" => "value" };
///
/// // With trailing comma
/// let map3 : BTreeMap< i32, &str > = bmap!( 1 => "one", 2 => "two", 3 => "three", );
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
/// let map : BTreeMap< &str, i32 > = bmap!( "one" => 1, "two" => 2, "three" => 3 );
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
/// # use collection_tools::{ BTreeMap, bmap };
/// let months : BTreeMap< String, i32 > = bmap!( "January" => 1, "February" => 2, "March" => 3 );
/// assert_eq!( months.get( &"January".to_string() ), Some( &1 ) );
/// assert_eq!( months.get( &"February".to_string() ), Some( &2 ) );
/// ```
///
/// # Example
///
/// Creating a `BTreeMap` of integers to strings from literals:
///
/// ```rust
/// # use collection_tools::{ BTreeMap, bmap };
/// let numbers : BTreeMap< i32, String > = bmap!( 1 => "one", 2 => "two", 3 => "three" );
/// assert_eq!( numbers.get( &1 ), Some( &"one".to_string() ) );
/// assert_eq!( numbers.get( &2 ), Some( &"two".to_string() ) );
/// assert_eq!( numbers.get( &3 ), Some( &"three".to_string() ) );
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
      let _ = _map.insert( $key.into(), $value.into() );
    )*
    _map
  }};
}

/// Creates a `BTreeSet` from a list of elements.
///
/// The `bset` macro allows for convenient creation of a `BTreeSet` with initial elements.
/// Elements passed to the macro are automatically converted into the set's element type
/// using `.into()`, facilitating the use of literals or values of different, but convertible types.
///
/// Note: The `bset` macro relies on the `.into()` method to convert each element into the target type
/// of the `BTreeSet`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `BTreeSet`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ BTreeSet, bset };
/// // BTreeSet of &str
/// let set1 : BTreeSet< &str > = bset!( "a", "b", "c" );
///
/// // BTreeSet of String
/// let set2 : BTreeSet< String > = bset!{ "a".to_string(), "b", "c" };
///
/// // With trailing comma
/// let set3 : BTreeSet< i32 > = bset!( 1, 2, 3, );
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
/// let set  : BTreeSet< &str > = bset!( "one", "two", "three" );
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
/// # use collection_tools::{ BTreeSet, bset };
/// let numbers : BTreeSet< i32 > = bset!( 1, 2, 3 );
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
/// # use collection_tools::{ BTreeSet, bset };
/// let s : BTreeSet< String > = bset!{ "value" };
/// assert!( s.contains( "value" ) );
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
    let mut _set = ::collection_tools::BTreeSet::new();
    $(
      _set.insert( $key.into() );
    )*
    _set
  }};
}

/// Creates a `BinaryHeap` from a list of elements.
///
/// The `heap` macro simplifies the creation of a `BinaryHeap` with initial elements.
/// Elements passed to the macro are automatically converted into the heap's element type
/// using `.into()`, allowing for the use of literals or values of different, but convertible types.
///
/// Note: The `heap` macro utilizes the `.into()` method to convert each element into the target type
/// of the `BinaryHeap`. This means that the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `BinaryHeap`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, heap };
/// // BinaryHeap of i32
/// let heap1 : BinaryHeap< i32 > = heap!( 3, 1, 4, 1, 5, 9 );
///
/// // BinaryHeap of String
/// let heap2 : BinaryHeap< String > = heap!{ "pear".to_string(), "apple", "banana" };
///
/// // With trailing comma
/// let heap3 : BinaryHeap< i32 > = heap!( 2, 7, 1, 8, );
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
/// let heap : BinaryHeap< i32 > = heap!( 5, 3, 7, 1 );
/// assert_eq!( heap.peek(), Some( &7 ) ); // The largest value is at the top of the heap
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, heap };
/// let chars : BinaryHeap< char > = heap!( 'a', 'b', 'c' );
/// assert_eq!( chars.peek(), Some( &'c' ) ); // Characters are ordered by their ASCII value
/// ```
///
/// # Example
///
/// Creating a `BinaryHeap` of `String` from string literals:
///
/// ```rust
/// # use collection_tools::{ BinaryHeap, heap };
/// let fruits : BinaryHeap< String > = heap!{ "cherry", "apple", "banana" };
/// assert_eq!( fruits.peek(), Some( &"cherry".to_string() ) ); // The lexicographically largest value is at the top
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
    let mut _heap = collection_tools::BinaryHeap::new();
    $(
      _heap.push( $key.into() );
    )*
    _heap
  }};
}

/// Creates a `HashMap` from a list of key-value pairs.
///
/// The `hmap` macro allows for convenient creation of a `HashMap` with initial elements.
/// Keys and values passed to the macro are automatically converted into the map's key and value types
/// using `.into()`, enabling the use of literals or values of different, but convertible types.
///
/// Note: The `hmap` macro relies on the `.into()` method to convert each key and value into the target types
/// of the `HashMap`. This means that the keys and values must be compatible with the `Into<K>` and `Into<V>` traits
/// for the key type `K` and value type `V` used in the `HashMap`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of key-value pairs. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// // HashMap of &str to i32
/// let map1 : HashMap< &str, i32 > = hmap!( "one" => 1, "two" => 2, "three" => 3 );
///
/// // HashMap of String to String
/// let map2 : HashMap< String, String > = hmap!{ "name".to_string() => "value".to_string(), "type" => "example" };
///
/// // With trailing comma
/// let map3 : HashMap< i32, &str > = hmap!( 1 => "one", 2 => "two", 3 => "three", );
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
/// Using with different types that implement `Into<K>` and `Into<V>`:
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// let items : HashMap< String, i32 > = hmap!( "pen" => 10, "book" => 45, "eraser" => 5 );
/// assert_eq!( items.get( &"pen".to_string() ), Some(&10 ) );
/// assert_eq!( items.get( &"book".to_string() ), Some(&45 ) );
/// ```
///
/// # Example
///
/// Creating a `HashMap` of integers to strings from literals:
///
/// ```rust
/// # use collection_tools::{ HashMap, hmap };
/// let pairs : HashMap< i32, String > = hmap!( 1 => "apple", 2 => "banana" );
/// assert_eq!( pairs.get( &1 ), Some( &"apple".to_string() ) );
/// assert_eq!( pairs.get( &2 ), Some( &"banana".to_string() ) );
/// ```
///

#[macro_export(local_inner_macros)]
macro_rules! hmap
{
  ( @single $( $x : tt )* ) => ( () );

  (
    @count $( $rest : expr ),*
  )
  =>
  (
    < [ () ] >::len( &[ $( hmap!( @single $rest ) ),* ] )
  );

  (
    $( $key : expr => $value : expr ),* $( , )?
  )
  =>
  {{
    let _cap = hmap!( @count $( $key ),* );
    let mut _map = collection_tools::HashMap::with_capacity( _cap );
    $(
      let _ = _map.insert( $key.into(), $value.into() );
    )*
    _map
  }};
}

/// Creates a `HashSet` from a list of elements.
///
/// The `hset` macro allows for convenient creation of a `HashSet` with initial elements.
/// Elements passed to the macro are automatically converted into the set's element type
/// using `.into()`, facilitating the use of literals or values of different, but convertible types.
///
/// Note: The `hset` macro relies on the `.into()` method to convert each element into the target type
/// of the `HashSet`. This means that the elements must be compatible with the `Into< T >` trait for the
/// type `T` used in the `HashSet`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ HashSet, hset };
/// // HashSet of &str
/// let set1 : HashSet< &str > = hset!( "a", "b", "c" );
///
/// // HashSet of String
/// let set2 : HashSet< String > = hset!{ "a".to_string(), "b", "c" };
///
/// // With trailing comma
/// let set3 : HashSet< i32 > = hset!( 1, 2, 3, );
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
/// let set : HashSet< &str > = hset!( "one", "two", "three" );
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
/// # use collection_tools::{ HashSet, hset };
/// let numbers : HashSet< i32 > = hset!( 1, 2, 3 );
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
/// # use collection_tools::{ HashSet, hset };
/// let s : HashSet< String > = hset!{ "value" };
/// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) );
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! hset
{
  ( @single $( $x : tt )* ) => ( () );

  (
    @count $( $rest : expr ),*
  )
  =>
  (
    < [ () ] >::len( &[ $( hset!( @single $rest ) ),* ] )
  );

  (
    $( $key : expr ),* $( , )?
  )
  =>
  {{
    let _cap = hset!( @count $( $key ),* );
    let mut _set = collection_tools::HashSet::with_capacity( _cap );
    $(
      let _ = _set.insert( $key.into() );
    )*
    _set
  }};
}

/// Creates a `LinkedList` from a list of elements.
///
/// The `list` macro facilitates the creation of a `LinkedList` with initial elements.
/// Elements passed to the macro are automatically converted into the list's element type
/// using `.into()`, making it convenient to use literals or values of different, but convertible types.
///
/// Note: The `list` macro leverages the `.into()` method to convert each element into the target type
/// of the `LinkedList`. Therefore, the elements must be compatible with the `Into<T>` trait for the
/// type `T` used in the `LinkedList`.
///
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ LinkedList, list };
/// // LinkedList of i32
/// let lst1 : LinkedList< i32 > = list!( 1, 2, 3, 4, 5 );
///
/// // LinkedList of String
/// let lst2 : LinkedList< String > = list!{ "hello".to_string(), "world", "rust" };
///
/// // With trailing comma
/// let lst3 : LinkedList< f64 > = list!( 1.1, 2.2, 3.3, );
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
/// let lst: LinkedList< i32 > = list!( 1, 2, 3 );
/// assert_eq!( lst.front(), Some( &1 ) ); // The first element is 1
/// assert_eq!( lst.back(), Some( &3 ) ); // The last element is 3
/// ```
///
/// # Example
///
/// Using with different types that implement `Into<T>`:
///
/// ```rust
/// # use collection_tools::{ LinkedList, list };
/// let chars : LinkedList< String > = list!( "a", "b", "c" );
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
/// # use collection_tools::{ LinkedList, list };
/// let fruits : LinkedList< String > = list!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple".to_string() ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry".to_string() ) ); // The last element
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
    let mut _lst = collection_tools::LinkedList::new();
    $(
      _lst.push_back( $key.into() );
    )*
    _lst
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
/// # Syntax
///
/// The macro can be called with a comma-separated list of elements. A trailing comma is optional.
///
/// ```rust
/// # use collection_tools::{ VecDeque, vecd };
/// // VecDeque of i32
/// let vd1 : VecDeque< i32 > = vecd!( 1, 2, 3, 4, 5 );
///
/// // VecDeque of String
/// let vd2 : VecDeque< String > = vecd!{ "hello".to_string(), "world", "rust" };
///
/// // With trailing comma
/// let vd3 : VecDeque< f64 > = vecd!( 1.1, 2.2, 3.3, );
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
/// Using with different types that implement `Into< T >`:
///
/// ```rust
/// # use collection_tools::{ VecDeque, vecd };
/// let chars : VecDeque< char > = vecd!( 'a', 'b', 'c' );
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
/// # use collection_tools::{ VecDeque, vecd };
/// let fruits : VecDeque< String > = vecd!{ "apple", "banana", "cherry" };
/// assert_eq!( fruits.front(), Some( &"apple".to_string() ) ); // The first element
/// assert_eq!( fruits.back(), Some( &"cherry".to_string() ) ); // The last element
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! vecd
{
  (
    $( $key : expr ),* $( , )?
  )
  =>
  {
    collection_tools::VecDeque::from
    (
      collection_tools::vec![ $( $key.into() ),* ]
    )
  }
}
