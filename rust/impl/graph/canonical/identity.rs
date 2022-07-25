/// Internal namespace.
pub( crate ) mod private
{
  use crate::prelude::*;
  use core::fmt;
  use core::hash::Hash;
  use core::cmp::{ PartialEq, Eq };
  use wtools::dt::prelude::*;

  // types!
  // {
  //   /// Identify an instance by name.
  //   #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default, Debug ) ]
  //   pub single IdentityWithPointer : usize;
  // }

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
        core::mem::transmute::< _, usize >( src )
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
        core::mem::transmute::< _, usize >( src )
      };
      Self( ptr )
    }
  }

  //

  // zzz : implement IdentityGenerableInterface for other identities. make it working
  // zzz : use type constructors

  // types!
  // {
  //   /// Identify an instance by name.
  //   #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  //   pub single IdentityWithName : &'static str;
  // }

  ///
  /// Identify an instance by name.
  ///

  #[ derive( PartialEq, Eq, Copy, Clone, Hash ) ]
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

  //

  types!
  {
    /// Identify an instance by integer.
    #[ derive( PartialEq, Eq, Copy, Clone, Hash ) ]
    pub single IdentityWithInt : isize;
  }

  impl IdentityGenerableInterface for IdentityWithInt
  {

    fn next( &self ) -> Self
    {
      let result = Self( self.0 + 1 );
      assert!( self.is_valid() );
      result
    }

    fn is_valid( &self ) -> bool
    {
      self.0 > 0
    }

  }

  impl Default for IdentityWithInt
  {
    fn default() -> Self { Self( 1 ) }
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
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

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
  pub use super::private::
  {
    IdentityWithPointer,
    IdentityWithName,
    IdentityWithInt,
  };
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
