//! Feed actions and reports.

use crate::*;
use action::{ Report, frame::SelectedEntries };
use sled_adapter::FeedStorage;
use entity::feed::FeedStore;
use error_tools::Result;

/// List all feeds from storage.
pub async fn feeds_list( mut storage : FeedStorage< gluesql::sled_storage::SledStorage > ) -> Result< impl Report >
{
  storage.feeds_list().await
}

const EMPTY_CELL : &'static str = "";

/// Information about result of execution of command for feed.
#[ derive( Debug ) ]
pub struct FeedsReport( pub SelectedEntries );

impl FeedsReport
{
  /// Create new empty report for feeds command.
  pub fn new() -> Self
  {
    Self ( SelectedEntries::new() )
  }
}

impl std::fmt::Display for FeedsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    writeln!( f, "Selected feeds:" )?;
    if !self.0.selected_rows.is_empty()
    {
      let mut rows = Vec::new();
      for row in &self.0.selected_rows
      {
        let mut new_row = vec![ EMPTY_CELL.to_owned() ];
        new_row.extend( row.iter().map( | cell | String::from( cell ) ) );
        rows.push( new_row );
      }
      let mut headers = vec![ EMPTY_CELL.to_owned() ];
      headers.extend( self.0.selected_columns.iter().map( | str | str.to_owned() ) );

      let table = tool::table_display::table_with_headers( headers, rows );
      if let Some( table ) = table
      {
        write!( f, "{}", table )?;
      }
    }
    else
    {
      writeln!( f, "No items currently in storage!" )?;
    }

    Ok( () )
  }
}

impl Report for FeedsReport {}
