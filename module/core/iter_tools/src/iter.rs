
#[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{
  use ::itertools::process_results;

  // fn collect_results< I, T, E >( iter : I ) -> core::result::Result< Vec< T >, E >
  // where
  //   I : Iterator< Item = core::result::Result< T, E > > + Clone,
  //   E : core::fmt::Debug,
  // {
  //   for e in iter.clone()
  //   {
  //     if e.is_err()
  //     {
  //       e?;
  //     }
  //   }
  //   Ok( iter.map( | e | e.unwrap() ).collect() )
  // }

  /// Extension of iterator.

  pub trait IterExt
  where
    Self : core::iter::Iterator,
  {
    /// Iterate each element and return `core::Result::Err` if any element is error.
    fn map_result< F, RE, El >( self, f : F ) -> core::result::Result< Vec< El >, RE >
    where
      Self : Sized + Clone,
      // Self : Sized,
      F : FnMut( < Self as core::iter::Iterator >::Item ) -> core::result::Result< El, RE >,
      RE : core::fmt::Debug,
      // El : Clone,
      // core::result::Result< El, RE > : Clone,
    ;
  }

  impl< Iterator > IterExt for Iterator
  where
    Iterator : core::iter::Iterator,
  {
    fn map_result< F, RE, El >( self, f : F ) -> core::result::Result< Vec< El >, RE >
    where
      Self : Sized + Clone,
      F : FnMut( < Self as core::iter::Iterator >::Item ) -> core::result::Result< El, RE >,
      RE : core::fmt::Debug,
      // El : Clone,
      // core::result::Result< El, RE > : Clone,
    {
      let vars_maybe = self.map( f );
      let vars : Vec< _ > = process_results( vars_maybe, | iter | iter.collect() )?;
      // let vars = collect_results( vars_maybe.clone() )?;
      Ok( vars )
      // let result : ( Vec< _ >, Vec< _ >, Vec< _ > )
      // = vars.into_iter().multiunzip();
      // Ok( result )
    }
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
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

  #[ cfg( not( feature = "no_std" ) ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use std::iter::zip;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
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

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  pub use super::private::IterExt;

}
