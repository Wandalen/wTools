/// Literally just a `BTreeMap` literal with keys and values into'd.
/// ```rust
/// # use collection_tools::*;
/// let m : BTreeMap< String, String > = bmap!{ "key" => "value" };
/// assert_eq!( m.get( "key" ), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! bmap
{
  (
    $( $key:expr => $value:expr ),* $( , )?
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
/// Literally just a `BTreeSet` literal with values into'd.
/// ```rust
/// # use collection_tools::*;
/// let s : BTreeSet< String > = bset!{ "value" };
/// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! bset
{
  (
    $( $key:expr ),* $( , )?
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
/// Literally just a `BinaryHeap` literal with values into'd.
/// ```rust
/// # use collection_tools::*;
/// let l : BinaryHeap< String > = heap![ "value" ];
/// assert_eq!( l.peek(), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! heap
{
  (
    $( $key:expr ),* $( , )?
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
/// Literally just a `HashMap` literal with keys and values into'd.
/// ```rust
/// # use collection_tools::*;
/// let m : HashMap< String, String > = hmap!{ "key" => "value" };
/// assert_eq!( m.get( "key" ), Some( &"value".to_string() ) )
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! hmap
{
  ( @single $( $x:tt )* ) => ( () );
  (
    @count $( $rest:expr ),*
  )
  =>
  (
    < [ () ] >::len( &[ $( hmap!( @single $rest ) ),* ] )
  );
  (
    $( $key:expr => $value:expr ),* $( , )?
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
/// Literally just a `HashSet` literal with values into'd.
/// ```rust
/// # use collection_tools::*;
/// let s : HashSet< String > = hset!{ "value" };
/// assert_eq!( s.get( "value" ), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! hset
{
  ( @single $( $x:tt )* ) => ( () );
  (
    @count $( $rest:expr ),*
  )
  =>
  (
    < [ () ] >::len( &[ $( hset!( @single $rest ) ),* ] )
  );
  (
    $( $key:expr ),* $( , )?
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
/// Literally just a `LinkedList` literal with values into'd.
/// ```rust
/// # use collection_tools::*;
/// let l : LinkedList< String > = list![ "value" ];
/// assert_eq!( l.front(), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! list
{
  (
    $( $key:expr ),* $( , )?
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
/// Literally just a `VecDeque` literal with values into'd.
/// ```rust
/// # use collection_tools::*;
/// let s : VecDeque< String > = vecd![ "value" ];
/// assert_eq!( s.get( 0 ), Some( &"value".to_string() ) )
/// ```
///
#[ macro_export( local_inner_macros ) ]
macro_rules! vecd
{
  (
    $( $key:expr ),* $( , )?
  )
  =>
  {
    collection_tools::VecDeque::from
    (
      collection_tools::vec![ $( $key.into() ),* ]
    )
  }
}