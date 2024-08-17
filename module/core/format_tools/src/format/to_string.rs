//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  use std::
  {
    fmt,
    borrow::Cow,
  };

  // ==

  /// Marker type for returning reference representing instance instead of allocating new string.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithRef;

  /// Marker type for using Debug formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDebug;

  /// Marker type for using Debug multiline formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDebugMultiline;

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
    fn to_string_with< 's >( &'s self ) -> Cow< 's, str >;
  }

  impl< 'a, T > ToStringWith< WithRef > for T
  where
    T : 'a,
    T : AsRef< str >,
    T : ?Sized,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with< 's >( &'s self ) -> Cow< 's, str >
    {
      // println!( " - WithRef" );
      Cow::Borrowed( self.as_ref() )
    }
  }

  impl< 'a, T > ToStringWith< WithDebug > for T
  where
    T : fmt::Debug,
    T : ?Sized,
  {
    /// Converts the type to a string using Debug formatting.
    #[ inline ]
    fn to_string_with< 's >( &'s self ) -> Cow< 's, str >
    {
      // println!( " - WithDebug {:?}", self );
      Cow::Owned( format!( "{:?}", self ) )
    }
  }

  impl< 'a, T > ToStringWith< WithDebugMultiline > for T
  where
    T : fmt::Debug,
    T : ?Sized,
  {
    /// Converts the type to a string using Debug formatting.
    #[ inline ]
    fn to_string_with< 's >( &'s self ) -> Cow< 's, str >
    {
      // println!( " - WithDebugMultiline {:#?}", self );
      Cow::Owned( format!( "{:#?}", self ) )
    }
  }

  impl< 'a, T > ToStringWith< WithDisplay > for T
  where
    T : 'a,
    T : fmt::Display,
    T : ?Sized,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with< 's >( &'s self ) -> Cow< 's, str >
    {
      // println!( " - WithDisplay {}", self );
      Cow::Owned( format!( "{}", self ) )
    }
  }

}

mod aref;

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
  pub use super::super::to_string;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
    WithDebug,
    WithDebugMultiline,
    WithDisplay,
    WithRef,
    WithWell,
    ToStringWith,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
