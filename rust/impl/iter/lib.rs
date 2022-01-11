#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Collection of general purpose tools to iterate. Currently it simply reexport itertools.
//!

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