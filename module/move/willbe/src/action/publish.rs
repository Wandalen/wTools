/// Define a private namespace for all its items.
mod private
{
  use crate::*;

  use std::{ env, fmt, fs };
  use
  {
    // error::untyped,
    error::ErrWith,
  };

  /// Represents a report of publishing packages
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    /// Represents the absolute path to the root directory of the workspace.
    pub workspace_root_dir : Option< AbsolutePath >,
    pub plan : Option< publish::PublishPlan >,
    /// Represents a collection of packages and their associated publishing reports.
    pub packages : Vec<( AbsolutePath, publish::PublishReport )>
  }

  impl fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
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
          let expected_to_publish : Vec< _ > = plan
          .plans
          .iter()
          .map
          (
            | p |
            (
              p.bump.crate_dir.clone().absolute_path(),
              p.package_name.clone(),
              p.bump.clone()
            )
          )
          .collect();
          let mut actually_published : Vec< _ > = self.packages.iter()
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
          .collect();

          writeln!( f, "Status :" )?;
          for ( path, name, version ) in expected_to_publish
          {
            if let Some( pos ) = actually_published
            .iter()
            .position( | p | p == &path )
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
  /// * `exclude_dev_dependencies` - A boolean value indicating whether to exclude dev dependencies from manifest before publish.
  /// * `dry` - A boolean value indicating whether to perform a dry run.
  /// * `temp` - A boolean value indicating whether to use a temporary directory.
  ///
  /// # Returns
  /// A Result containing a `PublishPlan` if successful, or an `Error` otherwise.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish_plan
  (
    patterns : Vec< String >,
    channel : channel::Channel,
    exclude_dev_dependencies : bool,
    commit_changes : bool,
    dry : bool,
    temp : bool
  )
  -> Result< publish::PublishPlan, error::untyped::Error >
  // qqq : use typed error
  {
    let mut paths = collection::HashSet::new();
    // find all packages by specified folders
    for pattern in &patterns
    {
      let current_path = AbsolutePath::try_from
      (
        fs::canonicalize( pattern.as_str() )?
      )?;
      // let current_path = AbsolutePath::try_from( std::path::PathBuf::from( pattern ) )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let workspace = if paths.is_empty()
    {
      Workspace::try_from( CurrentPath )?
    }
    else
    {
      // qqq : patterns can point to different workspaces. Current solution take first random path from list.
      // A problem may arise if a user provides paths to packages from different workspaces
      // and we do not check whether all packages are within the same workspace
      // In the current solution, we'll choose the workspace related to the first package
      let current_path = paths.iter().next().unwrap().clone();
      let dir = CrateDir::try_from( current_path )?;
      Workspace::try_from( dir )?
    };

    let workspace_root_dir : AbsolutePath = workspace
    .workspace_root()
    .try_into()?;

    let packages = workspace.packages();
    let packages_to_publish : Vec< String > = packages
    .clone()
    .filter( | &package | paths.contains( &package.crate_dir().unwrap().into() ) )
    .map( | p | p.name().to_string() )
    .collect();
    let package_map : collection::HashMap< String, package::Package< '_ > > = packages
    .map( | p | ( p.name().to_string(), package::Package::from( p ) ) )
    .collect();

    let graph = workspace_graph::graph( &workspace );
    let subgraph_wanted = graph::subgraph
    (
      &graph,
      &packages_to_publish[ .. ]
    );
    let tmp = subgraph_wanted
    .map
    (
      | _, n |
      graph[ *n ].clone(), | _, e | graph[ *e ].clone()
    );

    let mut unique_name = format!
    (
      "temp_dir_for_publish_command_{}",
      path::unique_folder_name()?
    );

    let dir = if temp
    {
      let mut temp_dir = env::temp_dir().join( unique_name );

      while temp_dir.exists()
      {
        unique_name = format!
        (
          "temp_dir_for_publish_command_{}",
          path::unique_folder_name()?
        );
        temp_dir = env::temp_dir().join( unique_name );
      }

      fs::create_dir( &temp_dir )?;
      Some( temp_dir )
    }
    else
    {
      None
    };

    let subgraph = graph::remove_not_required_to_publish
    (
      &package_map,
      &tmp,
      &packages_to_publish,
      dir.clone()
    )?;
    let subgraph = subgraph
    .map( | _, n | n, | _, e | e );

    let queue : Vec< _ > = graph::toposort( subgraph )
    .unwrap()
    .into_iter()
    .map( | n | package_map.get( &n ).unwrap() )
    .cloned()
    .collect();

    let roots : Vec< _ > = packages_to_publish
    .iter()
    .map( | p | package_map.get( p ).unwrap().crate_dir() ).collect();

    let plan = publish::PublishPlan::former()
    .channel( channel )
    .workspace_dir( CrateDir::try_from( workspace_root_dir ).unwrap() )
    .option_base_temp_dir( dir.clone() )
    .exclude_dev_dependencies( exclude_dev_dependencies )
    .commit_changes( commit_changes )
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
  pub fn publish( plan : publish::PublishPlan )
  ->
  ResultWithReport< PublishReport, error::untyped::Error >
  // qqq : use typed error
  {
    let mut report = PublishReport::default();
    let temp = plan.base_temp_dir.clone();

    report.plan = Some( plan.clone() );
    for package_report in publish::perform_packages_publish( plan ).err_with_report( &report )?
    {
      let path : &std::path::Path = package_report.get_info.as_ref().unwrap().current_path.as_ref();
      report.packages.push(( AbsolutePath::try_from( path ).unwrap(), package_report ));
    }

    if let Some( dir ) = temp
    {
      fs::remove_dir_all( dir ).err_with_report( &report )?;
    }

    Ok( report )
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
