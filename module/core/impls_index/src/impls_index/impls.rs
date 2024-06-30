/// Internal namespace.
pub( crate ) mod private
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
      $( , $( $Rest : tt )* )?
    )
    =>
    {
      $Name!( as $Alias );
      $crate::index!( $( $( $Rest )* )? );
    };

    (
      $Name : ident
      $( , $( $Rest : tt )* )?
    )
    =>
    {
      $Name!();
      $crate::index!( $( $( $Rest )* )? );
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

  // qqq : cover by tests
  // qqq : document the idea and module
  // qqq : add section idea to each module

  ///
  /// Define implementation putting each function under a macro.
  ///
  /// Use [index!] to generate code for each elment.
  /// Unlike elements of [impls_optional!], elements of [impls] are mandatory to be used in [index!].
  ///

  #[ macro_export ]
  macro_rules! impls_optional
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
      $crate::impls_optional!
      {
        @DefineFn
        @Meta{ $( #[ $Meta ] )* }
        @Vis{ $Vis }
        @Name{ $Name }
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
      @Rest
        $Item : item
        $( $Rest : tt )*
    )
    =>
    {
      #[ allow( unused_macros ) ]
      macro_rules! $Name
      {
        () =>
        {
          $Item
        };
      }

      $crate::impls_optional!
      {
        $( $Rest )*
      }
    };

  }
  ///
  /// Define implementation putting each function under a macro and adding attribute `#[ test ]`.
  ///
  /// Use [index!] to generate code for each elment.
  /// Unlike elements of [test_impls_optional!], elements of [test_impls] are mandatory to be used in [index!].
  ///

  #[ macro_export ]
  macro_rules! tests_impls
  {

    // empty

    // () => { type X = i32; };

    // empty

    () => {};

    // entry

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::tests_impls!
      {
        @DefineFn
        @Meta{ $( #[ $Meta ] )* }
        @Vis{ $Vis }
        @Name{ $Name }
        @Rest
          $( #[ $Meta ] )*
          $Vis fn $Name
          $( $Rest )*
      }
    };

    // parsed

    (
      @DefineFn
      @Meta{ $( #[ $Meta : meta ] )* }
      @Vis{ $Vis : vis }
      @Name{ $Name : ident }
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

      $crate::tests_impls!
      {
        $( $Rest )*
      }
    };

  }

  ///
  /// Define implementation putting each function under a macro and adding attribute `#[ test ]`.
  ///
  /// Use [index!] to generate code for each elment.
  /// Unlike elements of [test_impls!], elements of [test_impls_optional] are optional to be used in [index!].
  ///

  #[ macro_export ]
  macro_rules! tests_impls_optional
  {

    // empty

    // () => { type X = i32; };

    // empty

    () => {};

    // entry

    (
      $( #[ $Meta : meta ] )*
      $Vis : vis
      fn $Name : ident
      $( $Rest : tt )*
    )
    =>
    {
      $crate::tests_impls_optional!
      {
        @DefineFn
        @Meta{ $( #[ $Meta ] )* }
        @Vis{ $Vis }
        @Name{ $Name }
        @Rest
          $( #[ $Meta ] )*
          $Vis fn $Name
          $( $Rest )*
      }
    };

    // parsed

    (
      @DefineFn
      @Meta{ $( #[ $Meta : meta ] )* }
      @Vis{ $Vis : vis }
      @Name{ $Name : ident }
      @Rest
        $Item : item
        $( $Rest : tt )*
    )
    =>
    {
      #[ allow( unused_macros ) ]
      macro_rules! $Name
      {
        () =>
        {
          #[ test ]
          $Item
        };
      }

      $crate::tests_impls_optional!
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
  pub use impls_optional; /* qqq : write negative test. discuss please */
  pub use tests_impls;
  pub use tests_impls_optional; /* qqq : write negative test. discuss please */
  pub use impls2;
  pub use _impls_callback;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
}


/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    index,
    tests_index,
    impls1,
    impls_optional,
    tests_impls,
    tests_impls_optional,
    impls2,
    _impls_callback,
  };
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::impls_index_meta::impls3;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use impls3 as impls;
}
