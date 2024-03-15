//! Endpoint and report for commands for config files.

use crate::*;
use super::*;
use error_tools::{ err, for_app::Context, BasicError, Result };
use executor::FeedManager;
use storage::{ FeedStorage, FeedStore };
use gluesql::{ prelude::Payload, sled_storage::SledStorage };

/// Add configuration file with subscriptions to storage.
pub async fn add_config( storage : FeedStorage< SledStorage >, args : &wca::Args ) -> Result< impl Report >
{
  let path : std::path::PathBuf = args
  .get_owned::< wca::Value >( 0 )
  .ok_or_else::< BasicError, _ >( || err!( "Cannot get path argument for command .config.add" ) )?
  .into()
  ;

  let mut manager = FeedManager::new( storage );
  let path = path.canonicalize().context( format!( "Invalid path for config file {:?}", path ) )?;
  let config_report = manager.storage
  .add_config( path.to_string_lossy().to_string() )
  .await
  .context( "Added 0 config files.\n Failed to add config file to storage." )?
  ;

  let feeds = feed_config::read( path.to_string_lossy().to_string() )?
  .into_iter()
  .map( | feed | crate::storage::model::FeedRow::new( feed.link, feed.update_period ) )
  .collect::< Vec< _ > >()
  ;

  let new_feeds = manager.storage.add_feeds( feeds ).await?;

  Ok( ConfigReport{ payload : config_report, new_feeds : Some( new_feeds ) } )
}

/// Remove configuration file from storage.
pub async fn delete_config( storage : FeedStorage< SledStorage >, args : &wca::Args ) -> Result< impl Report >
{
  let path : std::path::PathBuf = args
  .get_owned::< wca::Value >( 0 )
  .ok_or_else::< BasicError, _ >( || err!( "Cannot get path argument for command .config.delete" ) )?
  .into()
  ;

  let path = path.canonicalize().context( format!( "Invalid path for config file {:?}", path ) )?;
  let mut manager = FeedManager::new( storage );
  Ok( ConfigReport::new( 
    manager.storage
    .delete_config( path.to_string_lossy().to_string() )
    .await
    .context( "Failed to remove config from storage." )?
  ) )
}

/// List all files with subscriptions that are currently in storage.
pub async fn list_configs( storage : FeedStorage< SledStorage >, _args : &wca::Args ) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  Ok( ConfigReport::new( manager.storage.list_configs().await? ) )
}

/// Information about result of command for subscription config.
#[ derive( Debug ) ]
pub struct ConfigReport
{
  payload : Payload,
  new_feeds : Option< Payload >,
}

impl ConfigReport
{
  /// Create new report for config report with provided payload.
  pub fn new( payload : Payload ) -> Self
  {
    Self { payload, new_feeds : None }
  }
}

impl std::fmt::Display for ConfigReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    const EMPTY_CELL : &'static str = "";
    
    match &self.payload
    {
      Payload::Insert( number ) => 
      {
        writeln!( f, "Added {} config file(s)", number )?;
        writeln!(
          f,
          "Added {} feeds",
          self.new_feeds
          .as_ref()
          .and_then( | payload |
            match payload
            {
              Payload::Insert( number ) => Some( *number ),
              _ => None,
            }
          )
          .unwrap_or_default(),
        )?;
      },
      Payload::Delete( number ) => writeln!( f, "Deleted {} config file", number )?,
      Payload::Select { labels: _label_vec, rows: rows_vec } =>
      {
        writeln!( f, "Selected configs:" )?;
        let mut rows = Vec::new();
        for row in rows_vec
        {
          rows.push( vec![ EMPTY_CELL.to_owned(), String::from( row[ 0 ].clone() ) ] );
        }

        let table = table_display::plain_table( rows );
        if let Some( table ) = table
        {
          write!( f, "{}", table )?;
        }
      },
      _ => {},
    };

    Ok( () )
  }
}

impl Report for ConfigReport {}
