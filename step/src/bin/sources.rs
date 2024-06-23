//! List all sources

use willbe::exposed::*;
use willbe::{ Sources, Entries };
// use willbe::error::Result;
use std::path::Path;

fn main() -> Result< () >
{
  let workspace = Workspace::try_from( CurrentPath )?;

  // let metadata = workspace.metadata.as_ref().unwrap();
  // dbg!( &metadata.packages.len() );
  // dbg!( &metadata.workspace_members.len() );
  // dbg!( &metadata.workspace_members );
  // dbg!( &metadata.packages[ 0 ] );

  // workspace
  // .packages()
  // .for_each( | package |
  // {
  //   println!( "{}", package.name() )
  // });

  // dbg!( &workspace.crate_dir );
  // let sources = workspace.entries();
  // sources.collect::< Vec< _ > >();

  dbg!( &workspace.crate_dir );

  let sources = workspace.sources();
  sources.for_each( | source |
  {
    println!( " - {source}" );
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
