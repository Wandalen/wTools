use std::path::PathBuf;

use gluesql::sled_storage::sled::Config;
use unitore::{
  executor::FeedManager, storage::FeedStorage
};
use unitore::storage::FeedStore;

#[ tokio::test ]
async fn add_config_file() -> Result< (), Box< dyn std::error::Error + Sync + Send > >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );
  //println!("{:?}", res);
  let path = path.canonicalize().expect( "Invalid path" );

  let config = Config::default()
  .path( "./test".to_owned() )
  .temporary( true )
  ;

  // unitore::executor::endpoints::config::add_config( path.clone() )?;

  let feed_storage = FeedStorage::init_storage( config ).await?;

  let mut manager = FeedManager::new( feed_storage );
  manager.storage.add_config( path.to_string_lossy().to_string() ).await?;

  let res = manager.get_all_feeds().await?;

  let feeds_links = res.selected_entries.selected_rows
  .iter()
  .map( | feed | String::from( feed[ 2 ].clone() ) )
  .collect::< Vec< _ > >()
  ;

  println!( "{:?}", res );

  // assert!( feeds_links.len() == 2 );
  // assert!( feeds_links.contains( &format!( "https://feeds.bbci.co.uk/news/world/rss.xml" ) ) );
  // assert!( feeds_links.contains( &format!( "https://rss.nytimes.com/services/xml/rss/nyt/World.xml" ) ) );
  println!("{:?}", feeds_links);

//   let mut manager = FeedManager
//   {
//     storage : f_store,
//     client : TestClient,
//     config : vec![],
//   };
//   manager.update_feed( vec![ feed_config ] ).await?;

  Ok( () )
}
