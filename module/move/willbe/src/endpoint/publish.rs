/// Internal namespace.
mod private
{
  use crate::package::functions as package;

  use crate::tools::
  {
    files,
    manifest,
    path
  };
  use crate::wtools;
  use wtools::error::Result;
  use std::
  {
    env,
    path::PathBuf,
  };
  use cargo_metadata::
  {
    MetadataCommand,
  };

  ///
  /// Publish package.
  ///

  pub fn publish( patterns : Vec< String >, dry : bool ) -> Result< () >
  {
    let current_path = env::current_dir().unwrap();

    let paths = files::find( &current_path, &patterns );
    let mut paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s.into() ) } else { None } ).collect::< Vec< PathBuf > >();
    if !patterns.is_empty() && paths.is_empty() && path::valid_is( &patterns[ 0 ] )
    {
      paths.push( PathBuf::from( &patterns[ 0 ] ) );
    }

    for path in paths
    {
      package::publish( &current_path, &path, dry )?;
    }

    Ok( () )
  }

  ///
  /// Publish packages from workspace.
  ///

  pub fn workspace_publish( path_to_workspace : PathBuf, dry : bool ) -> Result< () >
  {
    let current_path = env::current_dir().unwrap();

    let mut manifest = manifest::Manifest::new();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace ).unwrap();
    let package_metadata = MetadataCommand::new()
      .manifest_path( &manifest_path )
      .no_deps()
      .exec()
      .unwrap();

    let packages_map = package::filter( &package_metadata );
    let sorted = package::toposort( &packages_map );

    for name in sorted.iter()
    {
      package::publish( &current_path, &packages_map[ name ].manifest_path.clone().into(), dry )?;
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  /// Publish package.
  prelude use publish;
  /// Publish packages from workspace.
  prelude use workspace_publish;
}
