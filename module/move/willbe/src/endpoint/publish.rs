/// Internal namespace.
mod private
{
  use crate::*;

  use std::
  {
    collections::{ HashSet, HashMap }, io,
  };
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

      Workspace::with_crate_dir( dir ).map_err( | err | ( report.clone(), anyhow!( err ) ) )?
    };
    report.workspace_root_dir = Some
    ( 
      metadata
      .workspace_root()
      .map_err( | err | ( report.clone(), anyhow!( err ) ) )?
      .try_into()
      .map_err( | err: io::Error | ( report.clone(), anyhow!( err ) ) )?
    );
    let packages_to_publish : Vec< _ >= metadata
    .load()
    .map_err( | err | ( report.clone(), anyhow!( err ) ) )?
    .packages()
    .map_err( | err | ( report.clone(), anyhow!( err ) ) )?
    .iter()
    .filter( | &package | paths.contains( &AbsolutePath::try_from( package.manifest_path.as_std_path().parent().unwrap() ).unwrap() ) )
    .map( | p | p.name.clone() )
    .collect();
    let package_map = metadata.packages().unwrap().into_iter().map( | p | ( p.name.clone(), Package::from( p.clone() ) ) ).collect::< HashMap< _, _ > >();

    let graph = graph( &metadata );
    let subgraph_wanted = subgraph( &graph, &packages_to_publish );
    let reversed_subgraph =
    {
      let roots = subgraph_wanted.node_indices().map( | i | &graph[ subgraph_wanted[ i ] ] ).filter_map( | n | package_map.get( n ).map( | p | ( n, p ) ) ).inspect( |( _, p )| { cargo::package( p.crate_dir(), false ).unwrap(); } ).filter( |( _, package )| publish_need( package ).unwrap() ).map( |( name, _ )| name.clone() ).collect::< Vec< _ > >();

      let mut reversed = graph.clone();
      reversed.reverse();
      subgraph( &reversed, &roots )
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
    let subgraph = reversed_subgraph.map( | _, y | &graph[ *y ], | _, y | &graph[ subgraph_wanted[ *y ] ] );

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

  fn graph( workspace : &Workspace ) -> Graph< String, String >
  {
    let packages = workspace.packages().unwrap();
    let module_package_filter: Option< Box< dyn Fn( &cargo_metadata::Package ) -> bool > > = Some
    (
      Box::new( move | p | p.publish.is_none() )
    );
    let module_dependency_filter: Option< Box< dyn Fn( &cargo_metadata::Package, &cargo_metadata::Dependency) -> bool > > = Some
    (
      Box::new
      (
        move | _, d | d.path.is_some() && d.kind != cargo_metadata::DependencyKind::Development
      )
    );
    let module_packages_map = packages::filter
    (
      packages,
      packages::FilterMapOptions { package_filter: module_package_filter, dependency_filter: module_dependency_filter },
    );

    graph::construct( &module_packages_map ).map( | _, x | x.to_string(), | _, x | x.to_string() )
  }

  fn subgraph( graph : &Graph< String, String >, roots : &[ String ] ) -> Graph< NodeIndex, NodeIndex >
  {
    let mut subgraph = Graph::new();
    let mut node_map = HashMap::new();

    for root in roots
    {
      let root_id = graph.node_indices().find( | x | &graph[ *x ] == root ).unwrap();
      let mut dfs = Dfs::new( graph, root_id );
      while let Some( nx ) = dfs.next( &graph )
      {
        if !node_map.contains_key( &nx )
        {
          let sub_node = subgraph.add_node( nx );
          node_map.insert( nx, sub_node );
        }
      }
    }

    for ( _, sub_node_id ) in &node_map
    {
      let node_id_graph = subgraph[ *sub_node_id ];

      for edge in graph.edges( node_id_graph )
      {
        match ( node_map.get( &edge.source() ), node_map.get( &edge.target() ) )
        {
          ( Some( &from ), Some( &to ) ) =>
          {
            subgraph.add_edge( from, to, from );
          }
          _ => {}
        }
      }
    }

    subgraph
  }
}

//

crate::mod_interface!
{
  /// Publish package.
  orphan use publish;
}
