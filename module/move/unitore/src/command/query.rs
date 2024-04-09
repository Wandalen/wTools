//! Query command.

use crate::*;
use gluesql::sled_storage::sled::Config;
use wca::{ Command, Type, VerifiedCommand };
use storage::FeedStorage;
use action::{ Report, query::query_execute };
use error_tools::Result;

/// Struct that provides commands for queries.
#[ derive( Debug ) ]
pub struct QueryCommand;

impl QueryCommand
{
  /// Creates command for custom query execution.
  pub fn execute() -> Result< Command >
  {
    let rt  = tokio::runtime::Runtime::new()?;
  
    Ok
    (
      Command::former()
      .phrase( "query.execute" )
      .long_hint( concat!
      (
        "Execute custom query. Subject: query string, with special characters escaped.\n",
        "    Example query:\n",
        "  - select all frames:\n",
        r#"    .query.execute \'SELECT \* FROM frame\'"#,
        "\n",
        "  - select title and link to the most recent frame:\n",
        r#"    .query.execute \'SELECT title, links, MIN\(published\) FROM frame\'"#,
        "\n\n",
      ))
      .subject().hint( "Query" ).kind( Type::String ).optional( false ).end()
      .routine( move | o : VerifiedCommand |
      {
        let res = rt.block_on( async move
          {
            let path_to_storage = std::env::var( "UNITORE_STORAGE_PATH" )
            .unwrap_or( String::from( "./_data" ) )
            ;
            
            let config = Config::default()
            .path( path_to_storage )
            ;
  
            let feed_storage = FeedStorage::init_storage( &config ).await?;
            query_execute( feed_storage, &o.args ).await
          });
          match res
          {
            Ok( report ) => report.report(),
            Err( err ) => println!( "{:?}", err ),
          }
        
      })
      .end()
    )
  }
}