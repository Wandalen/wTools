pub( crate ) mod private
{
  use crate::*;

  /// Returns a graph of packages.
  pub fn graph( workspace : &Workspace ) -> petgraph::Graph< String, String >
  {
    let packages = workspace.packages();
    let module_package_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ > ) -> bool > > = Some
    (
      Box::new( move | p | p.publish().is_none() )
    );
    let module_dependency_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ >, DependencyRef< '_ > ) -> bool > > = Some
    (
      Box::new
      (
        move | _, d | d.crate_dir().is_some() && d.kind() != DependencyKind::Development
      )
    );
    let module_packages_map = packages::filter
    (
      // packages.as_slice(),
      packages,
      packages::FilterMapOptions { package_filter : module_package_filter, dependency_filter : module_dependency_filter },
    );

    graph::construct( &module_packages_map ).map( | _, x | x.to_string(), | _, x | x.to_string() )
  }
}

//

crate::mod_interface!
{
  protected use graph;
}

// xxx : move