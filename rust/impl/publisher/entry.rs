#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

use std::env;
// use wca::*;
use wpublisher::*;
// use std::collections::HashMap;

// fn publish_package()
// {
//   println!( "Publishing of package." );
// }
//
// fn commands_map_form( commands_map : &mut HashMap<&'static str, CommandOptions> )
// {
//   let mut publish_command = CommandOptions::default();
//   publish_command.hint( "Publish package" )
//   .long_hint( "Publish package" )
//   .subject_hint( "A path to root of package." )
//   .property_hint( "dry", "Run command dry, no publish." )
//   .property_hint( "logger", "A level of command verbosity. Default is 2." )
//   .routine( &publish_package );
//
//   commands_map.insert( ".publish", publish_command );
// }

fn main() -> anyhow::Result<()>
{
  // let mut commands_map: HashMap<&'static str, CommandOptions> = HashMap::new();
  // commands_map_form( &mut commands_map );

  let instruction = wpublisher::instruction::parse_from_splits( env::args().skip( 1 ) );
  println!( "{:?}", instruction );

  let manifest = manifest_get( instruction.subject[ 0 ].as_ref() )?;
  let data = manifest.manifest_data.as_ref().unwrap();
  // let package = data.package.as_ref().unwrap();
  println!("{}", data.to_string() );
  manifest.store()?;

  // let command = commands_map.get( instruction.command_name.as_ref() );
  // if command.is_some()
  // {
  //   let command = command.unwrap().form();
  //   println!( "{:#?}", command );
  // }

  Ok( () )
}

fn manifest_get( path : &str ) -> anyhow::Result<manifest::Manifest>
{
  let mut manifest = manifest::Manifest::new();
  manifest.manifest_path_from_str( path )?;
  manifest.load()?;
  Ok( manifest )
}
