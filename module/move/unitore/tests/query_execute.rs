use feed_rs::parser as feed_parser;
use unitore::
{
  feed_config::SubscriptionConfig,
  sled_adapter::{ FeedStorage, Store, MockStore },
  entity::{ config::ConfigStore, feed::FeedStore },
  action::{ query::{ self, QueryReport }, config },
  command::query::QueryCommand,
};
use gluesql::
{
  prelude::{ Payload::{ self, Select }, Value::{ Str, Timestamp } },
  core::chrono::NaiveDateTime,
  sled_storage::sled,
};
use wca::{ VerifiedCommand, CommandsAggregator, Type, Parser, Dictionary, Verifier, Executor };
use error_tools::Result;
use mockall::predicate;
use std::path::PathBuf;

#[ test ]
fn query_execute() -> Result< () >
{
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary::former()
  .command( QueryCommand::execute()? )
  .form()
  ;
  let verifier = Verifier;

  // init executor
  let executor = Executor::former().form();
  let args = vec![ ".query.execute".to_string(), "SELECT title FROM frame".into() ];
  let raw_program = parser.parse( args ).unwrap();
  let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();

  let res = executor.program( dictionary, grammar_program );
  assert!( res.is_ok() );

  // test action
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
        .expect_query_execute()
        .with( predicate::eq( "SELECT title FROM frame".to_string() ) )
        .times( 1 )
        .returning( | _ | Ok( QueryReport
          (
            vec!
            [
              Select { labels : vec![ Str( "title".to_string() ).into() ], rows : Vec::new() }
            ]
          )
        ) )
        ;  
        _ = rt.block_on( async move
        {
          let query_arg = o.args
          .get_owned::< String >( 0 )
          ;
  
          let query_str = query_arg.unwrap();
          query::query_execute( f_store, query_str ).await
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
  let temp_path = proper_path_tools::path::unique_folder_name().unwrap();

  let config = sled::Config::default()
  .path( format!( "./{}", temp_path ) )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  config::config_add( feed_storage.clone(), &path ).await?;

  let entries = feed_storage.query_execute( "SELECT link FROM feed".to_string() ).await?;

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
  let temp_path = proper_path_tools::path::unique_folder_name().unwrap();

  let config = sled::Config::default()
  .path( format!( "./{}", temp_path ) )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  let feed_config = SubscriptionConfig
  {
    update_period : std::time::Duration::from_secs( 1000 ),
    link : url::Url::parse( "https://www.nasa.gov/feed/" )?,
  };
  let mut feeds = Vec::new();

  let feed = feed_parser::parse( include_str!("./fixtures/plain_feed.xml").as_bytes() )?;
  feeds.push( ( feed, feed_config.update_period.clone(), feed_config.link.clone() ) );
  feed_storage.feeds_process( feeds ).await?;

  let entries = feed_storage.query_execute( "SELECT title, published FROM frame ORDER BY published".to_string() ).await?;

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
  let temp_path = proper_path_tools::path::unique_folder_name().unwrap();

  let config = sled::Config::default()
  .path( format!( "./{}", temp_path ) )
  .temporary( true )
  ;

  let mut feed_storage = FeedStorage::init_storage( &config ).await?;
  let _res = feed_storage.query_execute( format!( "INSERT INTO config VALUES ('{}') ", path.to_string_lossy().to_string() ) ).await?;
  let res = feed_storage.config_list().await?;

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
