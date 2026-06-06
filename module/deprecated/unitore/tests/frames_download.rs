//! Test for `frames_download` functionality.

use feed_rs ::parser as feed_parser;
use gluesql ::
{
  core ::
  {
  chrono :: { DateTime, Utc },
  data ::Value
 },
  gluesql_sled_storage ::sled ::Config,
};
use itertools ::Itertools;
use unitore ::
{
  feed_config ::SubscriptionConfig,
  sled_adapter ::FeedStorage,
  entity :: { frame ::FrameStore, feed ::FeedStore },
};
use error_tools ::untyped ::Result;

#[ tokio ::test ]
async fn test_save() -> Result< () >
{
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = Config ::default()
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

  let entries = feed_storage.frames_list().await?;

  let number_of_frames = entries.0[ 0 ].selected_frames.selected_rows.len();
  assert_eq!( number_of_frames, 10 );

  Ok( () )
}

#[ tokio ::test ]
async fn test_update() -> Result< () >
{
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;

  let feed_config = SubscriptionConfig
  {
  update_period: core ::time ::Duration ::from_secs( 1000 ),
  link: url ::Url ::parse( "https://www.nasa.gov/feed/" )?,
 };

  // initial fetch
  let feed = feed_parser ::parse( include_str!("./fixtures/plain_feed.xml").as_bytes() )?;
  let feeds = vec![ ( feed, feed_config.update_period, feed_config.link.clone() ) ];
  feed_storage.feeds_process( feeds ).await?;

  // updated fetch
  let feed = feed_parser ::parse( include_str!("./fixtures/updated_one_frame.xml").as_bytes() )?;

  let feeds = vec![ ( feed, feed_config.update_period, feed_config.link.clone() ) ];
  feed_storage.feeds_process( feeds ).await?;

  // check
  let payload = feed_storage.frames_list().await?;

  let entries = payload.0
  .iter()
  .flat_map(| val | val.selected_frames.selected_rows.clone())
  .collect :: < Vec< _ > >()
  ;

  let entries = entries.iter().map( | entry |
  {
   let id = match &entry[ 0 ]
   {
  Value ::Str( s ) => s.to_owned(),
  _ => String ::new(),
 };

   let published = match &entry[ 8 ]
   {
  Value ::Timestamp( date_time ) => date_time.and_utc(),
  _ => DateTime :: < Utc > ::default(),
 };
   ( id, published )
 }
 )
  .collect_vec()
  ;

  // no duplicates
  assert_eq!( entries.len(), 10 );

  // check date
  let updated = entries.iter().find
  (
  | ( id, _published ) | id == "https://www.nasa.gov/?post_type=image-article&p=631537"
 );
  assert!( updated.is_some() );
  let _updated = updated.unwrap();
  Ok( () )
}
