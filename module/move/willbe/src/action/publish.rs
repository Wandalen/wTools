/// Internal namespace.
mod private
{
  use crate::*;

  use std::collections::{ HashSet, HashMap };
  use core::fmt::Formatter;
  use std::{ env, fs };

  use wtools::error::for_app::{ Error, anyhow };
  use _path::AbsolutePath;
  use workspace::Workspace;
  use package::Package;

  /// Represents a report of publishing packages
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    /// Represents the absolute path to the root directory of the workspace.
    pub workspace_root_dir : Option< AbsolutePath >,
    pub plan : Option< package::PublishPlan >,
    /// Represents a collection of packages and their associated publishing reports.
    pub packages : Vec<( AbsolutePath, package::PublishReport )>
  }

  impl std::fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.packages.is_empty()
      {
        write!( f, "Nothing to publish" )?;
        return Ok( () );
      }

      writeln!( f, "Actions :" )?;
      for ( path, report ) in &self.packages
      {
        let report = report.to_string().replace("\n", "\n  ");
        let path = if let Some( wrd ) = &self.workspace_root_dir
        {
          path.as_ref().strip_prefix( &wrd.as_ref() ).unwrap()
        }
        else
        {
          path.as_ref()
        };
        write!( f, "Publishing crate by `{}` path\n  {report}", path.display() )?;
      }
      if let Some( plan ) = &self.plan
      {
        if !plan.dry
        {
          let expected_to_publish = plan
          .plans
          .iter()
          .map( | p | ( p.version_bump.crate_dir.absolute_path(), p.package_name.clone(), p.version_bump.clone() ) )
          .collect::< Vec< _ > >();
          let mut actually_published = self.packages.iter()
          .filter_map
          (
            |( path, repo )|
            if repo.publish.as_ref().is_some_and( | r | r.error.is_ok() )
            {
              Some( path.clone() )
            }
            else
            {
              None
            }
          )
          .collect::< Vec< _ > >();

          writeln!( f, "Status :" )?;
          for ( path, name, version )  in expected_to_publish
          {
            if let Some( pos ) = actually_published.iter().position( | p | p == &path )
            {
              writeln!( f, "✅ {name} {}", version.new_version )?;
              // want to check that only expected packages actually published
              _ = actually_published.remove( pos );
            }
            else
            {
              writeln!( f, "❌ {name} {}", version.old_version )?;
            }
          }
          if !actually_published.is_empty()
          {
            writeln!( f, "Logical error. Published unexpected packages" )?;
            return Err( std::fmt::Error );
          }
        }
      }

      Ok( () )
    }
  }

  /// Publishes packages based on the specified patterns.
  ///
  /// # Arguments
  /// * `patterns` - A vector of patterns specifying the folders to search for packages.
  /// * `dry` - A boolean value indicating whether to perform a dry run.
  /// * `temp` - A boolean value indicating whether to use a temporary directory.
  ///
  /// # Returns
  /// A Result containing a `PublishPlan` if successful, or an `Error` otherwise.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish_plan( patterns : Vec< String >, dry : bool, temp : bool  ) -> Result< package::PublishPlan, Error >
  {
    let mut paths = HashSet::new();
    // find all packages by specified folders
    for pattern in &patterns
    {
      let current_path = AbsolutePath::try_from( std::path::PathBuf::from( pattern ) )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let mut metadata = if paths.is_empty()
    {
      Workspace::from_current_path()?
    }
    else
    {
      // FIX : patterns can point to different workspaces. Current solution take first random path from list
      let current_path = paths.iter().next().unwrap().clone();
      let dir = CrateDir::try_from( current_path )?;

      Workspace::with_crate_dir( dir )?
    };
    let workspace_root_dir : AbsolutePath = metadata
    .workspace_root()?
    .try_into()?;
    let packages = metadata.load()?.packages()?;
    let packages_to_publish : Vec< _ > = packages
    .iter()
    .filter( | &package | paths.contains( &AbsolutePath::try_from( package.manifest_path().as_std_path().parent().unwrap() ).unwrap() ) )
    .map( | p | p.name().clone() )
    .collect();
    let package_map = packages.into_iter().map( | p | ( p.name().clone(), Package::from( p.clone() ) ) ).collect::< HashMap< _, _ > >();

    let graph = metadata.graph();
    let subgraph_wanted = graph::subgraph( &graph, &packages_to_publish );
    let tmp = subgraph_wanted.map( | _, n | graph[ *n ].clone(), | _, e | graph[ *e ].clone() );

    let mut unique_name = format!( "temp_dir_for_publish_command_{}", path_tools::path::unique_folder_name()? );

    let dir = if temp
    {
      let mut temp_dir = env::temp_dir().join( unique_name );

      while temp_dir.exists()
      {
        unique_name = format!( "temp_dir_for_publish_command_{}", path_tools::path::unique_folder_name()? );
        temp_dir = env::temp_dir().join( unique_name );
      }

      fs::create_dir( &temp_dir )?;
      Some( temp_dir )
    }
    else
    {
      None
    };

    let subgraph = graph::remove_not_required_to_publish( &package_map, &tmp, &packages_to_publish, dir.clone() )?;
    let subgraph = subgraph.map( | _, n | n, | _, e | e );

    let queue = graph::toposort( subgraph ).unwrap().into_iter().map( | n | package_map.get( &n ).unwrap() ).cloned().collect::< Vec< _ > >();

    let roots = packages_to_publish.iter().map( | p | package_map.get( p ).unwrap().crate_dir() ).collect::< Vec< _ > >();

    let plan = package::PublishPlan::former()
    .workspace_dir( CrateDir::try_from( workspace_root_dir ).unwrap() )
    .option_base_temp_dir( dir.clone() )
    .dry( dry )
    .roots( roots )
    .packages( queue )
    .form();

    Ok( plan )
  }

  ///
  /// Publish packages.
  ///

  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish( plan : package::PublishPlan ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();
    let temp = plan.base_temp_dir.clone();

    report.plan = Some( plan.clone() );
    for package_report in package::perform_packages_publish( plan ).err_with( || report.clone() )?
    {
      let path : &std::path::Path = package_report.get_info.as_ref().unwrap().current_path.as_ref();
      report.packages.push(( AbsolutePath::try_from( path ).unwrap(), package_report ));
    }

    if let Some( dir ) = temp
    {
      fs::remove_dir_all( dir ).err_with( || report.clone() )?;
    }

    Ok( report )
  }


  trait ErrWith< T, T1, E >
  {
    fn err_with< F >( self, f : F ) -> std::result::Result< T1, ( T, E ) >
    where
      F : FnOnce() -> T;
  }

  impl< T, T1, E > ErrWith< T, T1, Error > for Result< T1, E >
  where
    E : std::fmt::Debug + std::fmt::Display + Send + Sync + 'static,
  {
    fn err_with< F >( self, f : F ) -> Result< T1, ( T, Error ) >
    where
      F : FnOnce() -> T,
    {
      self.map_err( | e | ( f(), anyhow!( e ) ) )
    }
  }
}

//

crate::mod_interface!
{
  /// Create a plan for publishing packages
  orphan use publish_plan;
  /// Execute the publication plan
  orphan use publish;
}
