//! Test for `config_add` functionality.

use std ::path ::PathBuf;
use gluesql_sled_storage ::sled ::Config;
use unitore ::
{
  sled_adapter ::FeedStorage,
  entity ::feed ::FeedStore,
  action ::config,
};
use error_tools ::untyped ::Result;

#[ tokio ::test ]
async fn config_add() -> Result< () >
{
  let path = PathBuf ::from( "./tests/fixtures/test_config.toml" );
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  config ::config_add( feed_storage.clone(), &path ).await?;

  let res = feed_storage.feeds_list().await?;

  let feeds_links = res.0.selected_rows
  .iter()
  .map( | feed | String ::from( feed[ 1 ].clone() ) )
  .collect :: < Vec< _ > >()
  ;

  assert!( feeds_links.len() == 1 );
  assert!( feeds_links.contains( &"https://www.nasa.gov/feed/".to_string() ) );

  Ok( () )
}
