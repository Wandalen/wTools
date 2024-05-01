#[ allow( unused_imports ) ]
use super::*;

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn b_tree_map()
{

  // test.case( "empty" );
  let got : the_module::BTreeMap< i32, i32 > = the_module::into_bmap!{};
  let exp = the_module::BTreeMap::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_bmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::BTreeMap::new();
  exp.insert(3, 13);
  exp.insert(4, 1);
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn b_tree_set()
{

  // test.case( "empty" );
  let got : the_module::BTreeSet< i32 > = the_module::into_bset!{};
  let exp = the_module::BTreeSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_bset!{ 3, 13 };
  let mut exp = the_module::BTreeSet::new();
  exp.insert(3);
  exp.insert(13);
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn binary_heap()
{

  // test.case( "empty" );
  let got : the_module::BinaryHeap< i32 > = the_module::into_heap!{};
  let exp = the_module::BinaryHeap::new();
  assert_eq!( got.into_vec(), exp.into_vec() );

  // test.case( "multiple entry" );
  let got : the_module::BinaryHeap< i32 > = the_module::into_heap!{ 3, 13 };
  let mut exp = the_module::BinaryHeap::new();
  exp.push(3);
  exp.push(13);
  assert_eq!( got.into_sorted_vec(), exp.into_sorted_vec() );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn hash_map()
{

  // test.case( "empty" );
  let got : the_module::HashMap< i32, i32 > = the_module::into_hmap!{};
  let exp = the_module::HashMap::new();
  assert_eq!( got, exp );


  // test.case( "multiple entry" );
  let got = the_module::into_hmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::HashMap::new();
  exp.insert( 3, 13 );
  exp.insert( 4, 1 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn hash_set()
{

  // test.case( "empty" );
  let got : the_module::HashSet< i32 > = the_module::into_hset!{};
  let exp = the_module::HashSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_hset!{ 13, 11 };
  let mut exp = the_module::HashSet::new();
  exp.insert( 11 );
  exp.insert( 13 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn linked_list()
{

  // test.case( "empty" );
  let got : the_module::LinkedList< i32 > = the_module::into_list!{};
  let exp = the_module::LinkedList::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_list!{ 13, 15 };
  let mut exp = the_module::LinkedList::new();
  exp.push_front( 15 );
  exp.push_front( 13 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn vec()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{};
  let exp = the_module::Vec::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn vec_deque()
{

  // test.case( "empty" );
  let got : the_module::VecDeque< i32 > = the_module::into_vecd!{};
  let exp = the_module::VecDeque::new();
  assert_eq!( got, exp );

  // test.case( "single entry" );
  let got = the_module::into_vecd!{ 3, 13 };
  let mut exp = the_module::VecDeque::new();
  exp.push_front( 13 );
  exp.push_front( 3 );
  assert_eq!( got, exp );

}
