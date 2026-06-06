//! Query actions and report.

use crate :: *;
use sled_adapter ::Store;
use error_tools ::untyped ::Result;

pub use sled_adapter ::QueryReport;

/// Execute query specified in query string.
///
/// # Errors
/// Returns error if operation fails.
pub async fn query_execute
(
  mut storage: impl Store,
  query_str: String,
) -> Result< impl Report >
{
  storage.query_execute( query_str ).await
}
