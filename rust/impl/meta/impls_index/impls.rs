/// Internal namespace.
pub mod internal
{

  ///
  /// Internal impls macro. Don't use.
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
      macro_rules! $Name
      {
        ( as $Name2 : ident ) =>
        {
          $crate::fn_rename!{ @Name { $Name2 } @FN
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
  /// Define implementation putting each function under a macro.
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

  pub use _impls_callback;
  pub use impls2;
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
  pub use i::_impls_callback;
  pub use i::impls2;
  pub use i::impls;
  pub use i::index;
}
