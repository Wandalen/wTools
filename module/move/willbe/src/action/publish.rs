/// Internal namespace.
mod private
{
  use crate::*;

  use std::collections::{ HashSet, HashMap };
  use core::fmt::Formatter;
  use std::{ env, fs };

  use wtools::error::for_app::{ Error, anyhow };
  use path::AbsolutePath;
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
    /// Represents a collection of packages and their associated publishing reports.
    pub packages : Vec<( AbsolutePath, package::PublishReport )>
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
      write!( f, "Tree(-s):\n" )?;
      let name_bump_report = self
      .packages
      .iter()
      .filter_map( |( _, r )| r.bump.as_ref() )
      .map( | b | &b.base )
      .filter_map( | b | b.name.as_ref().and_then( | name  | b.old_version.as_ref().and_then( | old | b.new_version.as_ref().map( | new | ( name, ( old, new ) ) ) ) ) )
      .collect::< HashMap< _, _ > >();
      for wanted in &self.wanted_to_publish
      {
        let list = action::list
        (
          action::list::ListOptions::former()
          .path_to_manifest( wanted.clone() )
          .format( action::list::ListFormat::Tree )
          .dependency_sources([ action::list::DependencySource::Local ])
          .dependency_categories([ action::list::DependencyCategory::Primary ])
          .form()
        )
        .map_err( |( _, _e )| std::fmt::Error )?;
        let action::list::ListReport::Tree( list ) = list else { unreachable!() };

        fn callback( name_bump_report : &HashMap< &String, ( &String, &String) >, mut r : action::list::ListNodeReport ) -> action::list::ListNodeReport
        {
          if let Some(( old, new )) = name_bump_report.get( &r.name )
          {
            r.version = Some( format!( "({old} -> {new})" ) );
          }
          r.normal_dependencies = r.normal_dependencies.into_iter().map( | r | callback( name_bump_report, r ) ).collect();
          r.dev_dependencies = r.dev_dependencies.into_iter().map( | r | callback( name_bump_report, r ) ).collect();
          r.build_dependencies = r.build_dependencies.into_iter().map( | r | callback( name_bump_report, r ) ).collect();

          r
        }
        let list = list.into_iter().map( | r | callback( &name_bump_report, r ) ).collect();

        let list = action::list::ListReport::Tree( list );
        write!( f, "{}\n", list )?;
      }
      writeln!( f, "The following packages are pending for publication :" )?;
      for ( idx, package ) in self.packages.iter().map( |( _, p )| p ).enumerate()
      {
        if let Some( bump ) = &package.bump
        {
          match ( &bump.base.name, &bump.base.old_version, &bump.base.new_version )
          {
            ( Some( name ), Some( old ), Some( new ) ) => writeln!( f, "[{idx}] {name} ({old} -> {new})" )?,
            _ => {}
          }
        }
      }

      write!( f, "\nActions :\n" )?;
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
        f.write_fmt( format_args!( "Publishing crate by `{}` path\n  {report}\n", path.display() ) )?;
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
    report.workspace_root_dir = Some
    (
      metadata
      .workspace_root()
      .err_with( || report.clone() )?
      .try_into()
      .err_with( || report.clone() )?
    );
    let packages = metadata.load().err_with( || report.clone() )?.packages().err_with( || report.clone() )?;
    let packages_to_publish : Vec< _ > = packages
    .iter()
    .filter( | &package | paths.contains( &AbsolutePath::try_from( package.manifest_path.as_std_path().parent().unwrap() ).unwrap() ) )
    .map( | p | p.name.clone() )
    .collect();
    let package_map = packages.into_iter().map( | p | ( p.name.clone(), Package::from( p.clone() ) ) ).collect::< HashMap< _, _ > >();
    {
      for node in &packages_to_publish
      {
        report.wanted_to_publish.push( package_map.get( node ).unwrap().crate_dir() );
      }
    }

    let graph = metadata.graph();
    let subgraph_wanted = graph::subgraph( &graph, &packages_to_publish );
    let tmp = subgraph_wanted.map( | _, n | graph[ *n ].clone(), | _, e | graph[ *e ].clone() );

    let mut unique_name = format!( "temp_dir_for_publish_command_{}", path::unique_folder_name().err_with( || report.clone() )? );

    let dir = if temp
    {
      let mut temp_dir = env::temp_dir().join( unique_name );

      while temp_dir.exists()
      {
        unique_name = format!( "temp_dir_for_publish_command_{}", path::unique_folder_name().err_with( || report.clone() )? );
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

    let queue = graph::toposort( subgraph ).unwrap().into_iter().map( | n | package_map.get( &n ).unwrap() ).collect::< Vec< _ > >();

    for package in queue
    {
      let args = package::PublishSingleOptions::former()
      .package( package )
      .force( true )
      .option_base_temp_dir( &dir )
      .dry( dry )
      .form();
      let current_report = package::publish_single( args )
      .map_err
      (
        | ( current_report, e ) |
        {
          report.packages.push(( package.crate_dir().absolute_path(), current_report.clone() ));
          ( report.clone(), e.context( "Publish list of packages" ) )
        }
      )?;
      report.packages.push(( package.crate_dir().absolute_path(), current_report ));
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
