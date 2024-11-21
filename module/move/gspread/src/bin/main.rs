use std::error::Error;
use std::env;
use clap::Parser;
use dotenv::dotenv;

use gspread::
{
  client::hub,
  commands::{ Cli, CliCommand, self },
  secret::Secret,
};

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn Error > >
{
  dotenv().ok();

  let secret = Secret::load()?;

  let hub = hub( &secret ).await?;

  let cli = Cli::parse();

  match cli.command
  {
    CliCommand::GSpread( cmd ) =>
    {
      commands::gspread::command( &hub, cmd ).await;
    }
  }

  Ok( () )
}
