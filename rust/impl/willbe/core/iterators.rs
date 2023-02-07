/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use crate::*;

  /// Recursive iterate over all packages by PathBuf
  pub fn packages_recursive_iterate( path : PathBuf, depth : std::ops::Range< usize > ) -> Box< dyn Iterator< Item = Package > >
  {
    Box::new
    (
      unique_walk( path, &[ "Cargo.toml" ], depth )
      .filter_map( | path | path.parent().map( | p | p.to_path_buf() ) )
      .filter_map( | path | Package::try_from( path ).ok() )
    )
  }

  /// Iterate over all packages by PathBuf
  pub fn packages_iterate( path : PathBuf ) -> Box< dyn Iterator< Item = Package > >
  {
    if let Ok( package ) = Package::try_from( path.to_owned() )
    {
      return Box::new( Some( package ).into_iter() )
    }

    if let Ok( workspace ) = Workspace::try_from( path )
    {
      return Box::new( workspace.packages_iterate() )
    }

    Box::new( None.into_iter() )
  }

  /// Iterate over workspaces iterator
  pub fn workspaces_packages_iterate( workspaces : impl Iterator< Item = Workspace > ) -> impl Iterator< Item = Package >
  {
    workspaces.flat_map( move | workspace | workspace.packages_iterate() )
  }
}

//

wtools::mod_interface!
{
  prelude use packages_iterate;
  prelude use packages_recursive_iterate;
  prelude use workspaces_packages_iterate;
}
