/// Internal namespace.
mod internal
{
  use crate::exposed::*;
  use crate::type_rightmost;

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

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  use super::internal as i;

  pub use i::ContainerKind;
  pub use i::type_container_kind;
  pub use i::type_optional_container_kind;

}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}
