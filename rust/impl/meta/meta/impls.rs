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
      $( pub )?
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::impls!
      {
        as DEFINE_FN
        as META $( #[ $Meta ] )*
        as VIS $( pub )?
        as NAME $Name
        as INPUT ()
        as OUTPUT
        as BLOCK {}
        as REST
          $( #[ $Meta ] )*
          $( pub )? fn $Name
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
    //     as DEFINE_FN
    //     as META $( #[ $Meta ] )*
    //     as VIS
    //     as NAME $Name
    //     as INPUT ()
    //     as OUTPUT
    //     as BLOCK {}
    //     as REST
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
//         as DEFINE_FN
//         as META $( #[ $Meta ] )*
//         as VIS pub
//         as NAME $Name
//         as INPUT $Input
//         as OUTPUT $( -> $Output )?
//         as BLOCK $Block
//         as REST
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
//         as DEFINE_FN
//         as META $( #[ $Meta ] )*
//         as VIS
//         as NAME $Name
//         as INPUT $Input
//         as OUTPUT $( -> $Output )?
//         as BLOCK $Block
//         as REST
//           $( #[ $Meta ] )*
//           fn $Name $Input $( -> $Output )?
//           $Block
//           $( $Rest )*
//       }
//     };

    (
      as DEFINE_FN
      as META $( #[ $Meta : meta ] )*
      as VIS $( pub )*
      as NAME $Name : ident
      as INPUT $Input : tt
      as OUTPUT $( -> $Output : ty )?
      as BLOCK $Block : block
      as REST
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
        // ( as AS $Name : ident ) =>
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
//       $crate::index2!( as ACT $( $Rest )* );
//     }};
//
//     ( as ACT ) => { };
//
//     (
//       as ACT
//       $Name : ident ,
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       $Name!();
//       $crate::index2!( as ACT $( $Rest )* );
//     };
//
//   }

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
