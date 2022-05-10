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
      pub
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::impls!
      {
        @DEFINE_FN
        @NAME $Name
        @VIS pub
        @REST
          $( #[ $Meta ] )*
          pub fn $Name
          $( $Rest )*
      }
    };

    (
      $( #[ $Meta : meta ] )*
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::impls!
      {
        @DEFINE_FN
        @NAME $Name
        @VIS
        @REST
          $( #[ $Meta ] )*
          fn $Name
          $( $Rest )*
      }
    };

    (
      @DEFINE_FN
      @NAME $Name : ident
      @VIS $( pub )*
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

//   ///
//   /// Index of items.
//   ///
//
//   #[ macro_export ]
//   macro_rules! index2
//   {
//
//     () => { };
//
//     (
//       $Name : ident ,
//       $( $Rest : tt )*
//     )
//     =>
//     {{
//       $Name!();
//       $crate::index2!( @ACT $( $Rest )* );
//     }};
//
//     ( @ACT ) => { };
//
//     (
//       @ACT
//       $Name : ident ,
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       $Name!();
//       $crate::index2!( @ACT $( $Rest )* );
//     };
//
//   }

  pub use impls;
  pub use index;
  // pub use index2;
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
  // pub use i::index2;
}
