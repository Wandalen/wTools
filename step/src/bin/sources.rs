//! List all sources

use willbe::exposed::*;

use willbe::wtools::error::Result;
use std::path::Path;

fn main() -> Result< () >
{
  let workspace = Workspace::from_current_path()?;

  // let metadata = workspace.metadata.as_ref().unwrap();
  // dbg!( &metadata.packages.len() );
  // dbg!( &metadata.workspace_members.len() );
  // dbg!( &metadata.workspace_members );
  // dbg!( &metadata.packages[ 0 ] );

  workspace
  .packages()
  .for_each( | package |
  {
    println!( "{}", package.name() )
  });

  // // Iterate over all packages in the workspace
  // for package in &metadata.packages
  // {
  //   println!( "\nPackage {}", package.name );
  //   // Iterate over all targets (e.g., lib, bin, examples, tests, benches)
  //   for target in &package.targets
  //   {
  //     // Iterate over all source files in each target
  //     println!( " - Target {}", target.name );
  //     for src_path in target.src_path.iter()
  //     {
  //       let path = Path::new( &src_path );
  //       if path.extension().map_or( false, | ext | ext == "rs" )
  //       {
  //         println!( " -- Source {} {:?}", path.display(), target.kind );
  //       }
  //     }
  //   }
  // }

  return Ok( () );
}
