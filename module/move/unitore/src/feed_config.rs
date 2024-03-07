// use super::*;
use std::{ fs::OpenOptions, io::{ BufReader, Read } };
use serde::Deserialize;

/// Configuration for subscription to feed resource.
#[ derive( Debug, Clone, Deserialize ) ]
pub struct SubscriptionConfig
{
  /// Update period.
  #[serde(with = "humantime_serde")]
  pub period : std::time::Duration,
  /// Resource link.
  pub link : String,
}

/// All subscriptions read from config file.
#[ derive( Debug, Deserialize ) ]
pub struct Subscriptions
{
  /// List of subscriptions configurations.
  pub config : Vec< SubscriptionConfig >
}

// qqq : don't name like that. ask
/// Reads provided configuration file with list of subscriptions.
pub fn read_feed_config( file_path : String ) -> Result< Vec< SubscriptionConfig >, Box< dyn std::error::Error + Send + Sync > >
{
  let read_file = OpenOptions::new().read( true ).open( &file_path )?;
  let mut reader = BufReader::new( read_file );
  let mut buffer: Vec< u8 > = Vec::new();
  reader.read_to_end( &mut buffer )?;

  let feeds : Subscriptions = toml::from_str( &String::from_utf8( buffer )? )?;

  Ok( feeds.config )
}
