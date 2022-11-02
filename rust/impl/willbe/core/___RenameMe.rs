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

    // find all crates
    let workspaces = glob::glob( &format!( "{path}/**/Cargo.toml", path = path.display() ) ).unwrap();
    Box::new( workspaces_packages_iterate
    (
      workspaces
      // filter all valid paths
      .filter_map
      (
        | p |
        // map paths into Workspaces
        p.map( | mut p |
        {
          p.pop();
          Workspace::try_from( p )
        }).ok()
      )
      // filter all valid Workspaces
      .filter_map( Result::ok ),
      order
    ))
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
