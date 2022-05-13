/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;

  pub use ::itertools::
  {
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

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{

  pub use ::itertools::
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
  };

}
