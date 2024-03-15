use async_trait::async_trait;
use feed_rs::parser as feed_parser;
use gluesql::
{
  core::
  { 
    chrono::{ DateTime, Utc },
    data::Value
  },
  sled_storage::sled::Config,
};
use unitore::
{
  executor::FeedManager,
  feed_config::SubscriptionConfig,
  retriever::FeedFetch,
  storage::{ FeedStorage, frame::FrameStore },
};
use wca::wtools::Itertools;
use error_tools::Result;

/// Feed client for testing.
#[derive(Debug)]
pub struct TestClient ( String );

#[ async_trait ]
impl FeedFetch for TestClient
{
  async fn fetch( &self, _ : String ) -> Result< feed_rs::model::Feed >
  {
    let feed = feed_parser::parse( std::fs::read_to_string( &self.0 )?.as_bytes() )?;
    Ok( feed )
  }
}

#[ tokio::test ]
async fn test_update() -> Result< () >
{
  let config = Config::default()
  .path( "./test".to_owned() )
  .temporary( true )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;

  let feed_config = SubscriptionConfig
  {
    update_period : std::time::Duration::from_secs( 1000 ),
    link : String::from( "test" ),
  };

  let mut manager = FeedManager
  {
    storage : feed_storage,
    client : TestClient( "./tests/fixtures/plain_feed.xml".to_owned() ),
    config : vec![],
  };
  // initial fetch
  manager.update_feed( vec![ feed_config.clone() ] ).await?;

  manager.set_client( TestClient( "./tests/fixtures/updated_one_frame.xml".to_owned() ) );

  // updated fetch
  manager.update_feed( vec![ feed_config ] ).await?;
  // check
  let payload = manager.storage.list_frames().await?;

  let entries = payload.0.iter().map( | val | val.selected_frames.selected_rows.clone() ).flatten().collect::< Vec< _ > >();

  let entries = entries.iter().map( | entry |
    {
      let id = match &entry[ 0 ]
      {
        Value::Str( s ) => s.to_owned(),
        _ => String::new(),
      };

      let published = match &entry[ 8 ]
      {
        Value::Timestamp( date_time ) => date_time.and_utc(),
        _ => DateTime::< Utc >::default(),
      };
      ( id, published )
    }
  )
  .collect_vec()
  ;

  // no duplicates
  assert_eq!( entries.len(), 10 );

  // check date
  println!( "{:?}", entries );
  let updated = entries.iter().find( | ( id, _published ) | id == "https://www.nasa.gov/?post_type=image-article&p=631537" );
  assert!( updated.is_some() );
  let _updated = updated.unwrap();
  Ok( () )
}