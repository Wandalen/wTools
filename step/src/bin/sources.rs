//! List all sources

use willbe::exposed::*;
use willbe::{ Sources, Entries };

fn main() -> Result< () >
{
  let workspace = Workspace::try_from( CurrentPath )?;

  let package = workspace
  .packages_which()
  .crate_dir( CrateDir::transitive_try_from::< AbsolutePath >( CurrentPath )? )
  .find()
  .expect( "No workspace at current path" )
  ;

  println!( " = package - {}", package.crate_dir().unwrap() );

  package.sources().for_each( | source |
  {
    println!( "   - {source}" );
  });

  dbg!( &workspace.crate_dir );

  return Ok( () );
}
