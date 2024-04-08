use async_trait::async_trait;
use feed_rs::parser as feed_parser;
use unitore::
{
  feed_config::SubscriptionConfig,
  retriever::FeedFetch,
  storage::{ FeedStorage, feed::FeedStore, Store, config::ConfigStore, MockStore },
  executor::actions::{ query::{ self, QueryReport } },
};
use gluesql::
{
  prelude::{ Payload::{ self, Select }, Value::{ Str, Timestamp } },
  core::chrono::NaiveDateTime,
  sled_storage::sled::Config,
};
use wca::{ VerifiedCommand, CommandsAggregator, Type };
use error_tools::Result;
use mockall::predicate;
use std::path::PathBuf;

/// Feed client for testing.
#[derive(Debug)]
pub struct TestClient;

#[ async_trait ]
impl FeedFetch for TestClient
{
  async fn fetch( &self, _ : url::Url ) -> Result< feed_rs::model::Feed >
  {
    let feed = feed_parser::parse( include_str!( "./fixtures/plain_feed.xml" ).as_bytes() )?;

    Ok( feed )
  }
}

#[ test ]
fn query_execute() -> Result< () >
{
  let rt  = tokio::runtime::Runtime::new()?;  
  let ca = CommandsAggregator::former()
  .command( "query.execute" )
    .hint( "hint" )
    .long_hint( "long_hint" )
    .subject().hint( "SQL query" ).kind( Type::String ).optional( false ).end()
    .routine( move | o : VerifiedCommand |
      {  
        let mut f_store = MockStore::new();
        f_store
        .expect_execute_query()
        .with(predicate::eq("SELECT title FROM frame".to_string()))
        .times( 1 )
        .returning( | _ | Ok( QueryReport
          (
            vec!
            [
              Select { labels : vec![ Str("title".to_string()).into() ], rows : Vec::new() }
            ]
          )
        ) )
        ;  
        _ = rt.block_on( async move
        {
          query::execute_query( f_store, &o.args ).await
        } );  
      } )
    .end()
  .perform();  
  let entries = ca.perform( vec![ ".query.execute".to_string(), "SELECT title FROM frame".into() ] );
  assert!( entries.is_ok() );

  Ok( () )
}

#[ tokio::test ]
async fn query_feeds() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );
  let path = path.canonicalize().expect( "Invalid path" );

  let config = Config::default()
  .path( "./test_feeds".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( config ).await?;
  unitore::executor::actions::config::add_config( feed_storage.clone(), &wca::Args( vec![ wca::Value::Path( path ) ] ) ).await?;

  let entries = feed_storage.execute_query( "SELECT link FROM feed".to_string() ).await?;

  assert!( !entries.0.is_empty() );
  if let Select { labels, rows } = &entries.0[ 0 ]
  {
    assert_eq!( labels.len(), 1 );
    assert_eq!( labels[ 0 ], "link" );
    assert_eq!( rows.len(), 1 );
  }
  else
  {
    assert!( false )
  }

  Ok( () )
}

#[ tokio::test ]
async fn query_frames() -> Result< () >
{
  let config = gluesql::sled_storage::sled::Config::default()
  .path( "./test_frames".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( config ).await?;

  let feed_config = SubscriptionConfig
  {
    update_period : std::time::Duration::from_secs( 1000 ),
    link : url::Url::parse( "https://www.nasa.gov/feed/" )?,
  };

  let mut feeds = Vec::new();
  let client = TestClient;

  let feed = FeedFetch::fetch( &client, feed_config.link.clone()).await?;
  feeds.push( ( feed, feed_config.update_period.clone(), feed_config.link.clone() ) );
  feed_storage.process_feeds( feeds ).await?;

  let entries = feed_storage.execute_query( "SELECT title, published FROM frame ORDER BY published".to_string() ).await?;

  assert!( !entries.0.is_empty() );

  if let Select { labels, rows } = &entries.0[ 0 ]
  {
    assert_eq!( labels.len(), 2 );
    assert!( labels.contains( &String::from( "title" ) ) );
    assert!( labels.contains( &String::from( "published" ) ) );
    assert_eq!( rows.len(), 10 );
    assert_eq!( rows[ 0 ][ 0 ], Str( "8 Must-Have NASA Resources for Science Teachers in 2024".to_string() ) );
    assert_eq!( rows[ 0 ][ 1 ], Timestamp( NaiveDateTime::parse_from_str( "13 Mar 2024 16:31:23", "%d %b %Y %H:%M:%S" )? ) );
    assert_eq!( rows[ 9 ][ 0 ], Str( "Icing Cloud Characterization Engineer Emily Timko".to_string() ) );
    assert_eq!( rows[ 9 ][ 1 ], Timestamp( NaiveDateTime::parse_from_str( "14 Mar 2024 14:27:52", "%d %b %Y %H:%M:%S" )? ) );
  }
  else
  {
    assert!( false )
  }

  Ok( () )
}

#[ tokio::test ]
async fn query_configs() -> Result< () >
{
  let path = PathBuf::from( "./tests/fixtures/test_config.toml" );
  let path = path.canonicalize().expect( "Invalid path" );

  let config = Config::default()
  .path( "./test_config".to_owned() )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( config ).await?;
  let _res = feed_storage.execute_query( format!( "INSERT INTO config VALUES ('{}') ", path.to_string_lossy().to_string() ) ).await?;


  let res = feed_storage.list_configs().await?;

  if let Payload::Select{ labels, rows } = &res
  {
    assert_eq!( labels.len(), 1 );
    assert!( labels.contains( &String::from( "path" ) ) );
    assert_eq!( rows.len(), 1 );
    assert_eq!( rows[ 0 ][ 0 ], Str( path.to_string_lossy().to_string() ) );
  }
  else
  {
    assert!( false );
  }
  
  Ok( () )
}
