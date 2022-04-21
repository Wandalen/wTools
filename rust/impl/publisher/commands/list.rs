use crate::*;
use std::env;
use wtools::error::Error;

///
/// Publish package.
///

pub fn list( instruction : &instruction::Instruction ) -> Result<(), Error>
{
  let current_path = env::current_dir().unwrap();

  let paths = files::find( current_path, instruction.subject.split( " " ).collect::<Vec<&str>>().as_slice() );
  let paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s ) } else { None } );

  for path in paths
  {
    let manifest = manifest_get( path );
    let data = manifest.manifest_data.as_ref().unwrap();
    if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
    {
      let remote = data[ "package" ].get( "publish" ).is_none()
                   || data[ "package" ][ "publish" ].as_bool().unwrap() == true;
      let remote = if remote { "remote" } else { "local" };
      println!( "{} - {:?}, {}", data[ "package" ][ "name" ].to_string().trim(), path.parent().unwrap(), remote );
    }
  }

  Ok( () )
}

//

fn manifest_get( path : &std::path::Path ) -> manifest::Manifest
{
  let mut manifest = manifest::Manifest::new();
  manifest.manifest_path = path.into();
  manifest.load().unwrap();
  manifest
}
