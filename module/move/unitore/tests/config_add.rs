use std::path::PathBuf;
use gluesql::sled_storage::sled::Config;
use unitore::
{
  sled_adapter::FeedStorage,
  entity::feed::FeedStore,
  action::config,
};
use error_tools::Result;

#[ tokio::test ]
async fn config_add() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );

  let config = Config::default()
  .path( "./test_add".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  config::config_add( feed_storage.clone(), &path ).await?;

  let res = feed_storage.feeds_list().await?;

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
