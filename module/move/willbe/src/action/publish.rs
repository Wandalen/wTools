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
    /// Represents a collection of packages that are roots of the trees.
    pub wanted_to_publish : Vec< CrateDir >,
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
      if let Some( plan ) = &self.plan
      {
        write!( f, "Tree{} :\n", if self.wanted_to_publish.len() > 1 { "s" } else { "" } )?;
        plan.display_as_tree( f, &self.wanted_to_publish )?;

        writeln!( f, "The following packages are pending for publication :" )?;
        plan.display_as_list( f )?;
      }

      writeln!( f, "\nActions :" )?;
      for ( path, report ) in &self.packages
      {
        let report = report.to_string().replace("\n", "\n  ");
        // qqq : remove unwrap
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

      Ok( () )
    }
  }

  ///
  /// Publish packages.
  ///

  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish( patterns : Vec< String >, dry : bool, temp : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut paths = HashSet::new();
    // find all packages by specified folders
    for pattern in &patterns
    {
      let current_path = AbsolutePath::try_from( std::path::PathBuf::from( pattern ) ).err_with( || report.clone() )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let mut metadata = if paths.is_empty()
    {
      Workspace::from_current_path().err_with( || report.clone() )?
    }
    else
    {
      // FIX : patterns can point to different workspaces. Current solution take first random path from list
      let current_path = paths.iter().next().unwrap().clone();
      let dir = CrateDir::try_from( current_path ).err_with( || report.clone() )?;

      Workspace::with_crate_dir( dir ).err_with( || report.clone() )?
    };
    let workspace_root_dir : AbsolutePath = metadata
    .workspace_root()
    .err_with( || report.clone() )?
    .try_into()
    .err_with( || report.clone() )?;
    report.workspace_root_dir = Some( workspace_root_dir.clone() );
    let packages = metadata.load().err_with( || report.clone() )?.packages().err_with( || report.clone() )?;
    let packages_to_publish : Vec< _ > = packages
    .iter()
    .filter( | &package | paths.contains( &AbsolutePath::try_from( package.manifest_path().as_std_path().parent().unwrap() ).unwrap() ) )
    .map( | p | p.name().clone() )
    .collect();
    let package_map = packages.into_iter().map( | p | ( p.name().clone(), Package::from( p.clone() ) ) ).collect::< HashMap< _, _ > >();
    {
      for node in &packages_to_publish
      {
        report.wanted_to_publish.push( package_map.get( node ).unwrap().crate_dir() );
      }
    }

    let graph = metadata.graph();
    let subgraph_wanted = graph::subgraph( &graph, &packages_to_publish );
    let tmp = subgraph_wanted.map( | _, n | graph[ *n ].clone(), | _, e | graph[ *e ].clone() );

    let mut unique_name = format!( "temp_dir_for_publish_command_{}", path_tools::path::unique_folder_name().err_with( || report.clone() )? );

    let dir = if temp
    {
      let mut temp_dir = env::temp_dir().join( unique_name );

      while temp_dir.exists()
      {
        unique_name = format!( "temp_dir_for_publish_command_{}", path_tools::path::unique_folder_name().err_with( || report.clone() )? );
        temp_dir = env::temp_dir().join( unique_name );
      }

      fs::create_dir( &temp_dir ).err_with( || report.clone() )?;
      Some( temp_dir )
    }
    else
    {
      None
    };

    let subgraph = graph::remove_not_required_to_publish( &package_map, &tmp, &packages_to_publish, dir.clone() );
    let subgraph = subgraph.map( | _, n | n, | _, e | e );

    let queue = graph::toposort( subgraph ).unwrap().into_iter().map( | n | package_map.get( &n ).unwrap() ).cloned().collect::< Vec< _ > >();

    let plan = package::PublishPlan::former()
    .workspace_dir( CrateDir::try_from( workspace_root_dir ).unwrap() )
    .option_base_temp_dir( dir.clone() )
    .dry( dry )
    .packages( queue )
    .form();
    report.plan = Some( plan.clone() );
    for package_report in package::perform_packages_publish( plan ).err_with( || report.clone() )?
    {
      let path : &std::path::Path = package_report.get_info.as_ref().unwrap().current_path.as_ref();
      report.packages.push(( AbsolutePath::try_from( path ).unwrap(), package_report ));
    }

    if temp
    {
      fs::remove_dir_all( dir.unwrap() ).err_with( || report.clone() )?;
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
  /// Publish package.
  orphan use publish;
}
