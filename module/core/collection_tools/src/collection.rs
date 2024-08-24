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

/// [std::collections::BTreeMap] macros
pub mod bmap;
/// [std::collections::BTreeSet] macros
pub mod bset;
/// [std::collections::BinaryHeap] macros
pub mod heap;
/// [std::collections::HashMap] macros
pub mod hmap;
/// [std::collections::HashSet] macros
pub mod hset;
/// [std::collections::LinkedList] macros
pub mod llist;
/// [Vec] macros
pub mod vec;
/// [std::collections::VecDeque] macros
pub mod deque;
