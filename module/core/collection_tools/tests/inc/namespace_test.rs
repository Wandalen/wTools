use super :: *;

// Verify all 8 collection types and all 16 constructor macros are accessible
// from both the crate root and the `exposed` re-export module.

#[ test ]
fn exposed_collection_types_accessible()
{
  let _v : the_module ::Vec< u32 >       = the_module ::collection ::Vec ::new();
  let _v : the_module ::Vec< u32 >       = the_module ::exposed ::collection ::Vec ::new();
  let _m : the_module ::HashMap< u32, u32 >  = the_module ::collection ::HashMap ::new();
  let _m : the_module ::HashMap< u32, u32 >  = the_module ::exposed ::collection ::HashMap ::new();
  let _s : the_module ::HashSet< u32 >   = the_module ::collection ::HashSet ::new();
  let _s : the_module ::HashSet< u32 >   = the_module ::exposed ::collection ::HashSet ::new();
  let _m : the_module ::BTreeMap< u32, u32 > = the_module ::collection ::BTreeMap ::new();
  let _m : the_module ::BTreeMap< u32, u32 > = the_module ::exposed ::collection ::BTreeMap ::new();
  let _s : the_module ::BTreeSet< u32 >  = the_module ::collection ::BTreeSet ::new();
  let _s : the_module ::BTreeSet< u32 >  = the_module ::exposed ::collection ::BTreeSet ::new();
  let _l : the_module ::LinkedList< u32 > = the_module ::collection ::LinkedList ::new();
  let _l : the_module ::LinkedList< u32 > = the_module ::exposed ::collection ::LinkedList ::new();
  let _d : the_module ::VecDeque< u32 >  = the_module ::collection ::VecDeque ::new();
  let _d : the_module ::VecDeque< u32 >  = the_module ::exposed ::collection ::VecDeque ::new();
  let _h : the_module ::BinaryHeap< u32 > = the_module ::collection ::BinaryHeap ::new();
  let _h : the_module ::BinaryHeap< u32 > = the_module ::exposed ::collection ::BinaryHeap ::new();
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn strict_macros_accessible_from_root()
{
  let _v = the_module ::vec!{ 1u32, 2, 3 };
  let _m = the_module ::hmap!{ 1u32 => "a", 2 => "b" };
  let _s = the_module ::hset!{ 1u32, 2, 3 };
  let _m = the_module ::bmap!{ 1u32 => "a", 2 => "b" };
  let _s = the_module ::bset!{ 1u32, 2, 3 };
  let _l = the_module ::llist!{ 1u32, 2, 3 };
  let _d = the_module ::deque!{ 1u32, 2, 3 };
  let _h = the_module ::heap!{ 1u32, 2, 3 };
}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn strict_macros_accessible_from_exposed()
{
  // vec is re-exported as dlist in the exposed module
  let _v = the_module ::exposed ::dlist!{ 1u32, 2, 3 };
  let _m = the_module ::exposed ::hmap!{ 1u32 => "a", 2u32 => "b" };
  let _s = the_module ::exposed ::hset!{ 1u32, 2u32, 3u32 };
  let _m = the_module ::exposed ::bmap!{ 1u32 => "a", 2u32 => "b" };
  let _s = the_module ::exposed ::bset!{ 1u32, 2u32, 3u32 };
  let _l = the_module ::exposed ::llist!{ 1u32, 2u32, 3u32 };
  let _d = the_module ::exposed ::deque!{ 1u32, 2u32, 3u32 };
  let _h = the_module ::exposed ::heap!{ 1u32, 2u32, 3u32 };
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_macros_accessible_from_root()
{
  let _v  : the_module ::Vec< u32 >           = the_module ::into_vec!{ 1u32, 2u32, 3u32 };
  let _vd : the_module ::Vec< u32 >           = the_module ::into_dlist!{ 1u32, 2u32, 3u32 };
  let _m  : the_module ::HashMap< u32, &str > = the_module ::into_hmap!{ 1u32 => "a", 2u32 => "b" };
  let _s  : the_module ::HashSet< u32 >       = the_module ::into_hset!{ 1u32, 2u32, 3u32 };
  let _m2 : the_module ::BTreeMap< u32, &str > = the_module ::into_bmap!{ 1u32 => "a", 2u32 => "b" };
  let _s2 : the_module ::BTreeSet< u32 >      = the_module ::into_bset!{ 1u32, 2u32, 3u32 };
  let _l  : the_module ::LinkedList< u32 >    = the_module ::into_llist!{ 1u32, 2u32, 3u32 };
  let _d  : the_module ::VecDeque< u32 >      = the_module ::into_vecd!{ 1u32, 2u32, 3u32 };
  let _h  : the_module ::BinaryHeap< u32 >    = the_module ::into_heap!{ 1u32, 2u32, 3u32 };
}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_macros_accessible_from_exposed()
{
  let _v  : the_module ::Vec< u32 >            = the_module ::exposed ::into_vec!{ 1u32, 2u32, 3u32 };
  let _vd : the_module ::Vec< u32 >            = the_module ::exposed ::into_dlist!{ 1u32, 2u32, 3u32 };
  let _m  : the_module ::HashMap< u32, &str >  = the_module ::exposed ::into_hmap!{ 1u32 => "a", 2u32 => "b" };
  let _s  : the_module ::HashSet< u32 >        = the_module ::exposed ::into_hset!{ 1u32, 2u32, 3u32 };
  let _m2 : the_module ::BTreeMap< u32, &str > = the_module ::exposed ::into_bmap!{ 1u32 => "a", 2u32 => "b" };
  let _s2 : the_module ::BTreeSet< u32 >       = the_module ::exposed ::into_bset!{ 1u32, 2u32, 3u32 };
  let _l  : the_module ::LinkedList< u32 >     = the_module ::exposed ::into_llist!{ 1u32, 2u32, 3u32 };
  let _d  : the_module ::VecDeque< u32 >       = the_module ::exposed ::into_vecd!{ 1u32, 2u32, 3u32 };
  let _h  : the_module ::BinaryHeap< u32 >     = the_module ::exposed ::into_heap!{ 1u32, 2u32, 3u32 };
}
