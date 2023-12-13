/// Internal namespace.
mod private
{
  use crate::*;
  use package::{ DependenciesOptions, DependenciesSort };
  use std::
  {
    collections::HashSet,
  };
  use core::fmt::Formatter;
  use workspace::Workspace;
  use package::{ CrateId, Package };
  use wtools::error::for_app::Error;
  use path::AbsolutePath;

  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    workspace_root_dir : Option<AbsolutePath>,
    packages : Vec<( AbsolutePath, package::PublishReport )>
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
        let report = report.to_string().replace("\n", "\n\t");
        // qqq: remove unwrap
        let path = if let Some( wrd ) = &self.workspace_root_dir
        {
          path.as_ref().strip_prefix( &wrd.as_ref() ).unwrap()
        }
        else
        {
          path.as_ref()
        };
        f.write_fmt( format_args!( "Publishing crate by `{}` path\n\t{report}\n", path.display() ) )?;
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
      let current_path = AbsolutePath::try_from( std::path::PathBuf::from( pattern ) ).map_err( | e | ( report.clone(), e.into() ) )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let mut metadata = if paths.is_empty()
    {
      Workspace::from_current_path().map_err( | e | ( report.clone(), e.into() ) )?
    }
    else
    {
      // FIX: patterns can point to different workspaces. Current solution take first random path from list
      let current_path = paths.iter().next().unwrap().clone();
      let dir = CrateDir::try_from( current_path ).map_err( | e | ( report.clone(), e.into() ) )?;

      Workspace::with_crate_dir( dir )
    };
    report.workspace_root_dir = Some( metadata.workspace_root().try_into().unwrap() );

    let packages_to_publish : Vec< _ >= metadata
    .load()
    .packages_get()
    .iter()
    .filter( | &package | paths.contains( &AbsolutePath::try_from( package.manifest_path.as_std_path().parent().unwrap() ).unwrap() ) )
    .cloned()
    .collect();
    let mut queue = vec![];
    for package in &packages_to_publish
    {
      let local_deps_args = DependenciesOptions
      {
        recursive: true,
        sort: DependenciesSort::Topological,
        ..Default::default()
      };
      let deps = package::dependencies( &mut metadata, &Package::from( package.clone() ), local_deps_args )
      .map_err( | e | ( report.clone(), e.into() ) )?;

      for dep in deps
      {
        if !queue.contains( &dep )
        {
          queue.push( dep );
        }
      }
      let crate_id = CrateId::from( package );
      if !queue.contains( &crate_id )
      {
        queue.push( crate_id );
      }
    }

    for path in queue.into_iter().filter_map( | id | id.path )
    {
      let current_report = package::publish_single( &Package::try_from( path.clone() ).unwrap(), dry )
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
}

//

crate::mod_interface!
{
  /// Publish package.
  orphan use publish;
}
