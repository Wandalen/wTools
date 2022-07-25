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
  /// ### Sample
  /// ```
  /// use proc_macro_tools::prelude::*;
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
  /// ### Sample
  /// ```
  /// use proc_macro_tools::prelude::*;
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
  macro_rules! code_export_str
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
  /// ### Sample
  /// ```
  /// # use proc_macro_tools::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  macro_rules! syn_err
  {

    ( $msg:expr $(,)? ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( _, $msg:expr $(,)? ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( $span:expr, $msg:expr $(,)? ) =>
    {
      syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), $msg )
    };
    ( $span:expr, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), format!( $msg, $( $arg ),+ ) )
    };
    ( _, $msg:expr, $( $arg:expr ),+ $(,)? ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), format!( $msg, $( $arg ),+ ) )
    };

  }

  /// Check is the rightmost item of path refering a type is specified type.
  ///
  /// Good to verify `core::option::Option< i32 >` is optional.
  /// Good to verify `alloc::vec::Vec< i32 >` is vector.
  ///
  /// ### Sample
  /// ```
  /// use proc_macro_tools::*;
  ///
  /// let code = qt!( core::option::Option< i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let got = type_rightmost( &tree_type );
  /// assert_eq!( got, Some( "Option".to_string() ) );
  /// ```

  pub fn type_rightmost( ty : &syn::Type ) -> Option< String >
  {
    if let syn::Type::Path( path ) = ty
    {
      let last = &path.path.segments.last();
      if last.is_none()
      {
        return None;
      }
      return Some( last.unwrap().ident.to_string() );
    }
    None
  }

  use winterval::*;

  /// Return the specified number of parameters of the type.
  ///
  /// Good to getting `i32` from `core::option::Option< i32 >` or `alloc::vec::Vec< i32 >`
  ///
  /// ### Sample
  /// ```
  /// use proc_macro_tools::*;
  ///
  /// let code = qt!( core::option::Option< i8, i16, i32, i64 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let got = type_parameters( &tree_type, 0..=2 );
  /// got.iter().for_each( | e | println!( "{}", qt!( #e ) ) );
  /// // < i8
  /// // < i16
  /// // < i32
  /// ```

  pub fn type_parameters< R >( ty : &syn::Type, range : R ) -> Vec< &syn::Type >
  where
    R : std::convert::Into< Interval >
  {
    let range = range.into();
    if let syn::Type::Path( syn::TypePath{ path : syn::Path { ref segments, .. }, .. } ) = ty
    {
      let last = &segments.last();
      if last.is_none()
      {
        return vec![ &ty ]
      }
      let args = &last.unwrap().arguments;
      if let syn::PathArguments::AngleBracketed( ref args2 ) = args
      {
        let args3 = &args2.args;
        let selected : Vec< &syn::Type > = args3
        .iter()
        .skip_while( | e | if let syn::GenericArgument::Type( _ ) = e { false } else { true } )
        .skip( range.first().try_into().unwrap() )
        .take( range.len().try_into().unwrap() )
        .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
        .collect();
        return selected;
      }
    }
    vec![ &ty ]
  }

  ///
  /// For attribute like `#[former( default = 31 )]` return key `default` and value `31`,
  /// as well as syn::Meta as the last element of result tuple.
  ///
  /// ### Sample
  /// ``` ignore
  /// let ( key, val, meta ) = attr_pair_single( &attr )?;
  /// ```

  pub fn attr_pair_single( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ) >
  {
    use syn::spanned::Spanned;
    let meta = attr.parse_meta()?;

    // zzz : try to use helper from toolbox
    let ( key, val );
    match meta
    {
      syn::Meta::List( ref meta_list ) =>
      match meta_list.nested.first()
      {
        Some( nested_meta ) => match nested_meta
        {
          syn::NestedMeta::Meta( meta2 ) => match meta2
          {
            syn::Meta::NameValue( name_value ) => // match &name_value.lit
            {
              if meta_list.nested.len() != 1
              {
                return Err( syn::Error::new( attr.span(), format!( "Expected single element of the list, but got {}", meta_list.nested.len() ) ) );
              }
              key = name_value.path.get_ident().unwrap().to_string();
              val = name_value.lit.clone();
            },
            _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::NameValue( name_value )" ) ),
          },
          _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::NestedMeta::Meta( meta2 )" ) ),
        },
        _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected Some( nested_meta )" ) ),
      },
      _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
    };

    Ok( ( key, val, meta ) )
  }

  pub use
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_export_str,
    syn_err,
  };

}

#[ doc( inline ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  pub use super::private::
  {
    type_rightmost,
    type_parameters,
    attr_pair_single,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  pub use super::private::
  {
    tree_print,
    code_print,
    tree_diagnostics_str,
    code_diagnostics_str,
    code_export_str,
    syn_err,
  };

  #[ doc( inline ) ]
  pub use super::private::Result;
}
