//! Table and columns commands.

use crate::*;
use gluesql::sled_storage::sled::Config;
use wca::{ Command, Type, VerifiedCommand };
use sled_adapter::FeedStorage;
use action::{ Report, table::{ table_list, tables_list } };
use error_tools::Result;

/// Struct that provides commands for table information.
#[ derive( Debug ) ]
pub struct TableCommand;

impl TableCommand
{
  /// Creates command to list info about tables in storage.
  pub fn list() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "table.list" )
      .long_hint( concat!
      (
        "Delete file with feeds configuraiton. Subject: path to config file.\n",
        "    Example: .config.delete ./config/feeds.toml",
      ))
      .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
      .routine( move | o : VerifiedCommand |
      {
        let table_name_arg = o.args.get_owned::< String >( 0 );

        let res = rt.block_on( async move
        {
          let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
          .unwrap_or( String::from( "./_data" ) )
          ;
          
          let config = Config::default()
          .path( path_to_storage )
          ;

          let feed_storage = FeedStorage::init_storage( &config ).await?;
          table_list( feed_storage, table_name_arg ).await
        } );
        match res
        {
          Ok( report ) => report.report(),
          Err( err ) => println!( "{:?}", err ),
        }
      })
      .end()
    )
  }
}

/// Struct that provides commands for table columns information.
#[ derive( Debug ) ]
pub struct TablesCommand;

impl TablesCommand
{
  /// Creates command to list info about table columns in storage.
  pub fn list() -> Result< Command >
  {

    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "tables.list" )
      .long_hint( concat!
      (
        "Delete file with feeds configuraiton. Subject: path to config file.\n",
        "    Example: .config.delete ./config/feeds.toml",
      ))
      .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
      .routine( move | _o : VerifiedCommand |
      {
        let res = rt.block_on( async move
        {
          let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
          .unwrap_or( String::from( "./_data" ) )
          ;
          
          let config = Config::default()
          .path( path_to_storage )
          ;

          let feed_storage = FeedStorage::init_storage( &config ).await?;
          tables_list( feed_storage ).await
        } );
        match res
        {
          Ok( report ) => report.report(),
          Err( err ) => println!( "{:?}", err ),
        }
      })
      .end()
    )
  }
}