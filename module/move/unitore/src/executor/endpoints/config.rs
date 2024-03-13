use crate::*;
use executor::FeedManager;
use super::Report;
use storage::{ FeedStorage, FeedStore };
use gluesql::{ prelude::Payload, sled_storage::SledStorage };
use cli_table::
{
  format::{ Border, Separator}, Cell, Table
};

use feed_config::read_feed_config;

pub async fn add_config( storage : FeedStorage< SledStorage >, args : &wca::Args ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path : std::path::PathBuf = args.get_owned::< wca::Value >( 0 ).unwrap().into();
  let mut manager = FeedManager::new( storage );

  let path = path.canonicalize().expect( "Invalid path" );
  let config_report = manager.storage.add_config( path.to_string_lossy().to_string() ).await?;
  let feeds = read_feed_config( path.to_string_lossy().to_string() )?
  .into_iter()
  .map( | feed | crate::storage::model::FeedRow::new( feed.link, feed.update_period ) )
  .collect::< Vec< _ > >()
  ;

  manager.storage.add_feeds( feeds ).await?;
  Ok( ConfigReport( config_report ) )
}

pub async fn remove_config( storage : FeedStorage< SledStorage >, args : &wca::Args ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let path = args.get_owned::< String >( 0 ).unwrap().into();
  let mut manager = FeedManager::new( storage );
  Ok( ConfigReport( manager.storage.remove_config( path ).await? ) )
}

pub async fn list_configs( storage : FeedStorage< SledStorage >, _args : &wca::Args ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let mut manager = FeedManager::new( storage );
  Ok( ConfigReport( manager.storage.list_configs().await? ) )
}

/// Information about result of command for subscription config.
#[ derive( Debug ) ]
pub struct ConfigReport( Payload );

impl std::fmt::Display for ConfigReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    const EMPTY_CELL : &'static str = "";
    
    match &self.0
    {
      Payload::Insert( number ) => writeln!( f, "Added {} config", number )?,
      Payload::Delete( number ) => writeln!( f, "Deleted {} config", number )?,
      Payload::Select { labels: _label_vec, rows: rows_vec } =>
      {
        writeln!( f, "Selected configs:" )?;
        let mut rows = Vec::new();
        for row in rows_vec
        {
          rows.push( vec![ EMPTY_CELL.cell(), String::from( row[ 0 ].clone() ).cell() ] );
        }

        let table_struct = rows.table()
        .border( Border::builder().build() )
        .separator( Separator::builder().build() );

        let table = table_struct.display().unwrap();

        writeln!( f, "{}", table )?;

      },
      _ => {},
    };

    Ok( () )
  }
}

impl Report for ConfigReport {}
