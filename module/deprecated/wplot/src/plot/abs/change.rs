use crate ::abs ::ChangerInterface;
use super :: *;
use super ::identity ::Id;

/// Interface to describe change.
pub trait ChangeInterface
{
  /// Get id.
  fn id( &self ) -> Id;
}
