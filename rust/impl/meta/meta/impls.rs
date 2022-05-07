/// Internal namespace.
pub mod internal
{

  ///
  /// Internals of a test suite.
  ///

  #[ macro_export ]
  macro_rules! impls
  {

    () => {};

    (
      $( #[ $Meta : meta ] )*
      fn $Name : ident $( $Rest : tt )*
    )
    =>
    {
      $crate::impls!
      {
        @DEFINE_FN
        @NAME $Name
        @REST
          $( #[ $Meta ] )*
          fn $Name $( $Rest )*
      }
    };

    (
      @DEFINE_FN
      @NAME $Name : ident
      @REST
        $Item : item
        $( $Rest : tt )*
    )
    =>
    {
      #[ deny( unused_macros ) ]
      macro_rules! $Name
      {
        () =>
        {
          $Item
        };
      }

      $crate::impls!
      {
        $( $Rest )*
      }
    };

  }

  ///
  /// Index of items.
  ///

  #[ macro_export ]
  macro_rules! index
  {

    () => { };

    (
      $Name : ident ,
      $( $Rest : tt )*
    )
    =>
    {
      $Name!();
      $crate::index!( $( $Rest )* );
    };

  }

  pub use impls;
  pub use index;
}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::impls;
  pub use i::index;
}
