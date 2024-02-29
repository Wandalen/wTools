/// Internal namespace.
mod private
{
  use crate::*;

  use std::collections::{ HashSet, HashMap };
  use core::fmt::Formatter;
  use petgraph::prelude::*;

  use wtools::error::for_app::{ Error, anyhow };
  use path::AbsolutePath;
  use workspace::Workspace;
  use package::{ publish_need, Package };

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
        let list = endpoint::list
        (
          endpoint::list::ListArgs::former()
          .path_to_manifest( wanted.clone() )
          .format( endpoint::list::ListFormat::Tree )
          .dependency_sources([ endpoint::list::DependencySource::Local ])
          .dependency_categories([ endpoint::list::DependencyCategory::Primary ])
          .form()
        )
        .map_err( |( _, _e )| std::fmt::Error )?;
        let endpoint::list::ListReport::Tree( list ) = list else { unreachable!() };

        fn callback( name_bump_report: &HashMap< &String, ( &String, &String) >, mut r : endpoint::list::ListNodeReport ) -> endpoint::list::ListNodeReport
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

        let list = endpoint::list::ListReport::Tree( list );
        write!( f, "{}\n", list )?;
      }

      write!( f, "Actions:\n" )?;
      for ( path, report ) in &self.packages
      {
        let report = report.to_string().replace("\n", "\n  ");
        // qqq: remove unwrap
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

  pub fn publish( patterns : Vec< String >, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
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
      // FIX: patterns can point to different workspaces. Current solution take first random path from list
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

    let graph = metadata.graph();
    let subgraph_wanted = graph::subgraph( &graph, &packages_to_publish );
    let reversed_subgraph =
    {
      let roots = subgraph_wanted
      .node_indices()
      .map( | i | &graph[ subgraph_wanted[ i ] ] )
      .filter_map( | n | package_map.get( n )
      .map( | p | ( n, p ) ) )
      .map( |( n, p )| cargo::package( p.crate_dir(), false ).map( | _ | ( n, p ) ) )
      .collect::< Result< Vec< _ >, _ > >()
      .err_with( || report.clone() )?
      .into_iter()
      .filter( |( _, package )| publish_need( package ).unwrap() )
      .map( |( name, _ )| name.clone() )
      .collect::< Vec< _ > >();

      let mut reversed = graph.clone();
      reversed.reverse();
      graph::subgraph( &reversed, &roots )
    };
    {
      for node in reversed_subgraph.node_indices()
      {
        // `Incoming` - because of reversed
        if graph.neighbors_directed( reversed_subgraph[ node ], Incoming ).count() == 0
        {
          report.wanted_to_publish.push( package_map.get( &graph[ reversed_subgraph[ node ] ] ).unwrap().crate_dir() );
        }
      }
    }
    let subgraph = reversed_subgraph.map( | _, y | &graph[ *y ], | _, y | &graph[ *y ] );

    let queue = graph::toposort( subgraph ).unwrap().into_iter().map( | n | package_map.get( &n ).unwrap() ).rev().collect::< Vec< _ > >();

    for package in queue
    {
      let current_report = package::publish_single( package, true, dry )
      .map_err
      (
        | ( current_report, e ) |
        {
          report.packages.push(( package.crate_dir().absolute_path(), current_report.clone() ));
          ( report.clone(), e.context( "Publish list of packages" ).into() )
        }
      )?;
      report.packages.push(( package.crate_dir().absolute_path(), current_report ));
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
