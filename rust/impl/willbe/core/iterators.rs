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

    if let Ok( workspace ) = Workspace::try_from( path )
    {
      return Box::new( workspace.packages_iterate( order ) )
    }

    Box::new( None.into_iter() )
  }

  /// Iterate over workspaces iterator
  pub fn workspaces_packages_iterate( workspaces : impl Iterator< Item = Workspace >, order : OrderStrategy ) -> impl Iterator< Item = Package >
  {
    // TODO: Improve. Ordering through workspaces
    //? Problem:
    // At the moment, packages are sorted only in their own workspaces
    // If a package from first workspace will require a package from another workspace, it will not be processed correctly
    // ! Possible solution:
    // Split iteration and ordering
    // + single responsibility(iteration do iteration only)
    // + ordering implements once(easier to test)
    // - call two functions instead one
    workspaces.flat_map( move | workspace | workspace.packages_iterate( order ) )
  }
}

//

wtools::meta::mod_interface!
{
  prelude use packages_iterate;
  prelude use workspaces_packages_iterate;
}
