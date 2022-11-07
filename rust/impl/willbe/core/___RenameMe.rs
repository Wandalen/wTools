/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use crate::*;

  /// Iterate over all packages by PathBuf
  pub fn packages_iterate( path : PathBuf, order : OrderStrategy ) -> Box< dyn Iterator< Item = Package > >
  {
    if let Ok( package ) = Package::try_from( path.to_owned() )
    {
      return Box::new( Some( package ).into_iter() )
    }

    if let Ok( workspace ) = Workspace::try_from( path.to_owned() )
    {
      return Box::new( workspace.packages_iterate( order ) )
    }

    Box::new( None.into_iter() )
  }

  /// Iterate over workspace iterator
  pub fn workspaces_packages_iterate( workspaces : impl Iterator< Item = Workspace >, order : OrderStrategy ) -> impl Iterator< Item = Package >
  {
    workspaces.flat_map( move | workspace | workspace.packages_iterate( order ) )
  }
}

//

wtools::meta::mod_interface!
{
  prelude use packages_iterate;
  prelude use workspaces_packages_iterate;
}
