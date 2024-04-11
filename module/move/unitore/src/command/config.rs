//! Config files commands.

use std::path::PathBuf;

use crate::*;
use gluesql::sled_storage::sled::Config;
use wca::{ Command, Type, VerifiedCommand };
use sled_adapter::FeedStorage;
use action::{ Report, config::{ config_add, config_delete, config_list } };
use error_tools::Result;

/// Struct that provides commands for config files.
#[ derive( Debug ) ]
pub struct ConfigCommand;

impl ConfigCommand
{
  /// Create command for adding config.
  pub fn add() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "config.add" )
      .long_hint( concat!
      (
      "Add file with feeds configurations. Subject: path to config file.\n",
      "    Example: .config.add ./config/feeds.toml\n",
      "    The file should contain config entities with fields:\n",
      "   - `update_period` : update frequency for feed. Example values: `12h`, `1h 20min`, `2days 5h`;\n",
      "   - `link` : URL for feed source;\n\n",
      "    Example:\n",
      "    [[config]]\n",
      "    update_period = \"1min\"\n",
      "    link = \"https://feeds.bbci.co.uk/news/world/rss.xml\"\n",
      ))
      .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
      .routine( move | o : VerifiedCommand |
      {
        let path_arg = o.args
        .get_owned::< wca::Value >( 0 );

        if let Some( path ) = path_arg
        {
          let path : PathBuf = path.into();

          let res = rt.block_on
          ( async move
            {
              let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
              .unwrap_or( String::from( "./_data" ) )
              ;
              
              let config = Config::default()
              .path( path_to_storage )
              ;
    
              let feed_storage = FeedStorage::init_storage( &config ).await?;
              config_add( feed_storage, &path ).await
            }
          );

          match res
          {
            Ok( report ) => report.report(),
            Err( err ) => println!( "{:?}", err ),
          }
        }
      })
      .end()
    )
  }

  /// Create command for deleting config.
  pub fn delete() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok(
      Command::former()
      .phrase( "config.delete" )
      .long_hint( concat!
      (
      "Delete file with feeds configuraiton. Subject: path to config file.\n",
      "    Example: .config.delete ./config/feeds.toml",
      ))
      .subject().hint( "Path" ).kind( Type::Path ).optional( false ).end()
      .routine( move | o : VerifiedCommand |
        {
          let path_arg = o.args
          .get_owned::< wca::Value >( 0 );
  
          if let Some( path ) = path_arg
          {
            let path : PathBuf = path.into();
  
            let res = rt.block_on
            ( async move
              {
                let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
                .unwrap_or( String::from( "./_data" ) )
                ;
                
                let config = Config::default()
                .path( path_to_storage )
                ;
      
                let feed_storage = FeedStorage::init_storage( &config ).await?;
                config_delete( feed_storage, &path ).await
              }
            );
  
            match res
            {
              Ok( report ) => report.report(),
              Err( err ) => println!( "{:?}", err ),
            }
          }
        })
      .end()
    )
  }

  /// Create command for listing all config files in storage.
  pub fn list() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "config.list" )
      .long_hint( concat!
      (
      "List all config files saved in storage.\n",
      "    Example: .config.list",
      ))
      .routine( move | o : VerifiedCommand |
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
            config_list( feed_storage, &o.args ).await
          });
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
