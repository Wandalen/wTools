//! List all sources

use willbe::exposed::*;
use willbe::wtools::error::Result;

fn main() -> Result< () >
{

  let workspace = Workspace::from_current_path()?;

  println!( "experiment" );

  return Ok( () );
}