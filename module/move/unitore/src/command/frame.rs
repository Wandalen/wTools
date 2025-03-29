//! Frame commands.

use crate::*;
use gluesql::sled_storage::sled::Config;
use wca::{ Command, VerifiedCommand };
use sled_adapter::FeedStorage;
use action::{ Report, frame::{ frames_list, frames_download } };
use error_tools::untyped::Result;

/// Struct that provides commands for frames.
#[ derive( Debug ) ]
pub struct FrameCommand;

impl FrameCommand
{
  /// Create command that lists all frames in storage.
  pub fn list() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "frames.list" )
      .long_hint( concat!
      (
        "List all frames saved in storage.\n",
        "    Example: .frames.list",
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
            frames_list( feed_storage ).await
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

  /// Creates command that downloads frames from feeds specified in config files.
  pub fn download() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;

    Ok(
      Command::former()
      .phrase( "frames.download" )
      .hint( "Download frames from feed sources provided in config files." )
      .long_hint(concat!
      (
        "Download frames from feed sources provided in config files.\n",
        "    Example: .frames.download",
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
            frames_download( feed_storage ).await
          });
          match res
          {
            Ok( report ) => report.report(),
            Err( err ) => println!( "{:?}", err ),
          }
      })
    .end() )
  }
}