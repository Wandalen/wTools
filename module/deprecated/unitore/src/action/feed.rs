//! Feed actions and reports.

use crate :: *;
use sled_adapter ::FeedStorage;
use entity ::feed ::FeedStore;
use error_tools ::untyped ::Result;
use gluesql ::prelude ::SledStorage;

pub use entity ::feed ::FeedsReport;

/// List all feeds from storage.
///
/// # Errors
/// Returns error if operation fails.
pub async fn feeds_list( mut storage: FeedStorage< SledStorage > ) -> Result< impl Report >
{
  storage.feeds_list().await
}
