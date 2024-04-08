//! Execute plan.

use crate::*;
use feed_config::SubscriptionConfig;
use storage::Store;
use entity::{ feed::FeedStore, config::ConfigStore, table::TableStore, frame::FrameStore };
use wca::{ Dictionary, Executor, Parser, Verifier };
use error_tools::Result;

use action::Report;

/// Run feed updates.
pub fn execute() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  // init parser
  let parser = Parser;

  // init converter
  let dictionary = &Dictionary::former()
  .command
  (
    command::config::ConfigCommand::add()?
  )
  .command
  (
    command::config::ConfigCommand::delete()?
  )
  .command
  (
    command::config::ConfigCommand::list()?
  )
  .command
  (
    command::frame::FrameCommand::list()?
  )
  .command
  (
    command::frame::FrameCommand::download()?
  )
  .command
  (
    command::feed::FeedCommand::list()?
  )
  .command
  (
    command::table::TablesCommand::list()?
  )
  .command
  (
    command::table::TableCommand::list()?
  )
  .command
  (
    command::query::QueryCommand::list()?
  )
  .form();
  let verifier = Verifier;

  // init executor
  let executor = Executor::former().form();

  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
  let raw_program = parser.parse( args ).unwrap();
  let grammar_program = verifier.to_program( dictionary, raw_program ).unwrap();

  executor.program( dictionary, grammar_program )?;

  Ok( () )
}

/// Manages feed subsriptions and updates.
pub struct FeedManager< S : FeedStore + ConfigStore + FrameStore + Store + Send >
{
  /// Subscription configuration with link and update period.
  pub config : Vec< SubscriptionConfig >,
  /// Storage for saving feed.
  pub storage : S,
}

impl< S : FeedStore + ConfigStore + FrameStore + Store + Send > std::fmt::Debug for FeedManager< S >
{
  fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!(f, "Feed manager with storage and client" )
  }
}

impl< S : FeedStore + ConfigStore + FrameStore + TableStore + Store + Send > FeedManager< S >
{
  /// Create new instance of FeedManager.
  pub fn new( storage : S ) -> FeedManager< S >
  {
    Self
    {
      storage,
      config : Vec::new(),
    }
  }
}

impl< S : FeedStore + ConfigStore + FrameStore + TableStore + Store + Send > FeedManager< S >
{
  /// Set configurations for subscriptions.
  pub fn set_config( &mut self, configs : Vec< SubscriptionConfig > )
  {
    self.config = configs;
  }

  /// Execute custom query, print result.
  pub async fn execute_custom_query( &mut self, query : String ) -> Result< impl Report >
  {
    self.storage.execute_query( query ).await
  }

}
