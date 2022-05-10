/// Internal namespace.
pub mod internal
{
  // use crate::prelude::*;
  use core::fmt::Debug;
  use core::hash::Hash;
  use std::cmp::{ PartialEq, Eq };
  use std::fmt;

  ///
  /// Interface to identify an instance of somthging, for exampel a node.
  ///

  pub trait IdentityInterface
  where
    Self :
      'static +
      Copy +
      Hash +
      Debug +
      PartialEq +
      Eq +
  {
  }

  impl< T > IdentityInterface for T
  where
    T :
      'static +
      Copy +
      Hash +
      Debug +
      PartialEq +
      Eq +
    ,
  {
  }

  ///
  /// Instance has an id.
  ///

  pub trait HasId
  {
    /// Id of the node.
    type Id : IdentityInterface;
    /// Get id.
    fn id( &self ) -> Self::Id;
  }

  ///
  /// Identify an instance by its location in memory.
  ///

  #[ derive( Debug, PartialEq, Eq, Copy, Clone, Hash ) ]
  pub struct IdentityByPointer( usize );

  impl IdentityByPointer
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

  #[ test ]
  fn identity_implemented_for_identity_by_pointer()
  {
    let src = IdentityByPointer::make( &13 );
    check( src );
    fn check< T : IdentityInterface >( _ : T ){}
  }

  ///
  /// Identify an instance by name.
  ///

  #[ derive( PartialEq, Eq, Copy, Clone, Hash ) ]
  pub struct IdentityByName( pub &'static str )
  ;

  impl IdentityByName
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make( val : &'static str ) -> Self
    {
      Self( val.into() )
    }

  }

  impl From< &'static str > for IdentityByName
  {
    fn from( src : &'static str ) -> Self
    {
      Self( src )
    }
  }

  impl fmt::Debug for IdentityByName
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      f.write_fmt( format_args!( "{}", self.0 ) )
    }
  }

  #[ test ]
  fn identity_implemented_for_identity_by_name()
  {
    let src = IdentityByName::make( "abc" );
    check( src );
    fn check< T : IdentityInterface >( _ : T ){}
  }

}

/// Own namespace of the module.
pub mod own
{
  // use super::internal as i;
  pub use super::parented::*;
}

pub use own::*;

/// Parented namespace of the module.
pub mod parented
{
  // use super::internal as i;
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use i::IdentityByPointer;
  pub use i::IdentityByName;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::IdentityInterface;
  pub use i::HasId;
  // pub use i::EqById;
}
