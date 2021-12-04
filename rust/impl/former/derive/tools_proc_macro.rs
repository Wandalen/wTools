
/* xxx : move */

#[allow(unused_macros)]
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

//

#[allow(unused_macros)]
macro_rules! tree_export_str
{
  ( $src : expr ) =>
  {{
    let src2 = &$src;
    format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
  }};
}

//

#[derive( Debug )]
pub enum ContainerKind
{
  No,
  Vector,
  HashMap,
}

//

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

//

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

//

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
