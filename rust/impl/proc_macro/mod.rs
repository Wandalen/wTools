#![ warn( missing_docs ) ]

//!
//! Tools for writing procedural macroses.
//!

/// Macro for diagnostics purpose to print both syntax tree and source code behind it.

#[ macro_export ]
macro_rules! tree_print
{
  ( $src : expr ) =>
  {{
    println!( "{}", tree_export_str!( $src ) );
  }};
  ( $( $src : expr ),+ $(,)? ) =>
  {{
    $( tree_print!( $src ) );+;
  }};
}

/// Macro for diagnostics purpose to export both syntax tree and source code behind it into string.

#[ macro_export ]
macro_rules! tree_export_str
{
  ( $src : expr ) =>
  {{
    let src2 = &$src;
    format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
  }};
}

/// Kind of container.

/* qqq : for rust : add HashSet */
#[derive( Debug, PartialEq, Copy, Clone )]
pub enum ContainerKind
{
  /// Not a container.
  No,
  /// Vector-like.
  Vector,
  /// Hash map-like.
  HashMap,
}

/// Return kind of container specified by type.
/// Good to verify `alloc::vec::Vec< i32 >` is vector.
/// Good to verify `std::collections::HashMap< i32, i32 >` is hash map.

pub fn container_kind( ty : &syn::Type ) -> ContainerKind
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
      _ => { return ContainerKind::No }
    }
  }
  ContainerKind::No
}

/// Check is the rightmost item of path refering a type is specified type.
/// Good to verify `core::option::Option< i32 >` is optional.
/// Good to verify `alloc::vec::Vec< i32 >` is vector.

pub fn rightmost_is< R : AsRef< str > >( ty : &syn::Type, rightmost : R ) -> bool
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return false;
    }
    return last.unwrap().ident == rightmost.as_ref();
  }
  false
}

/// Return the specified number of parameters of the type.
/// Good to getting `i32` from `core::option::Option< i32 >` or `alloc::vec::Vec< i32 >`

pub fn parameters_internal( ty : &syn::Type, r : ::core::ops::RangeInclusive< usize > ) -> Vec< &syn::Type >
{
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
      .skip( *r.start() )
      .take( *r.end() - *r.start() + 1 )
      .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
      .collect();
      return selected;
    }
  }
  vec![ &ty ]
}

// /// Wrapper to implement trait Spanned for those structures for which it's not implemented.
//
// pub struct DataWrapped( pub syn::Data );
// impl syn::spanned::Spanned for DataWrapped
// {
//   fn span( &self ) -> proc_macro2::Span
//   {
//     let DataWrapped( ref data ) = self;
//     match data
//     {
//       syn::Data::Struct( syn::DataStruct { ref fields, .. } ) => fields.span(),
//       syn::Data::Enum( syn::DataEnum { ref variants, .. } ) => variants.span(),
//       syn::Data::Union( syn::DataUnion { ref fields, .. } ) => fields.span(),
//       // _ => proc_macro2::Span::call_site(),
//     }
//   }
// }
//
// pub use syn::spanned::Spanned;

/// Trait to implement method span() for those structures which do not have it implemented.

pub trait Spanned2
{
  /// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.
  fn span2( &self ) -> proc_macro2::Span;
}

//

impl Spanned2 for syn::Data
{
  fn span2( &self ) -> proc_macro2::Span
  {
    match self
    {
      syn::Data::Struct( syn::DataStruct { ref fields, .. } ) => fields.span(),
      syn::Data::Enum( syn::DataEnum { ref variants, .. } ) => variants.span(),
      syn::Data::Union( syn::DataUnion { ref fields, .. } ) => fields.span(),
    }
  }
}

/// Returns a Span covering the complete contents of this syntax tree node, or Span::call_site() if this node is empty.

pub fn span_of< Src : Spanned2 >( src : &Src ) -> proc_macro2::Span
{
  src.span2()
}
