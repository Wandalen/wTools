//! Feed command.

use crate::*;
use gluesql::sled_storage::sled::Config;
use wca::{ Command, VerifiedCommand };
use sled_adapter::FeedStorage;
use action::{ Report, feed::feeds_list };
use error_tools::untyped::Result;

/// Struct that provides commands for feed.
#[ derive( Debug ) ]
pub struct FeedCommand;

impl FeedCommand
{
  /// Create command that lists all feeds in storage.
  pub fn list() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "feeds.list" )
      .long_hint( concat!
      (
        "List all feeds from storage.\n",
        "    Example: .feeds.list",
      ))
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
            feeds_list( feed_storage ).await
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