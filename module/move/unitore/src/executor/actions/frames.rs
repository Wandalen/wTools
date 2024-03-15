//! Frames commands actions.

use crate::*;
use super::*;
use executor::FeedManager;
use storage::
{
  FeedStorage,
  config::ConfigStore,
  frame::{ FrameStore, RowValue }
};
use gluesql::prelude::{ Payload, Value, SledStorage };
use feed_config;
use error_tools::{ err, Result };

/// List all frames.
pub async fn list_frames
(
  storage : FeedStorage< SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
    let mut manager = FeedManager::new( storage );
    manager.storage.list_frames().await
}

/// Update all frames from config files saved in storage.
pub async fn download_frames
(
  storage : FeedStorage< SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  let payload = manager.storage.list_configs().await?;

  let configs = match &payload
  {
    Payload::Select { labels: _, rows: rows_vec } =>
    {
      rows_vec.into_iter().filter_map( | val |
      {
        match &val[ 0 ]
        {
          Value::Str( path ) => Some( path.to_owned() ),
          _ => None,
        }
      } ).collect::< Vec< _ > >()
    },
    _ => Vec::new(),
  };

  let mut subscriptions = Vec::new();
  for config in &configs
  {
    let sub_vec = feed_config::read( config.to_owned() )?;
    subscriptions.extend( sub_vec );
  }

  if subscriptions.is_empty()
  {
    return Err( err!( format!(
      "Failed to download frames.\n Config files {} contain no feed subscriptions!",
      configs.join( ", " )
    ) ) )
  }

  manager.update_feed( subscriptions ).await

}

const EMPTY_CELL : &'static str = "";
const INDENT_CELL : &'static str = "  ";

/// Information about result of execution of command for frames.
#[ derive( Debug ) ]
pub struct FramesReport
{
  /// Link of the feed which contains the frames.
  pub feed_link : String,
  /// Number of frames from the feed that were updated.
  pub updated_frames : usize,
  /// Number of new frames from the feed that were downloaded.
  pub new_frames : usize,
  /// Selected frames for commands that list frames.
  pub selected_frames : SelectedEntries,
  /// Number of frames that were in storage before update.
  pub existing_frames : usize,
  /// True if feed is downloaded for the first time.
  pub is_new_feed : bool,
}

impl FramesReport
{
  /// Create new report.
  pub fn new( feed_link : String ) -> Self
  {
    Self
    {
      feed_link,
      updated_frames : 0,
      new_frames : 0,
      selected_frames : SelectedEntries::new(),
      existing_frames : 0,
      is_new_feed : false,
    }
  }
}

impl std::fmt::Display for FramesReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    let initial = vec![ vec![ format!( "Feed title: {}", self.feed_link ) ] ];
    let table = table_display::table_with_headers( initial[ 0 ].clone(), Vec::new() );
    if let Some( table ) = table
    {
      write!( f, "{}", table )?;
    }

    let mut rows = vec!
    [
      vec![ EMPTY_CELL.to_owned(), format!( "Updated frames: {}", self.updated_frames ) ],
      vec![ EMPTY_CELL.to_owned(), format!( "Inserted frames: {}", self.new_frames ) ],
      vec![ EMPTY_CELL.to_owned(), format!( "Number of frames in storage: {}", self.existing_frames ) ],
    ];

    if !self.selected_frames.selected_columns.is_empty()
    {
      rows.push( vec![ EMPTY_CELL.to_owned(), format!( "Selected frames:" ) ] );
    }

    let table = table_display::plain_table( rows );
    if let Some( table ) = table
    {
      write!( f, "{}", table )?;
    }
      
    for frame in &self.selected_frames.selected_rows
    {
      let mut rows = Vec::new();
      for i in 0..self.selected_frames.selected_columns.len()
      {
        let inner_row = vec!
        [
          INDENT_CELL.to_owned(),
          self.selected_frames.selected_columns[ i ].clone(),
          textwrap::fill( &String::from( frame[ i ].clone() ), 120 ),
        ];
        rows.push( inner_row );
      }
      
      let table = table_display::plain_table( rows );
      if let Some( table ) = table
      {
        writeln!( f, "{}", table )?;
      }
    }

    Ok( () )
  }
}

impl Report for FramesReport {}

/// Items get from select query from storage.
#[ derive( Debug ) ]
pub struct SelectedEntries
{
  /// Labels of selected columns.
  pub selected_columns : Vec< String >,
  /// Selected rows with data.
  pub selected_rows : Vec< Vec< Value > >,
}

impl SelectedEntries
{
  /// Create new empty selected entries struct.
  pub fn new() -> Self
  {
    SelectedEntries { selected_columns : Vec::new(), selected_rows : Vec::new() }
  }
}

impl std::fmt::Display for SelectedEntries
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    if !self.selected_columns.is_empty()
    {
      for row in &self.selected_rows
      {
        for i in 0..self.selected_columns.len()
        {
          write!( f, "{} : {}, ", self.selected_columns[ i ], RowValue( &row[ i ] ) )?;
        }
        writeln!( f, "" )?;
      }
    }

    Ok( () )
  }
}

/// Report for downloading and updating frames.
#[ derive( Debug ) ]
pub struct UpdateReport( pub Vec< FramesReport > );

impl std::fmt::Display for UpdateReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    for report in &self.0
    {
      writeln!( f, "{}", report )?;
    }
    writeln!( f, "Total new feeds dowloaded : {}", self.0.iter().filter( | fr_report | fr_report.is_new_feed ).count() )?;
    writeln!
    (
      f,
      "Total feeds with updated or new frames : {}",
      self.0.iter().filter( | fr_report | fr_report.updated_frames + fr_report.new_frames > 0 ).count()
    )?;
    writeln!( f, "Total new frames : {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.new_frames ) )?;
    writeln!( f, "Total updated frames : {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.updated_frames ) )?;

    Ok( () )
  }
}

impl Report for UpdateReport {}

/// Report for listing frames.
#[ derive( Debug ) ]
pub struct ListReport( pub Vec< FramesReport > );

impl std::fmt::Display for ListReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    for report in &self.0
    {
      write!( f, "{}", report )?;
    }
    writeln!
    (
      f,
      "Total feeds in storage: {}",
      self.0.len()
    )?;
    writeln!
    (
      f,
      "Total frames in storage: {}",
      self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.selected_frames.selected_rows.len() )
    )?;
    writeln!( f, "" )?;

    Ok( () )
  }
}

impl Report for ListReport {}
