/// Internal namespace.
pub mod internal
{

  ///
  /// Index of items.
  ///

  #[ macro_export ]
  macro_rules! index
  {

    () => { };

    (
      $Name : ident as $Alias : ident,
      $( $Rest : tt )*
    )
    =>
    {
      $Name!( as $Alias );
      $crate::index!( $( $Rest )* );
    };

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
  /// Define implementation putting each function under a macro.
  ///

  #[ macro_export ]
  macro_rules! impls1
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
      $crate::impls1!
      {
        @DefineFn
        @Meta{ $( #[ $Meta ] )* }
        @Vis{ $Vis }
        @Name{ $Name }
        @Input{ () }
        @Output{}
        @Block{ {} }
        @Rest
          $( #[ $Meta ] )*
          $Vis fn $Name
          $( $Rest )*
      }
    };

    (
      @DefineFn
      @Meta{ $( #[ $Meta : meta ] )* }
      @Vis{ $Vis : vis }
      @Name{ $Name : ident }
      @Input{ $Input : tt }
      @Output{ $( -> $Output : ty )? }
      @Block{ $Block : block }
      @Rest
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

      $crate::impls1!
      {
        $( $Rest )*
      }
    };

  }

  ///
  /// Define implementation putting each function under a macro.
  ///

  #[ macro_export ]
  macro_rules! impls2
  {

    (
      $( $Rest : tt )*
    )
    =>
    {
      $crate::fns!
      {
        @Callback { $crate::_impls_callback }
        @Fns { $( $Rest )* }
      }
    };

  }

  ///
  /// Internal impls1 macro. Don't use.
  ///

  #[ macro_export ]
  macro_rules! _impls_callback
  {

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis
      fn $Name : ident
      $( $Rest : tt )*
    ) =>
    {
      #[ deny( unused_macros ) ]
      macro_rules! $Name
      {
        ( as $Name2 : ident ) =>
        {
          $crate::fn_rename!{ @Name { $Name2 } @Fn
          {
            $( #[ $Meta ] )*
            $Vis
            fn $Name
            $( $Rest )*
          }}
        };
        () =>
        {
          $( #[ $Meta ] )*
          $Vis
          fn $Name
          $( $Rest )*
        };
      }
    };

  }

  pub use index;
  pub use impls1;
  pub use impls2;
  pub use _impls_callback;

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
  pub use i::index;
  pub use i::impls1;
  pub use i::impls2;
  pub use i::_impls_callback;
  pub use ::impls_index_meta::impls3;
  pub use impls1 as impls;
}
