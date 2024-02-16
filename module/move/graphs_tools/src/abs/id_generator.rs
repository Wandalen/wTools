/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;
  // use core::fmt;
  // use core::hash::Hash;
  // use core::cmp::{ PartialEq, Eq };
  use crate::IdentityInterface;

  /// Has ID generator.

  pub trait HasIdGenerator< Id >
  where
    Id : IdentityInterface,
  {
    /// Associated id generator.
    type Generator : IdGeneratorTrait< Id >;
  }

  /// Interface to generate ids.

  pub trait IdGeneratorTrait< Id >
  where
    Id : IdentityInterface,
    Self : Default,
  {
    /// Generate a new id.
    fn id_next( &mut self ) -> Id;
    /// Check is id valid.
    fn is_id_valid( &self, src : Id ) -> bool;
  }

  // impl< T, G > HasIdGenerator< T > for T
  // where
  //   G : IdGeneratorTrait< T >,
  // {
  //   type Generator = G;
  // }

}

//

crate::mod_interface!
{
  prelude use super::private::
  {
    HasIdGenerator,
    IdGeneratorTrait,
    // IdGeneratorInt,
  };
}
