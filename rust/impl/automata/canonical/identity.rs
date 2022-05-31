/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt;
  use core::hash::Hash;
  use std::cmp::{ PartialEq, Eq };
  use wtools::dt::prelude::*;
  // use std::fmt;

  ///
  /// Identify an instance by its location in memory.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  pub struct IdentityWithPointer( usize );

  impl IdentityWithPointer
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make< T >( src : &T ) -> Self
    {
      // Safety : it differentiate different instances.
      let ptr = unsafe
      {
        std::mem::transmute::< _, usize >( src )
      };
      Self( ptr )
    }

  }

  impl< 'a, T > From< &'a T > for IdentityWithPointer
  {
    fn from( src : &'a T ) -> Self
    {
      let ptr = unsafe
      {
        std::mem::transmute::< _, usize >( src )
      };
      Self( ptr )
    }
  }

  ///
  /// Identify an instance by name.
  ///

  #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  pub struct IdentityWithName( pub &'static str )
  ;

  impl IdentityWithName
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make( val : &'static str ) -> Self
    {
      Self( val.into() )
    }

  }

  impl From< &'static str > for IdentityWithName
  {
    fn from( src : &'static str ) -> Self
    {
      Self( src )
    }
  }

  impl< Src > From< &Src > for IdentityWithName
  where
    Src : Clone,
    IdentityWithName : From< Src >,
  {
    fn from( src : &Src ) -> Self
    {
      From::< Src >::from( src.clone() )
    }
  }

  impl fmt::Debug for IdentityWithName
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "{}", self.0 ) )
    }
  }

  ///
  /// Identify an instance by integer.
  ///

  #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  pub struct IdentityWithInt( pub isize )
  ;

  // types!
  // {
  //   /// Identify an instance by integer.
  //   #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default, Debug ) ]
  //   single IdentityWithInt : isize;
  //}

  impl IdentityWithInt
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make( val : isize ) -> Self
    {
      Self( val.into() )
    }
    // xxx : make?

  }

  // xxx : implement IdentityIncInterface for other identities
  impl IdentityIncInterface for IdentityWithInt
  {
    fn inc( &self ) -> Self
    {
      let result = Self( self.0 + 1 );
      assert!( result.0 > 0 );
      result
    }
  }

  impl From< isize > for IdentityWithInt
  {
    fn from( src : isize ) -> Self
    {
      Self( src )
    }
  }

  impl< Src > From< &Src > for IdentityWithInt
  where
    Src : Clone,
    IdentityWithInt : From< Src >,
  {
    fn from( src : &Src ) -> Self
    {
      From::< Src >::from( src.clone() )
    }
  }

  impl fmt::Debug for IdentityWithInt
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "{}", self.0 ) )
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::private::IdentityWithPointer;
  pub use super::private::IdentityWithName;
  pub use super::private::IdentityWithInt;
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}

