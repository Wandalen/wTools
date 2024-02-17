// use super::*;
use std::{ fs::OpenOptions, io::{ BufReader, Read } };
use serde::Deserialize;

#[ derive( Debug, Deserialize ) ]
pub struct FeedConfig
{
//   pub name : String,
  #[serde(with = "humantime_serde")]
  pub period : std::time::Duration,
  pub link : String,
}

#[ derive( Debug, Deserialize ) ]
pub struct Feeds
{
  pub config : Vec< FeedConfig >
}

pub fn read_feed_config() -> Result< Vec< FeedConfig >, Box< dyn std::error::Error > >
{
  let path = format!( "./config/feeds.toml" );
  // qqq : parametrize

  let read_file = OpenOptions::new().read( true ).open( &path )?;
  let mut reader = BufReader::new( read_file );
  let mut buffer: Vec< u8 > = Vec::new();
  reader.read_to_end( &mut buffer )?;

  let feeds : Feeds = toml::from_str( &String::from_utf8( buffer )? )?;

  // println!( "{:#?}", feeds );

  Ok( feeds.config )
}

