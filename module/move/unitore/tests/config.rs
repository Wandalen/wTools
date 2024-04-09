use std::path::PathBuf;
use gluesql::
{
  sled_storage::sled::Config,
  prelude::Payload::Select,
};
use unitore::
{
  storage::FeedStorage,
  entity::{ feed::FeedStore, config::ConfigStore },
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
  config::config_add( feed_storage.clone(), &wca::Args( vec![ wca::Value::Path( path ) ] ) ).await?;

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

#[ tokio::test ]
async fn config_delete() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );

  let config = Config::default()
  .path( "./test_del".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  config::config_add( feed_storage.clone(), &wca::Args( vec![ wca::Value::Path( path.clone() ) ] ) ).await?;

  config::config_delete( feed_storage.clone(), &wca::Args( vec![ wca::Value::Path( path ) ] ) ).await?;

  let list = feed_storage.config_list().await?;

  if let Select{ labels : _, rows } = list
  {
    assert!( rows.len() == 0 )
  }
  else
  {
    assert!( false );
  }

  Ok( () )
}
