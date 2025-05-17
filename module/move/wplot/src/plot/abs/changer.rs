use crate::abs::ChangeInterface;
use super::*;
use super::identity::Id;

/// Interface to describe changer.
pub trait ChangerInterface
{
  /// Get id.
  fn id( &self ) -> Id;
  /// Get parent.
  fn parent( &self ) -> &dyn super::ContextInterface;
  /// Get root.
  fn root( &self ) -> *const dyn super::ContextInterface;
}
