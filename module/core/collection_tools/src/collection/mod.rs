/// Not meant to be called directly.
#[ doc( hidden ) ]
#[ macro_export( local_inner_macros ) ]
macro_rules! count
{
  ( @single $( $x : tt )* ) => ( () );

  (
    @count $( $rest : expr ),*
  )
  =>
  (
    < [ () ] >::len( &[ $( count!( @single $rest ) ),* ] )
  );
}

#[ cfg( feature = "enabled" ) ]
#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
extern crate alloc;

/// [std::collections::BTreeMap] macros
pub mod btree_map;
/// [std::collections::BTreeSet] macros
pub mod btree_set;
/// [std::collections::BinaryHeap] macros
pub mod binary_heap;
/// [std::collections::HashMap] macros
pub mod hash_map;
/// [std::collections::HashSet] macros
pub mod hash_set;
/// [std::collections::LinkedList] macros
pub mod linked_list;
/// [Vec] macros
pub mod vector;
/// [std::collections::VecDeque] macros
pub mod vec_deque;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  pub use super::
  {
    btree_map,
    btree_set,
    binary_heap,
    hash_map,
    hash_set,
    linked_list,
    vector,
    vec_deque,
  };

  #[ doc( inline ) ]
  pub use orphan::*;

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use super::super::collection;

  #[ doc( inline ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ cfg( feature = "collection_constructors" ) ]
  pub use crate::
  {
    vec as dlist,
    deque,
    llist,
    hset,
    hmap,
    bmap,
    bset,
  };

  #[ doc( inline ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ cfg( feature = "collection_into_constructors" ) ]
  pub use crate::
  {
    into_vec,
    into_vec as into_dlist,
    into_vecd,
    into_llist,
    into_hset,
    into_hmap,
    into_bmap,
    into_bset,
  };

  // #[ cfg( feature = "reexports" ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  pub use
  {
    btree_map::BTreeMap,
    btree_set::BTreeSet,
    binary_heap::BinaryHeap,
    hash_map::HashMap,
    hash_set::HashSet,
    linked_list::LinkedList,
    vector::Vec,
    vec_deque::VecDeque,
  };

  // #[ cfg( feature = "reexports" ) ]
  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  pub use
  {
    LinkedList as Llist,
    Vec as Dlist,
    VecDeque as Deque,
    HashMap as Map,
    HashMap as Hmap,
    HashSet as Set,
    HashSet as Hset,
    BTreeMap as Bmap,
    BTreeSet as Bset,
  };

  // qqq : cover by tests presence of all containers immidiately in collection_tools::* and in collection_tools::exposed::*

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
