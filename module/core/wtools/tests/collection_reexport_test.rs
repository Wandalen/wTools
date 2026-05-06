//! Tests verifying that `wtools` correctly re-exports `collection_tools` functionality.
//!
//! `wtools` is a thin facade over `collection_tools`, providing variadic constructors
//! (`hmap!`, `hset!`) and unified collection types (`HashMap`, `HashSet`, `BTreeMap`, etc.).
//! These tests ensure the re-export surface is complete and functional.

#[ test ]
fn hmap_macro_constructs_hashmap()
{
  let map = wtools::hmap!{ "a" => 1, "b" => 2, "c" => 3 };
  assert_eq!( map.len(), 3 );
  assert_eq!( map[ "b" ], 2 );
}

#[ test ]
fn hmap_macro_empty()
{
  let map : wtools::HashMap< String, i32 > = wtools::hmap!{};
  assert!( map.is_empty() );
}

#[ test ]
fn hset_macro_constructs_hashset()
{
  let set = wtools::hset!{ 10_i32, 20, 30 };
  assert_eq!( set.len(), 3 );
  assert!( set.contains( &20 ) );
}

#[ test ]
fn hset_macro_empty()
{
  let set : wtools::HashSet< i32 > = wtools::hset!{};
  assert!( set.is_empty() );
}

#[ test ]
fn bmap_macro_constructs_btreemap()
{
  let map = wtools::bmap!{ 1 => "one", 2 => "two" };
  assert_eq!( map.len(), 2 );
  assert_eq!( map[ &1 ], "one" );
}

#[ test ]
fn bset_macro_constructs_btreeset()
{
  let set = wtools::bset!{ 3_i32, 1, 2 };
  assert_eq!( set.len(), 3 );
  assert!( set.contains( &1 ) );
}

#[ test ]
fn hashmap_type_is_accessible()
{
  let mut map = wtools::HashMap::< &str, i32 >::new();
  map.insert( "x", 42 );
  assert_eq!( map.get( "x" ), Some( &42 ) );
}

#[ test ]
fn hashset_type_is_accessible()
{
  let mut set = wtools::HashSet::< i32 >::new();
  set.insert( 7 );
  assert!( set.contains( &7 ) );
}

#[ test ]
fn btreemap_type_is_accessible()
{
  let mut map = wtools::BTreeMap::< i32, &str >::new();
  map.insert( 1, "hello" );
  assert_eq!( map.get( &1 ), Some( &"hello" ) );
}

#[ test ]
fn btreeset_type_is_accessible()
{
  let mut set = wtools::BTreeSet::< i32 >::new();
  set.insert( 5 );
  assert!( set.contains( &5 ) );
}

#[ test ]
fn linked_list_type_is_accessible()
{
  let mut list = wtools::LinkedList::< i32 >::new();
  list.push_back( 1 );
  list.push_back( 2 );
  assert_eq!( list.len(), 2 );
}

#[ test ]
fn vec_deque_type_is_accessible()
{
  let mut deque = wtools::VecDeque::< i32 >::new();
  deque.push_back( 10 );
  deque.push_front( 5 );
  assert_eq!( deque[ 0 ], 5 );
  assert_eq!( deque[ 1 ], 10 );
}

#[ test ]
fn binary_heap_type_is_accessible()
{
  let mut heap = wtools::BinaryHeap::< i32 >::new();
  heap.push( 3 );
  heap.push( 7 );
  heap.push( 1 );
  assert_eq!( heap.peek(), Some( &7 ) );
}
