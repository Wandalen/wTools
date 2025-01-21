/// Define a private namespace for all its items.
mod private
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
      Self( val )
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
  // =
  //

  types!
  {
    /// Identify an instance by integer.
    #[ derive( PartialEq, Eq, Copy, Clone, Hash ) ]
    pub single IdentityWithInt : isize;
  }

  ///
  /// Interface to to generate a new IDs for IdentityWithInt
  ///

  #[ derive( Debug, Copy, Clone, Default ) ]
  pub struct IdGeneratorInt
  {
    counter : IdentityWithInt,
  }

  impl IdGeneratorTrait< IdentityWithInt > for IdGeneratorInt
  {
    /// Generate a new identity based on the current increasing it.
    fn id_next( &mut self ) -> IdentityWithInt
    {
      self.counter.0 += 1;
      self.counter
    }
    /// Check is the identity valid.
    fn is_id_valid( &self, src : IdentityWithInt ) -> bool
    {
      src.0 >= 0 && src.0 < self.counter.0
    }
  }

  impl HasIdGenerator< IdentityWithInt > for IdentityWithInt
  {
    type Generator = IdGeneratorInt;
  }

//   impl IdentityGenerableInterface for IdentityWithInt
//   {
//
//     fn next( &self ) -> Self
//     {
//       let result = Self( self.0 + 1 );
//       assert!( self.is_valid() );
//       result
//     }
//
//     fn is_valid( &self ) -> bool
//     {
//       self.0 > 0
//     }
//
//   }

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

//

crate::mod_interface!
{
  exposed use super::private::
  {
    IdentityWithPointer,
    IdentityWithName,
    IdentityWithInt,
    IdGeneratorInt,
  };
}
