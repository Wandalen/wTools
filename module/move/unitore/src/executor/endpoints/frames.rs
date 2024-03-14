use crate::*;
use executor::FeedManager;
use super::Report;
use storage::{ FeedStorage, FeedStore };
use gluesql::prelude::{ Payload, Value, SledStorage };
use feed_config::read_feed_config;
use error_tools::{err, Result};

/// List all frames.
pub async fn list_frames(
  storage : FeedStorage< SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
    let mut manager = FeedManager::new( storage );
    manager.storage.get_all_frames().await
}

/// Update all frames from config files saved in storage.
pub async fn download_frames(
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
    let sub_vec = read_feed_config( config.to_owned() )?;
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

use cli_table::
{
  format::{ Border, Separator}, Cell, Style, Table
};

const EMPTY_CELL : &'static str = "";
const INDENT_CELL : &'static str = "  ";

/// Information about result of execution of command for frames.
#[ derive( Debug ) ]
pub struct FramesReport
{
  pub feed_title : String,
  pub updated_frames : usize,
  pub new_frames : usize,
  pub selected_frames : SelectedEntries,
  pub existing_frames : usize,
  pub is_new_feed : bool,
}

impl FramesReport
{
  /// Create new report.
  pub fn new( feed_title : String ) -> Self
  {
    Self
    {
      feed_title,
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
    let initial = vec![ vec![ format!( "Feed title: {}", self.feed_title).cell().bold( true )  ] ];
    let table_struct = initial.table()
    .border( Border::builder().build() )
    .separator( Separator::builder().build() );

    let table = table_struct.display().unwrap(); 
    write!( f, "{}", table )?;

    let mut rows = vec![
      vec![ EMPTY_CELL.cell(), format!( "Updated frames: {}", self.updated_frames ).cell() ],
      vec![ EMPTY_CELL.cell(), format!( "Inserted frames: {}", self.new_frames ).cell() ],
      vec![ EMPTY_CELL.cell(), format!( "Number of frames in storage: {}", self.existing_frames ).cell() ],
    ];

    if !self.selected_frames.selected_columns.is_empty()
    {
      rows.push( vec![ EMPTY_CELL.cell(), format!( "Selected frames:" ).cell() ] );
    }
    let table_struct = rows.table()
    .border( Border::builder().build() )
    .separator( Separator::builder().build() );

    let table = table_struct.display().unwrap(); 

    write!( f, "{}", table )?;
      
    for frame in &self.selected_frames.selected_rows
    {
      let mut rows = Vec::new();
      for i in 0..self.selected_frames.selected_columns.len()
      {
        let inner_row = vec!
        [
          INDENT_CELL.cell(),
          self.selected_frames.selected_columns[ i ].clone().cell(),
          textwrap::fill( &String::from( frame[ i ].clone() ), 120 ).cell(),
        ];
        rows.push( inner_row );
      }
      
      let table_struct = rows.table()
      .border( Border::builder().build() )
      .separator( Separator::builder().build() )
      ;
      
      let table = table_struct.display().unwrap();
      writeln!( f, "{}", table )?;
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
          write!( f, "{} : {}, ", self.selected_columns[ i ], storage::model::RowValue( &row[ i ] ) )?;
        }
        writeln!( f, "" )?;
      }
    }

    Ok( () )
  }
}

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
