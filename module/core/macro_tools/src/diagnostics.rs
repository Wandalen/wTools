//!
//! Macro helpers.
//!

/// Internal namespace.
pub( crate ) mod private
{
  // pub use winterval::exposed::*;

  ///
  /// Result with syn::Error.
  ///

  pub type Result< T > = std::result::Result< T, syn::Error >;

  ///
  /// Macro for diagnostics purpose to print both syntax tree and source code behind it with syntax tree.
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::prelude::*;
  ///
  /// let code = qt!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// tree_print!( tree_type );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! tree_print
  {
    ( $src:expr ) =>
    {{
      let result = $crate::tree_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src:expr ),+ $(,)? ) =>
    {{
      $( $crate::tree_print!( $src ) );+
    }};
  }

  ///
  /// Macro for diagnostics purpose to print both syntax tree and source code behind it without syntax tree.
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::prelude::*;
  ///
  /// let code = qt!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// tree_print!( tree_type );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! code_print
  {
    ( $src:expr ) =>
    {{
      let result = $crate::code_diagnostics_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src:expr ),+ $(,)? ) =>
    {{
      $( $crate::code_print!( $src ) );+
    }};
  }

  ///
  /// Macro for diagnostics purpose to export both syntax tree and source code behind it into a string.
  ///

  #[ macro_export ]
  macro_rules! tree_diagnostics_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{} : {} :\n{:#?}", stringify!( $src ), $crate::qt!{ #src2 }, $src )
    }};
  }

  ///
  /// Macro for diagnostics purpose to diagnose source code behind it and export it into a string.
  ///

  #[ macro_export ]
  macro_rules! code_diagnostics_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{} : {}", stringify!( $src ), $crate::qt!{ #src2 } )
    }};
  }

  ///
  /// Macro to export source code behind a syntax tree into a string.
  ///

  #[ macro_export ]
  macro_rules! code_to_str
  {
    ( $src:expr ) =>
    {{
      let src2 = &$src;
      format!( "{}", $crate::qt!{ #src2 } )
    }};
  }

  ///
  /// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
  ///
  /// ### Basic use-case.
  /// ```
  /// # use macro_tools::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  macro_rules! syn_err
  {

    ( $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( _, $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( $span:expr, $msg:expr $(,)? ) =>
    {
      $crate::syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), $msg )
    };
    ( $span:expr, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      $crate::syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), format!( $msg, $( $arg ),+ ) )
    };
    ( _, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      $crate::syn::Error::new( proc_macro2::Span::call_site(), format!( $msg, $( $arg ),+ ) )
    };

  }

  ///
  /// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
  ///
  /// ### Basic use-case.
  /// ```
  /// # use macro_tools::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  macro_rules! return_syn_err
  {
    ( $( $Arg : tt )* ) =>
    {
      $crate::syn_err!( $( $Arg )* )
    };
  }

  pub use
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_to_str,
    syn_err,
  };

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    Result,
    // type_rightmost,
    // type_parameters,
    // eq_pair,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_to_str,
    syn_err,
  };

  // #[ doc( inline ) ]
  // pub use super::private::Result;
}
