/// Internal namespace.
mod private
{
  use crate::*;
  use package::{ DependenciesOptions, DependenciesSort };
  use tools::path;
  use std::
  {
    path::PathBuf,
    collections::HashSet,
  };
  use core::fmt::Formatter;
  use cache::WorkspaceCache;
  use package::CrateId;
  use wtools::error::for_app::Error;

  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    packages : Vec<( PathBuf,  package::PublishReport )>
  }

  impl std::fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.packages.is_empty()
      {
        f.write_fmt( format_args!( "Nothing to publish" ) )?;
        return Ok( () );
      }

      for ( path, report ) in &self.packages
      {
        f.write_fmt( format_args!( "[ {} ]\n{report}\n", path.display() ) )?;
      }

      Ok( () )
    }
  }

  ///
  /// Publish packages.
  ///

  pub fn publish( patterns : Vec< String >, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut paths = HashSet::new();
    // find all packages by specified folders
    for pattern in &patterns
    {
      let current_path = path::canonicalize( pattern ).map_err( | e | ( report.clone(), e.into() ) )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let mut metadata = if paths.is_empty()
    {
      WorkspaceCache::default()
    }
    else
    {
      // FIX: patterns can point to different workspaces. Current solution take first random path from list
      WorkspaceCache::with_manifest_path( paths.iter().next().unwrap() )
    };

    let packages_to_publish : Vec< _ >= metadata.load().packages_get().iter().filter( | &package | paths.contains( package.manifest_path.as_std_path().parent().unwrap() ) ).cloned().collect();
    let mut queue = vec![];
    for package in &packages_to_publish
    {
      // get sorted dependencies
      let local_deps_args = DependenciesOptions
      {
        recursive: true,
        sort: DependenciesSort::Topological,
        ..Default::default()
      };
      let deps = package::dependencies( &mut metadata, package.manifest_path.as_std_path(), local_deps_args )
      .map_err( | e | ( report.clone(), e.into() ) )?;

      // add dependencies to publish queue
      for dep in deps
      {
        if !queue.contains( &dep )
        {
          queue.push( dep );
        }
      }
      // add current package to publish queue if it isn't already here
      let crate_id = CrateId::from( package );
      if !queue.contains( &crate_id )
      {
        queue.push( crate_id );
      }
    }

    // process publish
    for path in queue.into_iter().filter_map( | id | id.path )
    {
      let current_report = package::publish_single( &path, dry )
      .map_err
      (
        | ( current_report, e ) |
        {
          report.packages.push(( path.clone(), current_report.clone() ));
          ( report.clone(), e.context( "Publish list of packages" ).into() )
        }
      )?;
      report.packages.push(( path, current_report ));
    }

    Ok( report )
  }

//   ///
//   /// Publish packages from workspace.
//   ///
//
//   pub fn workspace_publish( path_to_workspace : PathBuf, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
//   {
//     let mut report = PublishReport::default();
//
//     let mut package_metadata = WorkspaceCache::with_manifest_path( path_to_workspace );
//
//     let packages_map = package::packages_filter_map
//     (
//       &package_metadata.load().packages_get(),
//       FilterMapOptions{ package_filter: Some( Box::new( | p |{ p.publish.is_none() } ) ), ..Default::default() }
//     );
//     let package_path_map: HashMap< _, _ > = package_metadata
//     .load()
//     .packages_get()
//     .iter()
//     .map( | p | ( &p.name, &p.manifest_path ) )
//     .collect();
//
//     let graph = package::graph_build( &packages_map );
//     let sorted = package::toposort( graph );
//
//     for name in &sorted
//     {
//       let path = package_path_map[ name ].as_std_path();
//       package::publish_single( &path, dry )
//       .map_err
//       (
//         | ( current_report, e ) |
//         {
//           report.packages.push(( path.to_path_buf(), current_report.clone() ));
//           ( report.clone(), e.context( "Publish list of packages" ).into() )
//         }
//       )?;
//     }
//
//     Ok( report )
//   }

}

//

crate::mod_interface!
{
  /// Publish package.
  orphan use publish;
  // /// Publish packages from workspace.
  // orphan use workspace_publish;
}
