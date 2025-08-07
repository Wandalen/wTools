use gluesql::sled_storage::sled::Config;
use unitore::
{
  sled_adapter::FeedStorage,
  entity::table::TableStore,
};
use error_tools::untyped::Result;

#[ tokio::test ]
async fn tables_list() -> Result< () >
{
  let temp_path = pth::path::unique_folder_name().unwrap();

  let config = Config::default()
  .path( format!( "./{}", temp_path ) )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  let res = feed_storage.tables_list().await?;

  let table_names = res.0
  .iter()
  .map( | ( table_name, _info ) | table_name )
  .collect::< Vec< _ > >()
  ;

  assert_eq!( table_names.len(), 3 );
  assert!( table_names.contains( &&String::from( "config") ) );
  assert!( table_names.contains( &&String::from( "feed" ) ) );
  assert!( table_names.contains( &&String::from( "frame" ) ) );

  Ok( () )
}
