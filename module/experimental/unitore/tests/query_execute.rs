//! Test for `query_execute` functionality.

use feed_rs ::parser as feed_parser;
use unitore ::
{
  feed_config ::SubscriptionConfig,
  sled_adapter :: { FeedStorage, Store },
  entity :: { config ::ConfigStore, feed ::FeedStore },
  action :: { query, config },
};
use gluesql ::
{
  prelude :: { Payload :: { self, Select }, Value :: { Str, Timestamp } },
  core ::chrono ::NaiveDateTime,
  gluesql_sled_storage ::sled,
};
use error_tools ::untyped ::Result;
use std ::path ::PathBuf;

#[ tokio ::test ]
async fn query_execute() -> Result< () >
{
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );
  let config = sled ::Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut storage = FeedStorage ::init_storage( &config ).await?;

  // Verify storage-level query returns correct structure on empty table.
  let result = storage.query_execute( "SELECT title FROM frame".to_string() ).await?;
  assert!( !result.0.is_empty() );
  let Select { labels, rows } = &result.0[ 0 ]
  else { panic!( "Expected Select payload" ); };
  assert_eq!( labels[ 0 ], "title" );
  assert!( rows.is_empty() );

  // Verify the action function delegates to storage correctly.
  let action_result = query ::query_execute( storage.clone(), "SELECT title FROM frame".to_string() ).await;
  assert!( action_result.is_ok() );

  Ok( () )
}

#[ tokio ::test ]
async fn query_feeds() -> Result< () >
{
  let path = PathBuf ::from( "./tests/fixtures/test_config.toml" );
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = sled ::Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  config ::config_add( feed_storage.clone(), &path ).await?;

  let entries = feed_storage.query_execute( "SELECT link FROM feed".to_string() ).await?;

  assert!( !entries.0.is_empty() );
  let Select { labels, rows } = &entries.0[ 0 ]
  else
  {
  panic!( "Expected Select payload" );
 };
  assert_eq!( labels.len(), 1 );
  assert_eq!( labels[ 0 ], "link" );
  assert_eq!( rows.len(), 1 );

  Ok( () )
}

#[ tokio ::test ]
async fn query_frames() -> Result< () >
{
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = sled ::Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  let feed_config = SubscriptionConfig
  {
  update_period: core ::time ::Duration ::from_secs( 1000 ),
  link: url ::Url ::parse( "https://www.nasa.gov/feed/" )?,
 };
  let mut feeds = Vec ::new();

  let feed = feed_parser ::parse( include_str!("./fixtures/plain_feed.xml").as_bytes() )?;
  feeds.push( ( feed, feed_config.update_period, feed_config.link.clone() ) );
  feed_storage.feeds_process( feeds ).await?;

  let entries = feed_storage.query_execute( "SELECT title, published FROM frame ORDER BY published".to_string() ).await?;

  assert!( !entries.0.is_empty() );

  let Select { labels, rows } = &entries.0[ 0 ]
  else
  {
  panic!( "Expected Select payload" );
 };
  assert_eq!( labels.len(), 2 );
  assert!( labels.contains( &String ::from( "title" ) ) );
  assert!( labels.contains( &String ::from( "published" ) ) );
  assert_eq!( rows.len(), 10 );
  assert_eq!( rows[ 0 ][ 0 ], Str( "8 Must-Have NASA Resources for Science Teachers in 2024".to_string() ) );
  assert_eq!( rows[ 0 ][ 1 ], Timestamp( NaiveDateTime ::parse_from_str( "13 Mar 2024 16:31:23", "%d %b %Y %H:%M:%S" )? ) );
  assert_eq!( rows[ 9 ][ 0 ], Str( "Icing Cloud Characterization Engineer Emily Timko".to_string() ) );
  assert_eq!( rows[ 9 ][ 1 ], Timestamp( NaiveDateTime ::parse_from_str( "14 Mar 2024 14:27:52", "%d %b %Y %H:%M:%S" )? ) );

  Ok( () )
}

#[ tokio ::test ]
async fn query_configs() -> Result< () >
{
  let path = PathBuf ::from( "./tests/fixtures/test_config.toml" );
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = sled ::Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  let _res = feed_storage.query_execute( format!( "INSERT INTO config VALUES ('{}') ", path.to_string_lossy() ) ).await?;
  let res = feed_storage.config_list().await?;

  let Payload ::Select{ labels, rows } = &res
  else
  {
  panic!( "Expected Select payload" );
 };
  assert_eq!( labels.len(), 1 );
  assert!( labels.contains( &String ::from( "path" ) ) );
  assert_eq!( rows.len(), 1 );
  assert_eq!( rows[ 0 ][ 0 ], Str( path.to_string_lossy().to_string() ) );

  Ok( () )
}
