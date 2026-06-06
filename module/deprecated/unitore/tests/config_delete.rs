//! Test for `config_delete` functionality.

use gluesql ::
{
  gluesql_sled_storage ::sled ::Config,
  prelude ::Payload ::Select,
};
use unitore ::
{
  sled_adapter ::FeedStorage,
  entity ::config ::ConfigStore,
  action ::config,
};
use error_tools ::untyped ::Result;

#[ tokio ::test ]
async fn config_delete() -> Result< () >
{

  let path = std ::path ::PathBuf ::from( "./tests/fixtures/test_config.toml" );
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  config ::config_add( feed_storage.clone(), &path ).await?;

  config ::config_delete( feed_storage.clone(), &path ).await?;

  let list = feed_storage.config_list().await?;

  let Select{ labels: _, rows } = list
  else
  {
  panic!( "Expected Select payload" );
 };
  assert!( rows.is_empty() );

  Ok( () )
}
