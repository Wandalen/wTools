use gluesql::
{
    sled_storage::sled::Config,
    prelude::{ Payload, Value::Str },
};
use unitore::
{
  sled_adapter::FeedStorage,
  entity::table::TableStore,
};
use error_tools::Result;

#[ tokio::test ]
async fn table_list() -> Result< () >
{
  let temp_path = proper_path_tools::path::unique_folder_name().unwrap();

  let config = Config::default()
  .path( format!( "./{}", temp_path ) )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  let res = feed_storage.table_list( String::from( "feed" ) ).await?;

  if let Payload::Select { labels: _, rows } = &res[ 0 ]
  {
    let column_names = rows
    .iter()
    .map( | row | row[ 1 ].clone() )
    .collect::< Vec< _ > >()
    ;
  
    assert_eq!( column_names.len(), 9 );
    assert!( column_names.contains( &Str( String::from( "published") ) ) );
    assert!( column_names.contains( &Str( String::from( "authors") ) ) );
    assert!( column_names.contains( &Str( String::from( "description") ) ) );
    assert!( column_names.contains( &Str( String::from( "type") ) ) );
    assert!( column_names.contains( &Str( String::from( "title") ) ) );
    assert!( column_names.contains( &Str( String::from( "updated") ) ) );
    assert!( column_names.contains( &Str( String::from( "link") ) ) );
    assert!( column_names.contains( &Str( String::from( "update_period" ) ) ) );
    assert!( column_names.contains( &Str( String::from( "config_file" ) ) ) );
  }

  Ok( () )
}
