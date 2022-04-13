#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Tools for writing procedural macroses.
//!
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

///
/// Internals.
///

#[ macro_use ]
pub mod internal
{

  ///
  /// Macro for diagnostics purpose to print both syntax tree and source code behind it.
  ///
  /// # Sample
  /// ```
  /// use proc_macro_tools::*;
  /// use quote::quote;
  ///
  /// let code = quote!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// tree_print!( tree_type );
  /// ```
  ///

  #[ macro_export ]
  // #[ macro_use ]
  macro_rules! _tree_print
  {
    ( $src : expr ) =>
    {{
      let result = $crate::tree_export_str!( $src );
      println!( "{}", result );
      result
    }};
    ( $( $src : expr ),+ $(,)? ) =>
    {{
      $( $crate::tree_print!( $src ) );+
    }};
  }

  ///
  /// Macro for diagnostics purpose to export both syntax tree and source code behind it into string.
  ///

  #[ macro_export ]
  // #[ macro_use ]
  macro_rules! _tree_export_str
  {
    ( $src : expr ) =>
    {{
      let src2 = &$src;
      format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
    }};
  }

  ///
  /// Macro to generate syn error either with span of a syntax tree element or with default one `proc_macro2::Span::call_site()`.
  ///
  /// # Sample
  /// ```
  /// # use proc_macro_tools::*;
  /// syn_err!( "No attr" );
  /// # ()
  /// ```
  ///

  #[ macro_export ]
  // #[ macro_use ]
  macro_rules! _syn_err
  {

    ( $msg : expr ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( _, $msg : expr ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), $msg )
    };
    ( $span : expr, $msg : expr ) =>
    {
      // syn::Error::new( ( $span ).span(), $msg )
      syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), $msg )
    };
    ( $span : expr, $msg : expr, $( $arg : expr ),+ ) =>
    {
      // syn::Error::new( ( $span ).span(), format!( $msg, $( $arg ),+ ) )
      syn::Error::new( syn::spanned::Spanned::span( &( $span ) ), format!( $msg, $( $arg ),+ ) )
    };
    ( _, $msg : expr, $( $arg : expr ),+ ) =>
    {
      syn::Error::new( proc_macro2::Span::call_site(), format!( $msg, $( $arg ),+ ) )
    };

  }

  ///
  /// Kind of container.
  ///

  #[derive( Debug, PartialEq, Copy, Clone )]
  pub enum ContainerKind
  {
    /// Not a container.
    No,
    /// Vector-like.
    Vector,
    /// Hash map-like.
    HashMap,
    /// Hash set-like.
    HashSet,
  }

  /// Return kind of container specified by type.
  ///
  /// Good to verify `alloc::vec::Vec< i32 >` is vector.
  /// Good to verify `std::collections::HashMap< i32, i32 >` is hash map.
  ///
  /// # Sample
  /// ```
  /// use proc_macro_tools::*;
  /// use quote::quote;
  ///
  /// let code = quote!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let kind = type_container_kind( &tree_type );
  /// assert_eq!( kind, ContainerKind::HashMap );
  /// ```

  pub fn type_container_kind( ty : &syn::Type ) -> ContainerKind
  {

    if let syn::Type::Path( path ) = ty
    {
      let last = &path.path.segments.last();
      if last.is_none()
      {
        return ContainerKind::No
      }
      match last.unwrap().ident.to_string().as_ref()
      {
        "Vec" => { return ContainerKind::Vector }
        "HashMap" => { return ContainerKind::HashMap }
        "HashSet" => { return ContainerKind::HashSet }
        _ => { return ContainerKind::No }
      }
    }
    ContainerKind::No
  }

  /// Return kind of container specified by type. Unlike [type_container_kind] it also understand optional types.
  ///
  /// Good to verify `Option< alloc::vec::Vec< i32 > >` is optional vector.
  ///
  /// # Sample
  /// ```
  /// use proc_macro_tools::*;
  /// use quote::quote;
  ///
  /// let code = quote!( Option< std::collections::HashMap< i32, i32 > > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let ( kind, optional ) = type_optional_container_kind( &tree_type );
  /// assert_eq!( kind, ContainerKind::HashMap );
  /// assert_eq!( optional, true );
  /// ```

  pub fn type_optional_container_kind( ty : &syn::Type ) -> ( ContainerKind, bool )
  {

    // use inspect_type::*;

    if type_rightmost( ty ) == Some( "Option".to_string() )
    {
      let ty2 = type_parameters( ty, 0 ..= 0 ).first().map( | e | *e );
      // inspect_type::inspect_type_of!( ty2 );
      if ty2.is_none()
      {
        return ( ContainerKind::No, false )
      }
      let ty2 = ty2.unwrap();
      return ( type_container_kind( ty2 ), true );
    }

    return ( type_container_kind( ty ), false );
  }

  /// Check is the rightmost item of path refering a type is specified type.
  ///
  /// Good to verify `core::option::Option< i32 >` is optional.
  /// Good to verify `alloc::vec::Vec< i32 >` is vector.
  ///
  /// # Sample
  /// ```
  /// use proc_macro_tools::*;
  /// use quote::quote;
  ///
  /// let code = quote!( core::option::Option< i32 > );
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
  /// # Sample
  /// ```
  /// use proc_macro_tools::*;
  /// use quote::quote;
  ///
  /// let code = quote!( core::option::Option< i8, i16, i32, i64 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let got = type_parameters( &tree_type, 0..=2 );
  /// got.iter().for_each( | e | println!( "{}", quote!( #e ) ) );
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
  /// # Sample
  /// ``` ignore
  /// let ( key, val, meta ) = attr_pair_single( &attr )?;
  /// ```

  pub fn attr_pair_single( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ), syn::Error >
  {
    use syn::spanned::Spanned;
    let meta = attr.parse_meta()?;

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

}

