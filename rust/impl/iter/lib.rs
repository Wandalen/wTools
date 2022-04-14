#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose tools to iterate. Currently it simply reexport itertools.
//!
// //! # Sample
// //! ```
// //! use iter_tools::*;
// //!
// //! fn main()
// //! {
// //!   /* standard functions */
// //!   let vec = vec![ 5, 1, -2 ];
// //!   let min = min( &vec );
// //!   assert_eq!( *min.unwrap(), -2 );
// //!
// //!   /* non standard functions */
// //!   let vec = vec![ 5, 1, -2 ];
// //!   let added = vec![ "a", "b", "c" ];
// //!   let mut result = vec![];
// //!   let zipped = zip( &vec, &added );
// //!   for ( left, right ) in zipped
// //!   {
// //!     result.push( ( *left, *right ) );
// //!   }
// //!   assert_eq!( result, vec![ ( 5, "a" ), ( 1, "b" ), ( -2, "c" ) ] );
// //! }
// //! ```
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// pub use itertools as itertools;

pub use itertools::
{
  Diff,
  Either,
  EitherOrBoth,
  FoldWhile,
  MinMaxResult,
  Position,
  Itertools,
  /*MultiUnzip,*/
  PeekingNext,
  all,
  any,
  assert_equal,
  chain,
  cloned,
  concat,
  cons_tuples,
  diff_with,
  enumerate,
  equal,
  fold,
  interleave,
  intersperse,
  intersperse_with,
  iterate,
  join,
  kmerge,
  kmerge_by,
  max,
  merge,
  merge_join_by,
  min,
  multipeek,
  multiunzip,
  multizip,
  partition,
  peek_nth,
  process_results,
  put_back,
  put_back_n,
  rciter,
  repeat_n,
  rev,
  sorted,
  unfold,
  zip,
  zip_eq,
};
