//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use std::
  {
    fmt,
  };

  // ==

  /// Marker type for using Debug formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDebug;

  /// Marker type for using Display formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDisplay;

  /// Marker type for usign Well formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithWell;

  // ==

  /// Trait to convert a type to a string using a specified formatting method.
  pub trait ToStringWith< How >
  {
    /// Converts the type to a string using the specified formatting method.
    fn to_string_with( &self ) -> String;
  }

  impl< T > ToStringWith< WithDebug > for T
  where
    T : fmt::Debug,
  {
    /// Converts the type to a string using Debug formatting.
    fn to_string_with( &self ) -> String
    {
      format!( "{:?}", self )
    }
  }

  impl< T > ToStringWith< WithDisplay > for T
  where
    T : fmt::Display,
  {
    /// Converts the type to a string using Display formatting.
    fn to_string_with( &self ) -> String
    {
      format!( "{}", self )
    }
  }

  // impl ToStringWith< WithDisplay > for String
  // {
  //   /// Converts the type to a string using Display formatting.
  //   fn to_string_with( &self ) -> String
  //   {
  //     format!( "x{}", self )
  //   }
  // }

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
  #[ doc( inline ) ]
  pub use private::
  {
    WithDebug,
    WithDisplay,
    WithWell,
    ToStringWith,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