// ///
// /// Canonize path and return it in string format.
// ///
//
// pub fn path_of( syn_path : &syn::Path ) -> String
// {
//   // use quote::*;
//   use syn::*;
//   let result : String = format!( "{}", syn_path.to_token_stream() );
//   // let result : String = format!( "{}", quote!{ #syn_path } );
//   result
// }

//

// don't delete!
// good example of overloading to reuse
//
// pub use syn::spanned::Spanned;
//
// /// Trait to implement method span() for those structures which [module::syn](https://docs.rs/syn/latest/syn/spanned/index.html) do not have it implemented.
//
// pub trait Spanned2
// {
//   /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
//   fn span2( &self ) -> proc_macro2::Span;
// }
//
// //
//
// impl Spanned2 for syn::Data
// {
//   fn span2( &self ) -> proc_macro2::Span
//   {
//     // data_fields_of( &self ).span()
//     match self
//     {
//       syn::Data::Struct( syn::DataStruct { ref fields, .. } ) => fields.span(),
//       syn::Data::Enum( syn::DataEnum { ref variants, .. } ) => variants.span(),
//       syn::Data::Union( syn::DataUnion { ref fields, .. } ) => fields.span(),
//     }
//   }
// }
//
// impl< T : Spanned2 > Spanned2 for &T
// {
//   fn span2( &self ) -> proc_macro2::Span
//   {
//     ( *self ).span2()
//   }
// }
//
// //
//
// #[ doc( hidden ) ]
// pub struct Data< 'a, T >( &'a T );
//
// #[ doc( hidden ) ]
// pub trait Span1
// {
//   fn act( self ) -> proc_macro2::Span;
// }
//
// impl< 'a, T > Span1
// for Data< 'a, T >
// where T : syn::spanned::Spanned,
// {
//   fn act( self ) -> proc_macro2::Span
//   {
//     self.0.span()
//   }
// }
//
//
// #[ doc( hidden ) ]
// pub trait Span2
// {
//   fn act( self ) -> proc_macro2::Span;
// }
//
// impl< 'a, T > Span2
// for Data< 'a, T >
// where T : Spanned2,
// {
//   fn act( self ) -> proc_macro2::Span
//   {
//     self.0.span2()
//   }
// }
//
// #[ doc( hidden ) ]
// pub fn _span_of< T : Sized >( src : &T ) -> Data< T >
// {
//   Data( src )
// }
//
// // fn span2_of< T : Sized >( src : &T )
// // {
// //   _span_of( src ).act()
// // }
//
// /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
//
// #[ macro_export ]
// macro_rules! span_of
// {
//   ( $src : expr ) =>
//   {
//     $crate::_span_of( &$src ).act()
//   }
// }
//
// /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
// ///
// /// Works only for items for which span is not implemented in [module::syn](https://docs.rs/syn/latest/syn/spanned/index.html). For other use macro [`span_of!`](span_of!).
//
// pub fn span_of< Src : Spanned2 >( src : &Src ) -> proc_macro2::Span
// {
//   src.span2()
// }
//

// =

// repeat!{ ( pub use internal::, ) ( ; ) ( ; )
// {
//
//   ContainerKind
//
//   type_container_kind
//   type_optional_container_kind
//   type_rightmost
//   type_parameters
//   attr_pair_single
//
// }}

/* xxx : register error_tools */
/* xxx : alias werror -> error_tools  */
/* xxx : register text_tools as alias for wstring */

/* xxx : implement module::mod_at */
/* xxx : implement and publish mod_expose */

/* xxx : use skeptic? */
/* xxx : rename dt -> adt */

///
/// Dependencies of the module.
///

pub mod dependencies
{
  pub use syn;
  pub use proc_macro2;
}

// mod_expose!{
// {
//
//   _tree_print as tree_print,
//   _tree_export_str as tree_export_str,
//   _syn_err as syn_err,
//
//   ContainerKind,
//
//   type_container_kind,
//   type_optional_container_kind,
//   type_rightmost,
//   type_parameters,
//   attr_pair_single,
//
// }};

pub use _tree_print as tree_print;
pub use _tree_export_str as tree_export_str;
pub use _syn_err as syn_err;

pub use internal::ContainerKind;

pub use internal::type_container_kind;
pub use internal::type_optional_container_kind;
pub use internal::type_rightmost;
pub use internal::type_parameters;
pub use internal::attr_pair_single;

// pub type ContainerKind = internal::ContainerKind;
// pub fn type_container_kind                        ( ty : &syn::Type ) -> ContainerKind
// {
//   internal::type_container_kind( ty )
// }
// pub fn type_optional_container_kind               ( ty : &syn::Type ) -> ( ContainerKind, bool )
// {
//   internal::type_optional_container_kind( ty )
// }
// pub fn type_rightmost                             ( ty : &syn::Type ) -> Option< String >
// {
//   internal::type_rightmost( ty )
// }
// pub fn type_parameters< R >                       ( ty : &syn::Type, range : R ) -> Vec< &syn::Type >
// where
//   R : std::convert::Into< Interval >
// {
//   internal::type_parameters( ty, range )
// }
// pub fn attr_pair_single                           ( attr : &syn::Attribute ) -> Result< ( String, syn::Lit, syn::Meta ), syn::Error >
// {
//   internal::attr_pair_single( attr )
// }
