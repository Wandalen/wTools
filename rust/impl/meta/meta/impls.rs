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
      $Vis : vis
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::impls!
      {
        @DEFINE_FN
        @META $( #[ $Meta ] )*
        @VIS{ $Vis }
        @NAME $Name
        // @INPUT ()
        // @OUTPUT
        // @BLOCK {}
        @REST
          $( #[ $Meta ] )*
          $Vis fn $Name
          $( $Rest )*
      }
    };

    // (
    //   $( #[ $Meta : meta ] )*
    //   fn $Name : ident
    //   // $( < $( $ParamName : ident : $ParamType : ty ),* > )?
    //   // $Input : tt
    //   // $( -> $Output : ty )?
    //   // $Block : block
    //   $( $Rest : tt )*
    // )
    // =>
    // {
    //   $crate::impls!
    //   {
    //     @DEFINE_FN
    //     @META $( #[ $Meta ] )*
    //     @VIS
    //     @NAME $Name
    //     @INPUT ()
    //     @OUTPUT
    //     @BLOCK {}
    //     @REST
    //       $( #[ $Meta ] )*
    //       fn $Name
    //       // $Input $( -> $Output )?
    //       // $Block
    //       $( $Rest )*
    //   }
    // };

//     (
//       $( #[ $Meta : meta ] )*
//       pub
//       fn $Name : ident
//       $Input : tt
//       $( -> $Output : ty )?
//       $Block : block
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       $crate::impls!
//       {
//         @DEFINE_FN
//         @META $( #[ $Meta ] )*
//         @VIS pub
//         @NAME $Name
//         @INPUT $Input
//         @OUTPUT $( -> $Output )?
//         @BLOCK $Block
//         @REST
//           $( #[ $Meta ] )*
//           pub fn $Name $Input $( -> $Output )?
//           $Block
//           $( $Rest )*
//       }
//     };
//
//     (
//       $( #[ $Meta : meta ] )*
//       fn $Name : ident
//       $Input : tt
//       $( -> $Output : ty )?
//       $Block : block
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       $crate::impls!
//       {
//         @DEFINE_FN
//         @META $( #[ $Meta ] )*
//         @VIS
//         @NAME $Name
//         @INPUT $Input
//         @OUTPUT $( -> $Output )?
//         @BLOCK $Block
//         @REST
//           $( #[ $Meta ] )*
//           fn $Name $Input $( -> $Output )?
//           $Block
//           $( $Rest )*
//       }
//     };

    (
      @DEFINE_FN
      @META $( #[ $Meta : meta ] )*
      @VIS{ $Vis : vis }
      @NAME $Name : ident
      // @INPUT $Input : tt
      // @OUTPUT $( -> $Output : ty )?
      // @BLOCK $Block : block
      @REST
        $Item : item
        $( $Rest : tt )*
    )
    =>
    {
      // #[ deny( unused_macros ) ]
      macro_rules! $Name
      {
        () =>
        {
          $Item
        };
        // ( @AS $Name : ident ) =>
        // {
        //   $( #[ $Meta ] )*
        //   fn $Name
        // };
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

  ///
  /// Index of items.
  ///

  #[ macro_export ]
  macro_rules! ignore_macro
  {

    () => { };

    (
      $Name : ident ,
      $( $Rest : tt )*
    )
    =>
    {
      $Name!();
      stringify!( $crate::index!( $( $Rest )* ) );
    };

  }

  pub use impls;
  pub use index;
  pub use ignore_macro;
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
  pub use i::ignore_macro;
}
