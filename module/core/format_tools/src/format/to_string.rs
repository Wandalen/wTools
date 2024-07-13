//!
//! Flexible ToString augmentation.
//!

/// Internal namespace.
pub( crate ) mod private
{

  pub use super::
  {
    aref::{ Ref, Ref2 },
    // aref2::{ Ref2 },
  };

  use std::
  {
    fmt,
    borrow::Cow,
  };

  // ==

  /// Marker type for using Debug formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDebug;

  /// Marker type for using Display formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithDisplay;

  /// Marker type for returning reference representing instance instead of allocating new string.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithRef;

  /// Marker type for usign Well formatting.
  #[ derive( Debug, Default, Clone, Copy ) ]
  pub struct WithWell;

  // ==

  /// Trait to convert a type to a string using a specified formatting method.
  pub trait ToStringWith< 'a, How >
  {
    /// Converts the type to a string using the specified formatting method.
    fn to_string_with( &'a self ) -> Cow< 'a, str >;
  }

  impl< 'a, T > ToStringWith< 'a, WithDebug > for T
  where
    T : fmt::Debug,
    T : ?Sized,
  {
    /// Converts the type to a string using Debug formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      println!( " - WithDebug Ref {:?}", self );
      Cow::Owned( format!( "{:?}", self ) )
    }
  }

  impl< 'a, T > ToStringWith< 'a, WithDebug > for Ref< 'a, T, WithDebug >
  where
    // T : 'a,
    T : fmt::Debug,
    T : ?Sized,
  {
    /// Converts the type to a string using Debug formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      // println!( " - WithDebug Ref2 {:?}", self.0 );
      Cow::Owned( format!( "{:?}", self.0.0 ) )
    }
  }

  impl< 'a, T > ToStringWith< 'a, WithDisplay > for T
  where
    T : 'a,
    T : fmt::Display,
    T : ?Sized,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      Cow::Owned( format!( "{}", self ) )
    }
  }

  impl< 'a, T > ToStringWith< 'a, WithDisplay > for Ref< 'a, T, WithDisplay >
  where
    T : 'a,
    T : fmt::Display,
    T : ?Sized,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      Cow::Owned( format!( "{}", self.0.0 ) )
    }
  }

  // impl< 'a, AsStr > ToStringWith< 'a, WithDisplay > for Ref< 'a, AsStr, WithDisplay >
  // where
  //   AsStr : AsRef< str >,
  //   AsStr : ?Sized,
  // {
  //   /// Converts the type to a string using Display formatting.
  //   #[ inline ]
  //   fn to_string_with( &'a self ) -> Cow< 'a, str >
  //   {
  //     Cow::Borrowed( self.0.0.as_ref() )
  //   }
  // }

  // impl< 'a, T > ToStringWith< 'a, WithDisplay > for Ref2< 'a, T, WithDisplay >
  // where
  //   T : 'a,
  //   T : fmt::Display,
  //   T : ?Sized,
  // {
  //   /// Converts the type to a string using Display formatting.
  //   #[ inline ]
  //   fn to_string_with( &'a self ) -> Cow< 'a, str >
  //   {
  //     println!( " - WithDisplay Ref2 {}", self.0 );
  //     Cow::Owned( format!( "{}", self.0 ) )
  //   }
  // }

  impl< 'a, T > ToStringWith< 'a, WithRef > for T
  where
    T : 'a,
    T : AsRef< str >,
    T : ?Sized,
  {
    /// Converts the type to a string using Display formatting.
    #[ inline ]
    fn to_string_with( &'a self ) -> Cow< 'a, str >
    {
      Cow::Borrowed( self.as_ref() )
    }
  }

  // impl< 'a, AsStr > ToStringWith< 'a, WithRef > for Ref< 'a, AsStr, WithRef >
  // where
  //   AsStr : AsRef< str >,
  //   AsStr : ?Sized,
  // {
  //   /// Converts the type to a string using Display formatting.
  //   #[ inline ]
  //   fn to_string_with( &'a self ) -> Cow< 'a, str >
  //   {
  //     Cow::Borrowed( self.0.0.as_ref() )
  //   }
  // }

}

mod aref;
// mod aref2;

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
    Ref,
    Ref2,
  };
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
