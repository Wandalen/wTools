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

/// [BTreeMap] macros
pub mod bmap;
/// [BTreeSet] macros
pub mod bset;
/// [BinaryHeap] macros
pub mod heap;
/// [HashMap] macros
pub mod hmap;
/// [HashSet] macros
pub mod hset;
/// [LinkedList] macros
pub mod list;
/// [Vec] macros
pub mod vec;
/// [VecDeque] macros
pub mod vecd;
