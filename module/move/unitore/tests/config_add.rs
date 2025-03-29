use std::path::PathBuf;
use gluesql::sled_storage::sled::Config;
use unitore::
{
  sled_adapter::FeedStorage,
  entity::feed::FeedStore,
  action::config,
};
use error_tools::untyped::Result;

#[ tokio::test ]
async fn config_add() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );
  let temp_path = pth::path::unique_folder_name().unwrap();

  let config = Config::default()
  .path( format!( "./{}", temp_path ) )
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

  assert!( feeds_links.len() == 1 );
  assert!( feeds_links.contains( &format!( "https://www.nasa.gov/feed/" ) ) );

  Ok( () )
}
