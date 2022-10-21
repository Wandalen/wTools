/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
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
    // zip,
    zip_eq,
  };

  #[ doc( inline ) ]
  pub use core::iter::zip;

}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
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
