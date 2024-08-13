//! Execute plan.

use crate::*;
use wca::{ Dictionary, Executor, Parser, Verifier };
use error_tools::untyped::Result;

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
    command::query::QueryCommand::execute()?
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
