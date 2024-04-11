use std::path::PathBuf;
use gluesql::
{
  sled_storage::sled::Config,
  prelude::Payload::Select,
};
use unitore::
{
  sled_adapter::FeedStorage,
  entity::config::ConfigStore,
  action::config,
};
use error_tools::Result;

#[ tokio::test ]
async fn config_delete() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );

  let config = Config::default()
  .path( "./test_del".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  config::config_add( feed_storage.clone(), &path ).await?;

  config::config_delete( feed_storage.clone(), &path ).await?;

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
