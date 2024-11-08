//!
//! Determine kind of a container.
//!

/// Define a private namespace for all its items.
mod private
{
  use crate::*;
  // use crate::type_rightmost;

  ///
  /// Kind of container.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone ) ]
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
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::exposed::*;
  ///
  /// let code = qt!( std::collections::HashMap< i32, i32 > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let kind = container_kind::of_type( &tree_type );
  /// assert_eq!( kind, container_kind::ContainerKind::HashMap );
  /// ```

  pub fn of_type( ty : &syn::Type ) -> ContainerKind
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

  /// Return kind of container specified by type. Unlike [of_type] it also understand optional types.
  ///
  /// Good to verify `Option< alloc::vec::Vec< i32 > >` is optional vector.
  ///
  /// ### Basic use-case.
  /// ```
  /// use macro_tools::exposed::*;
  ///
  /// let code = qt!( Option< std::collections::HashMap< i32, i32 > > );
  /// let tree_type = syn::parse2::< syn::Type >( code ).unwrap();
  /// let ( kind, optional ) = container_kind::of_optional( &tree_type );
  /// assert_eq!( kind, container_kind::ContainerKind::HashMap );
  /// assert_eq!( optional, true );
  /// ```

  pub fn of_optional( ty : &syn::Type ) -> ( ContainerKind, bool )
  {

    if typ::type_rightmost( ty ) == Some( "Option".to_string() )
    {
      let ty2 = typ::type_parameters( ty, 0 ..= 0 ).first().copied();
      // inspect_type::inspect_type_of!( ty2 );
      if ty2.is_none()
      {
        return ( ContainerKind::No, false )
      }
      let ty2 = ty2.unwrap();
      return ( of_type( ty2 ), true )
    }

    ( of_type( ty ), false )
  }

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use private::
  {
    ContainerKind,
    of_type,
    of_optional,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::container_kind;

  // pub use super::own as container_kind;

  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
