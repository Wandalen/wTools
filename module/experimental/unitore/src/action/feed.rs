//! Feed actions and reports.

use crate :: *;
use action :: { Report, frame ::SelectedEntries };
use sled_adapter ::FeedStorage;
use entity ::feed ::FeedStore;
use error_tools ::untyped ::Result;
use gluesql ::prelude ::SledStorage;

/// List all feeds from storage.
///
/// # Errors
/// Returns error if operation fails.
pub async fn feeds_list( mut storage: FeedStorage< SledStorage > ) -> Result< impl Report >
{
  storage.feeds_list().await
}

const EMPTY_CELL: &str = "";

/// Information about result of execution of command for feed.
#[ derive( Debug ) ]
pub struct FeedsReport( pub SelectedEntries );

impl Default for FeedsReport
{
  fn default() -> Self
  {
  Self ( SelectedEntries ::new() )
 }
}

impl FeedsReport
{
  /// Create new empty report for feeds command.
  #[must_use]
  pub fn new() -> Self
  {
  Self ::default()
 }
}

impl core ::fmt ::Display for FeedsReport
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  writeln!( f, "Selected feeds: " )?;
  if self.0.selected_rows.is_empty() {
    writeln!( f, "No items currently in storage!" )?;
  } else {
    let mut rows = Vec ::new();
    for row in &self.0.selected_rows
    {
   let mut new_row = vec![ EMPTY_CELL.to_owned() ];
   new_row.extend( row.iter().map( String ::from ) );
   rows.push( new_row );
  }
    let mut headers = vec![ EMPTY_CELL.to_owned() ];
    headers.extend( self.0.selected_columns.iter().map( std::borrow::ToOwned::to_owned ) );

    let table = tool ::table_display ::table_with_headers( headers, rows );
    if let Some( table ) = table
    {
   write!( f, "{table}" )?;
  }
  }

  Ok( () )
 }
}

impl Report for FeedsReport {}
