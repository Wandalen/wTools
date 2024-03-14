use crate::*;
use cli_table::{ format::{ Border, Separator }, Cell, Style, Table };
use executor::FeedManager;
use super::{ Report, frames::SelectedEntries };
use storage::{ FeedStorage, FeedStore };
use error_tools::Result;

/// List all feeds.
pub async fn list_feeds(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  manager.storage.get_all_feeds().await
}

const EMPTY_CELL : &'static str = "";

/// Information about result of execution of command for feed.
#[ derive( Debug ) ]
pub struct FeedsReport
{
  pub selected_entries : SelectedEntries,
}

impl FeedsReport
{
  pub fn new() -> Self
  {
    Self { selected_entries : SelectedEntries::new() }
  }
}

impl std::fmt::Display for FeedsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!( f, "Selected feeds:" )?;
    if !self.selected_entries.selected_rows.is_empty()
    {
      let mut rows = Vec::new();
      for row in &self.selected_entries.selected_rows
      {
        let mut new_row = vec![ EMPTY_CELL.cell() ];
        new_row.extend( row.iter().map( | cell | String::from( cell ).cell() ) );
        rows.push( new_row );
      }
      let mut headers = vec![ EMPTY_CELL.cell() ];
      headers.extend( self.selected_entries.selected_columns.iter().map( | header | header.cell().bold( true ) ) );
      let table_struct = rows.table()
      .title( headers )
      .border( Border::builder().build() )
      .separator( Separator::builder().build() );

      let table = table_struct.display().unwrap();
      writeln!( f, "{}", table )?;
    }
    else
    {
      writeln!( f, "No items currently in storage!" )?;
    }

    Ok( () )
  }
}

impl Report for FeedsReport {}
