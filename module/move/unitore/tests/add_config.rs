use std::path::PathBuf;

use gluesql::sled_storage::sled::Config;
use unitore::
{
  executor::FeedManager,
  storage::{ FeedStorage, feed::FeedStore },
};
use error_tools::Result;

#[ tokio::test ]
async fn add_config_file() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );
  let path = path.canonicalize().expect( "Invalid path" );

  let config = Config::default()
  .path( "./test".to_owned() )
  .temporary( true )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;
  unitore::executor::actions::config::add_config( feed_storage.clone(), &wca::Args( vec![ wca::Value::Path( path ) ] ) ).await?;

  let mut manager = FeedManager::new( feed_storage );
  let res = manager.storage.get_all_feeds().await?;

  let feeds_links = res.0.selected_rows
  .iter()
  .map( | feed | String::from( feed[ 1 ].clone() ) )
  .collect::< Vec< _ > >()
  ;

  assert!( feeds_links.len() == 2 );
  assert!( feeds_links.contains( &format!( "https://feeds.bbci.co.uk/news/world/rss.xml" ) ) );
  assert!( feeds_links.contains( &format!( "https://rss.nytimes.com/services/xml/rss/nyt/World.xml" ) ) );

  Ok( () )
}
