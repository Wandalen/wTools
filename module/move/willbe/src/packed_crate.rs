mod private
{
  use crate::*;
  
  use std::path::PathBuf;
  use wtools::error::Result;

  /// Returns the local path of a packed `.crate` file based on its name, version, and manifest path.
  ///
  /// # Args:
  /// - `name` - the name of the package.
  /// - `version` - the version of the package.
  /// - `manifest_path` - path to the package `Cargo.toml` file.
  ///
  /// # Returns:
  /// The local packed `.crate` file of the package
  pub fn local_path< 'a >( name : &'a str, version : &'a str, crate_dir: CrateDir ) -> Result< PathBuf >
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );

    let workspace = Workspace::with_crate_dir( crate_dir )?;

    let mut local_package_path = PathBuf::new();
    local_package_path.push( workspace.target_directory()? );
    local_package_path.push( buf );

    Ok( local_package_path )
  } 
}

//

crate::mod_interface!
{
  
  protected use local_path;
  
}
