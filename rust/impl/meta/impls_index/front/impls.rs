/// Internal namespace.
mod internal
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
        // @Input{ () }
        // @Output{}
        // @Block{ {} }
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
      // @Input{ $Input : tt }
      // @Output{ $( -> $Output : ty )? }
      // @Block{ $Block : block }
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

  // xxx : cover by tests
  // xxx : document the idea and module
  // xxx : add section idea to each module

  ///
  /// Define implementation putting each function under a macro and adding attribute `#[ test ]`.
  ///

  #[ macro_export ]
  macro_rules! tests_impls
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
        // @Input{ () }
        // @Output{}
        // @Block{ {} }
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
      // @Input{ $Input : tt }
      // @Output{ $( -> $Output : ty )? }
      // @Block{ $Block : block }
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
          #[ test ]
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
  pub use index as tests_index;
  pub use impls1;
  pub use tests_impls;
  pub use impls2;
  pub use _impls_callback;

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::internal::
  {
    index,
    tests_index,
    impls1,
    tests_impls,
    impls2,
    _impls_callback,
  };
  pub use ::impls_index_meta::impls3;
  pub use impls1 as impls;
}
