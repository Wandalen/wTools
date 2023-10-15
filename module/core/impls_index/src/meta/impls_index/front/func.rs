/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Get name of a function.
  ///

  #[ macro_export ]
  macro_rules! fn_name
  {

    (
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $Name
    };

    (
      $First : tt
      $( $Rest : tt )*
    )
    =>
    {
      $crate::fn_name!( $( $Rest )* );
    };

  }

  ///
  /// Macro to rename function.
  ///

  #[ macro_export ]
  macro_rules! fn_rename
  {

    (
      @Prefix { $( $Prefix : tt )* }
      @Name { $Name : ident }
      @Postfix
      {
        fn $OldName : ident
        $( $Postfix : tt )*
      }
    )
    =>
    {
      $( $Prefix )*
      fn $Name
      $( $Postfix )*
    };

    (
      @Prefix { $( $Prefix : tt )* }
      @Name { $Name : ident }
      @Postfix
      {
        $First : tt
        $( $Postfix : tt )*
      }
    )
    =>
    {
      $crate::fn_rename!
      {
        @Prefix { $( $Prefix )* }
        @Name { $Name }
        @Postfix { $( $Postfix )* }
      }
    };

    (
      @Name { $Name : ident }
      @Fn { $( $Fn : tt )* }
    )
    =>
    {
      $crate::fn_rename!
      {
        @Prefix {}
        @Name { $Name }
        @Postfix { $( $Fn )* }
      }
    };

  }

  ///
  /// Split functions.
  ///

  #[ macro_export ]
  macro_rules! fns
  {

    (
      @Callback { $Callback : path }
      @Rest
      {
        $( #[ $Meta : meta ] )*
        $Vis : vis
        fn $Name : ident
        $( < $( $ParamName : ident $( : $ParamType : path )? ),* $(,)? > )?
        ( $( $In : tt )* )
        $( -> $Out : ty )?
        $( where $( $WhereParamName : ident $( : $WhereParamType : path )? ),*  $(,)? )?
        $Block : block

        $( $Rest : tt )*
      }
    )
    =>
    {
      $Callback!
      {
        $( #[ $Meta ] )*
        $Vis
        fn $Name
        $( < $( $ParamName $( : $ParamType )? ),* > )?
        ( $( $In )* )
        $( -> $Out )?
        $( where $( $WhereParamName $( : $WhereParamType )? ),* )?
        $Block
      }
      $crate::fns!
      {
        @Callback { $Callback }
        @Rest
        {
          $( $Rest )*
        }
      }
    };

    (
      @Callback { $Callback : path }
      @Rest {}
    )
    =>
    {
    };

    (
      @Callback { $Callback : path }
      @Rest { $( $Rest : tt )* }
    )
    =>
    {
      compile_error!( concat!( "= Cant parse function\n", stringify!( $( $Rest )* ) ) );
    };

    (
      @Callback { $Callback : path }
      @Fns { $( $Fns : tt )* }
    )
    =>
    {
      $crate::fns!
      {
        @Callback { $Callback }
        // @Current {}
        @Rest { $( $Fns )* }
      }
    };

  }

  ///
  /// Split functions.
  ///

  #[ macro_export ]
  macro_rules! fns2
  {

    (
      @Callback { $Callback : path }
      @Rest
      {
        $( $Item : item )*
      }
    )
    =>
    {
      $(
        $Callback!
        {
          $Item
        }
      )*
    };

    (
      @Callback { $Callback : path }
      @Rest {}
    )
    =>
    {
    };

    (
      @Callback { $Callback : path }
      @Rest { $( $Rest : tt )* }
    )
    =>
    {
      compile_error!( concat!( "= Cant parse function\n", stringify!( $( $Rest )* ) ) );
    };

    (
      @Callback { $Callback : path }
      @Fns { $( $Fns : tt )* }
    )
    =>
    {
      $crate::fns2!
      {
        @Callback { $Callback }
        @Rest { $( $Fns )* }
      }
    };

  }

//   #[ macro_export ]
//   macro_rules! impls2
//   {
//
//     (
//       @SINGLE_FN1
//       $( $Token : tt )*
//     )
//     =>
//     {
//       $crate::impls2!( @SINGLE_FN2 $( $Token )* )
//     };
//
//     (
//       @SINGLE_FN2
//       $( #[ $Meta : meta ] )*
//       $Vis : vis
//       fn $Name : ident
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       compile_error!("yyy");
//       $crate::impls2!
//       {
//         @DefineFn
//         @Meta{ $( #[ $Meta ] )* }
//         @Vis{ $Vis }
//         @Name{ $Name }
//         @BEFORE_Name
//         {
//           $( #[ $Meta ] )*
//           $Vis fn
//         }
//         @AFTER_Name
//         {
//           $( $Rest : tt )*
//         }
//       }
//     };
//
//     (
//       @DefineFn
//       @Meta{ $( #[ $Meta : meta ] )* }
//       @Vis{ $Vis : vis }
//       @Name{ $Name : ident }
//       @BEFORE_Name
//       {
//         $( $Before : tt )*
//       }
//       @AFTER_Name
//       {
//         $( $After : tt )*
//       }
//     )
//     =>
//     {
//       // #[ deny( unused_macros ) ]
//       macro_rules! $Name
//       {
//         () =>
//         {
//           $Before
//           $Name
//           $After
//         };
//         // ( @AS $Name : ident ) =>
//         // {
//         //   $( #[ $Meta ] )*
//         //   fn $Name
//         // };
//       }
//     };
//
//     (
//       $( $Item : item )+
//     )
//     =>
//     {
//       $( $crate::impls2!( @SINGLE_FN1 $Item ) )+
//     };
//
//   }

//   ///
//   /// Index of items.
//   ///
//
//   #[ macro_export ]
//   macro_rules! ignore_macro
//   {
//
//     () => {};
//
//     (
//       $Name : ident ,
//       $( $Rest : tt )*
//     )
//     =>
//     {
//       $Name!();
//       stringify!( $crate::index!( $( $Rest )* ) );
//     };
//
//   }

  pub use fn_rename;
  pub use fn_name;
  pub use fns;
  pub use fns2;
  // pub use ignore_macro;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::fn_rename;
  #[ doc( inline ) ]
  pub use super::private::fn_name;
  #[ doc( inline ) ]
  pub use super::private::fns;
  #[ doc( inline ) ]
  pub use super::private::fns2;
  // pub use super::private::ignore_macro;
}
