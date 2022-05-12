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
      Hash +
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
      Hash +
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

  // impl< Src > From< &Src > for IdentityWithPointer
  // where
  //   Src : Clone,
  //   IdentityWithPointer : From< Src >,
  // {
  //   fn from( src : &Src ) -> Self
  //   {
  //     From::< Src >::from( src.clone() )
  //   }
  // }

  #[ test ]
  fn identity_implemented_for_identity_by_pointer()
  {
    let x = 1;
    let y = 1;
    let src1 = IdentityWithPointer::make( &x );
    let src2 = IdentityWithPointer::make( &y );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    assert_ne!( src1, src2 );
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

  #[ test ]
  fn identity_implemented_for_identity_by_name()
  {
    let src1 = IdentityWithName::make( "abc" );
    let src2 = IdentityWithName::make( "abc" );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    assert_eq!( src1, src2 );
  }

  ///
  /// Identify an instance by integer.
  ///

  #[ derive( PartialEq, Eq, Copy, Clone, Hash, Default ) ]
  pub struct IdentityWithInt( pub isize )
  ;

  impl IdentityWithInt
  {

    /// Construct from an arbitrary reference.
    #[ inline ]
    pub fn make( val : isize ) -> Self
    {
      Self( val.into() )
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

  #[ test ]
  fn identity_implemented_for_identity_by_int()
  {
    let src1 = IdentityWithInt::make( 3 );
    let src2 = IdentityWithInt::make( 3 );
    check( src1 );
    fn check< T : IdentityInterface >( _ : T ){}
    assert_eq!( src1, src2 );
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
  pub use i::IdentityWithPointer;
  pub use i::IdentityWithName;
  pub use i::IdentityWithInt;
  pub use super::prelude::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::IdentityInterface;
  pub use i::HasId;
}
