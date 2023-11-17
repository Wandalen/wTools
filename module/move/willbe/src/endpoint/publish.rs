/// Internal namespace.
mod private
{
  use crate::package::functions as package;

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
    for pattern in &patterns
    {
      let current_path = path::canonicalize( pattern ).map_err( | e | ( report.clone(), e.into() ) )?;
      let current_paths = files::find( current_path, &[ "**/Cargo.toml" ] );
      paths.extend( current_paths );
    }

    let paths = paths.iter().filter_map( | s | if s.ends_with( "Cargo.toml" ) { Some( s.into() ) } else { None } ).collect::< Vec< PathBuf > >();

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

    let packages_map = package::filter( &package_metadata );
    let sorted = package::toposort( &packages_map );

    for name in sorted.iter()
    {
      let path = packages_map[ name ].manifest_path.as_std_path();
      package::publish_single( path, dry )
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
