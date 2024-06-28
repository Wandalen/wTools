//! List all sources

use willbe::exposed::*;
use willbe::{ Entries, Sources, Items };

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

    source
    .items()
    .for_each( | item |
    {
      println!( "     - {}", std::any::type_name_of_val( &item ) );
      // println!( "     - item : {item:?}" );
    });

  });

  dbg!( &workspace.crate_dir );

  return Ok( () );
}
