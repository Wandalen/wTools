use async_trait::async_trait;
use feed_rs::parser as feed_parser;
use gluesql::
{
  core::{ chrono::{  DateTime, Utc} , data::Value },
  sled_storage::sled::Config,
};
use unitore::{ executor::FeedManager, feed_config::SubscriptionConfig, retriever::FeedFetch, storage::FeedStorage };
use wca::wtools::Itertools;
pub struct TestClient ( String );

#[ async_trait ]
impl FeedFetch for TestClient
{
  async fn fetch( &self, _ : String ) -> Result< feed_rs::model::Feed, Box< dyn std::error::Error + Send + Sync > >
  {
    let feed = feed_parser::parse( std::fs::read_to_string( &self.0 )?.as_bytes() )?;

    Ok( feed )
  }
}

#[ tokio::test ]
async fn test_update() -> Result< (), Box< dyn std::error::Error + Sync + Send > >
{
  let config = Config::default()
  .path( "./test".to_owned() )
  .temporary( true )
  ;

  let feed_storage = FeedStorage::init_storage( config ).await?;

  let feed_config = SubscriptionConfig
  {
    period : std::time::Duration::from_secs( 1000 ),
    link : String::from( "test" ),
  };

  let mut manager = FeedManager
  {
    storage : feed_storage,
    client : TestClient( "./tests/fixtures/plain_feed.xml".to_owned() ),
    config : vec![ feed_config ],
  };
  // initial fetch
  manager.update_feed().await?;

  manager.set_client( TestClient( "./tests/fixtures/updated_one_frame.xml".to_owned() ) );

  // updated fetch
  manager.update_feed().await?;

  // check
  let payload = manager.get_all_frames().await?;

  let entries = payload.selected_frames.selected_rows;

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
  assert!( entries.len() == 2 );

  // check date
  let updated = entries.iter().find( | ( id, _published ) | id == "https://www.nasa.gov/?p=622174" );
  assert!( updated.is_some() );
  let updated = updated.unwrap();

  assert_eq!( updated.1, DateTime::parse_from_str( "27 Feb 2024 19:42:10 +0000", "%d %b %Y %H:%M:%S %z" ).unwrap() );
  Ok( () )
}