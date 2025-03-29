/// Define a private namespace for all its items.
mod private
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

  pub use fn_rename;
  pub use fn_name;
  pub use fns;
  pub use fns2;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::fn_rename;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::fn_name;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::fns;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::fns2;
  // pub use private::ignore_macro;
}
