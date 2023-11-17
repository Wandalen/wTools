/// Internal namespace.
mod private
{
  use crate::package::{ functions as package, local_dependencies, LocalDependenciesOptions };

  use crate::tools::
  {
    files,
    manifest,
    path,
  };
  use anyhow::Error;
  use std::
  {
    path::PathBuf,
    collections::HashSet,
  };
  use core::fmt::Formatter;
  use cargo_metadata::
  {
    MetadataCommand,
  };
  use wca::wtools::Itertools;
  use crate::package::functions::toposort_by_paths;
  use std::collections::HashMap;
  use crate::package::functions::FilterMapOptions;

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
        f.write_fmt( format_args!( "[ {} ]\n{report:#?}\n", path.display() ) )?;
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
      let current_paths = files::find( current_path, &[ "**/Cargo.toml" ] );
      paths.extend( current_paths );
    }

    // FIX: Maybe unsafe. Take metadata of workspace in current dir. Patterns can link to another.
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let paths : Vec< _ > = paths
    .into_iter()
    // with local dependencies
    .flat_map( | path | local_dependencies( &metadata, &path, LocalDependenciesOptions::default() ).unwrap().into_iter().chain( Some( path.parent().unwrap().to_path_buf() ) ) )
    // unique packages only
    .unique()
    .collect();

    // sort the list
    let paths = toposort_by_paths( &metadata, &paths );

    // publish sorted
    for path in paths
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

  ///
  /// Publish packages from workspace.
  ///

  pub fn workspace_publish( path_to_workspace : PathBuf, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut manifest = manifest::Manifest::new();
    let manifest_path = manifest.manifest_path_from_str( &path_to_workspace ).map_err( | e | ( report.clone(), e.into() ) )?;
    let package_metadata = MetadataCommand::new()
    .manifest_path( &manifest_path )
    .no_deps()
    .exec()
    .map_err( | e | ( report.clone(), e.into() ) )?;

    let packages_map = package::packages_filter_map
    (
      &package_metadata.packages,
      FilterMapOptions{ package_filter: Some( Box::new( | p |{ p.publish.is_none() } ) ), ..Default::default() }
    );
    let package_path_map: HashMap< _, _ > = package_metadata
    .packages
    .iter()
    .map( | p | ( &p.name, &p.manifest_path ) )
    .collect();

    let graph = package::graph_build( &packages_map );
    let sorted = package::toposort( graph );

    for name in &sorted
    {
      let path = package_path_map[ name ].clone().into();
      package::publish_single( &path, dry )
      .map_err
      (
        | ( current_report, e ) |
        {
          report.packages.push(( path.to_path_buf(), current_report.clone() ));
          ( report.clone(), e.context( "Publish list of packages" ).into() )
        }
      )?;
    }

    Ok( report )
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
