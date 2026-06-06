//! Test for `tables_list` functionality.

use gluesql_sled_storage ::sled ::Config;
use unitore ::
{
  sled_adapter ::FeedStorage,
  entity ::table ::TableStore,
};
use error_tools ::untyped ::Result;

#[ tokio ::test ]
async fn tables_list() -> Result< () >
{
  let temp_path = std ::env ::temp_dir().join( pth ::path ::unique_folder_name().unwrap() );

  let config = Config ::default()
  .path( temp_path.to_string_lossy().to_string() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage ::init_storage( &config ).await?;
  let res = feed_storage.tables_list().await?;

  let table_names = res.0.keys()
  .collect :: < Vec< _ > >()
  ;

  assert_eq!( table_names.len(), 3 );
  assert!( table_names.contains( &&String ::from( "config") ) );
  assert!( table_names.contains( &&String ::from( "feed" ) ) );
  assert!( table_names.contains( &&String ::from( "frame" ) ) );

  Ok( () )
}
