use super::*;

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn b_tree_map()
{
  let mut map : the_module::BTreeMap< i32, i32 > = the_module::BTreeMap::new();
  map.insert( 1, 2 );
  let exp = 2;
  let got = *map.get( &1 ).unwrap();
  assert_eq!( exp, got );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn b_tree_set()
{
  let mut map : the_module::BTreeSet< i32 > = the_module::BTreeSet::new();
  map.insert( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn binary_heap()
{
  let mut map : the_module::BinaryHeap< i32 > = the_module::BinaryHeap::new();
  map.push( 1 );
  let exp = Some(1).as_ref();
  let got = map.peek();
  assert_eq!( exp, got );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn hash_map()
{
  let mut map : the_module::HashMap< i32, i32 > = the_module::HashMap::new();
  map.insert( 1, 2 );
  let exp = 2;
  let got = *map.get( &1 ).unwrap();
  assert_eq!( exp, got );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn hash_set()
{
  let mut map : the_module::HashSet< i32 > = the_module::HashSet::new();
  map.insert( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn linked_list()
{
  let mut map : the_module::LinkedList< i32 > = the_module::LinkedList::new();
  map.push_back( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );
}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn vec()
{

  let mut map : the_module::Vec< i32 > = the_module::Vec::new();
  map.push( 1 );
  map.push( 2 );
  let got = map.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = map.last().unwrap().clone();
  assert_eq!( got, 2 );

}

//

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn vec_deque()
{
  let mut map : the_module::VecDeque< i32 > = the_module::VecDeque::new();
  map.push_back( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );
}
