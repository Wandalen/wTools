/// Internal namespace.
mod private
{
  use crate::*;
  use std::
  {
    fmt::{ Formatter, Write },
    path::PathBuf,
    collections::HashSet,
  };
  use petgraph::
  {
    prelude::*,
    algo::{ toposort, has_path_connecting },
    visit::Topo,
  };
  use std::str::FromStr;
  use packages::FilterMapOptions;
  use wtools::error::
  {
    for_app::{ Error, Context, format_err },
    err
  };
  use cargo_metadata::
  {
    Dependency,
    DependencyKind,
    Package
  };
  use petgraph::prelude::{ Dfs, EdgeRef };
  use former::Former;

  use workspace::Workspace;
  use path::AbsolutePath;

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFormat
  {
    /// Tree like format.
    #[ default ]
    Tree,
    /// Topologically sorted list.
    Topological,
  }

  impl FromStr for ListFormat
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "tree" => ListFormat::Tree,
        "toposort" => ListFormat::Topological,
        e => return Err( err!( "Unknown format '{}'. Available values: [tree, toposort]", e ))
      };

      Ok( value )
    }
  }

  /// Enum representing the different dependency categories.
  ///
  /// These categories include:
  /// - `Primary`: This category represents primary dependencies.
  /// - `Dev`: This category represents development dependencies.
  /// - `Build`: This category represents build-time dependencies.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum DependencyCategory
  {
    /// Represents the primary dependencies, i.e., libraries or packages that
    /// are required for your code to run. These are typically listed in your
    /// `Cargo.toml`'s `[dependencies]` section.
    Primary,
    /// Represents the development dependencies. These are used for compiling
    /// tests, examples, or benchmarking code. They are not used when compiling
    /// the normal application or library. These are typically listed in your
    /// `Cargo.toml`'s `[dev-dependencies]` section.
    Dev,
    /// Represents build-time dependencies. These are used only to compile
    /// build scripts (`build.rs`) but not for the package code itself. These
    /// are typically listed in your `Cargo.toml`'s `[build-dependencies]` section.
    Build,
  }

  /// Enum representing the source of a dependency.
  ///
  /// This enum has the following values:
  /// * `Local` - Represents a dependency located locally.
  /// * `Remote` - Represents a dependency fetched from a remote source.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum DependencySource
  {
    /// Represents a dependency that is located on the local file system.
    Local,
    /// Represents a dependency that is to be fetched from a remote source.
    Remote,
  }

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFilter
  {
    /// With all packages.
    #[ default ]
    Nothing,
    /// With local only packages.
    Local,
  }

  impl FromStr for ListFilter
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "nothing" => ListFilter::Nothing,
        "local" => ListFilter::Local,
        e => return Err( err!( "Unknown filter '{}'. Available values: [nothing, local]", e ) )
      };

      Ok( value )
    }
  }

  /// Output of the `list` endpoint
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// With tree format.
    Tree
    {
      /// Dependencies graph.
      graph : Graph< String, String >,
      /// Packages indexes to display.
      names : Vec< petgraph::stable_graph::NodeIndex >,
    },
    /// With topologically sorted list.
    List( Vec< String > ),
    /// Nothing to show.
    #[ default ]
    Empty
  }

  /// Wrapper to redirect output from `ptree` graph to `fmt::Write`
  pub struct Io2FmtWrite< 'a, W >
  {
    /// This struct provides a mutable reference to a writer and is used as a formatting writer.
    pub f : &'a mut W,
  }

  impl< T > std::fmt::Debug for Io2FmtWrite< '_, T >
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( std::any::type_name::< Self >() ).finish()
    }
  }

  impl< W : std::fmt::Write > std::io::Write for Io2FmtWrite< '_, W >
  {
    fn write( &mut self, buf : &[ u8 ] ) -> std::io::Result< usize >
    {
      use std::io::ErrorKind;

      let size = buf.len();

      self.f.write_str
      (
        std::str::from_utf8( buf )
        .map_err( | _ | std::io::Error::new( ErrorKind::InvalidData, "Allow only valid UTF-8 string" ) )?
      )
      .map_err( | e | std::io::Error::new( ErrorKind::Other, e ) )?;

      Ok( size )
    }

    fn flush( &mut self ) -> std::io::Result< () >
    {
      Ok( () )
    }
  }

  impl std::fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        ListReport::Tree { graph, names } => for n in names
        {
          ptree::graph::write_graph_with( &graph, *n, Io2FmtWrite { f }, &ptree::PrintConfig::from_env() ).unwrap();
        },
        ListReport::List ( list ) => for ( i ,e ) in list.iter().enumerate() { writeln!( f, "{i}) {e}" )? },
        _ => {},
      }

      Ok( () )
    }
  }

  /// A struct representing the arguments for listing crates.
  ///
  /// This struct is used to pass the necessary arguments for listing crates. It includes the
  /// following fields:
  ///
  /// - `path_to_manifest`: A `CrateDir` representing the path to the manifest of the crates.
  /// - `format`: A `ListFormat` enum representing the desired format of the output.
  /// - `dependency_sources`: A `HashSet` of `DependencySource` representing the sources of the dependencies.
  #[ derive( Debug, Former ) ]
  pub struct ListArgs
  {
    path_to_manifest : CrateDir,
    format : ListFormat,
    dependency_sources: HashSet< DependencySource >,
    dependency_categories: HashSet< DependencyCategory >,
  }

  #[ derive( Debug, Clone ) ]
  pub struct ListNodeReport
  {
    name: String,
    version: Option< String >,
    path: Option< PathBuf >,
    normal_dependencies: Vec< ListNodeReport >,
    dev_dependencies: Vec< ListNodeReport >,
    build_dependencies: Vec< ListNodeReport >,
  }

  impl ListNodeReport
  {
    fn display_with_spacer( &self, spacer : &str, depth : usize ) -> Result< String, std::fmt::Error >
    {
      let mut f = String::new();

      write!( f, "{spacer}{}", self.name )?;
      if let Some( version ) = &self.version { write!( f, " {version}" )? }
      if let Some( path ) = &self.path { write!( f, " {}", path.display() )? }
      write!( f, "\n" )?;

      let spacer = format!( "{spacer}[{depth}] " );
      let depth = depth + 1;

      for dep in &self.normal_dependencies
      {
        write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
      }
      if !self.dev_dependencies.is_empty()
      {
        write!( f, "{spacer}[dev-dependencies]\n" )?;
        let spacer = format!( "{spacer}| " );
        for dep in &self.dev_dependencies
        {
          write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
        }
      }
      if !self.build_dependencies.is_empty()
      {
        write!( f, "{spacer}[build-dependencies]\n" )?;
        let spacer = format!( "{spacer}| " );
        for dep in &self.build_dependencies
        {
          write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
        }
      }

      Ok( f )
    }
  }

  impl std::fmt::Display for ListNodeReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.display_with_spacer( "", 0 )? )?;

      Ok( () )
    }
  }

  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReportV2
  {
    Tree( Vec< ListNodeReport > ),
    #[ default ]
    Empty,
  }

  impl std::fmt::Display for ListReportV2
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        Self::Tree( v ) => write!( f, "{}", v.iter().map( | l | l.to_string() ).collect::< Vec< _ > >().join( "\n" ) ),
        Self::Empty => write!( f, "" ),
      }
    }
  }

  fn process_package_dependency
  (
    workspace : &Workspace,
    package : &Package,
    args : &ListArgs,
    dep_rep : &mut ListNodeReport,
    visited : &mut HashSet< String >
  )
  {
    for dependency in &package.dependencies
    {
      if dependency.path.is_some() && !args.dependency_sources.contains( &DependencySource::Local ) { continue; }
      if dependency.path.is_none() && !args.dependency_sources.contains( &DependencySource::Remote ) { continue; }
      let dep_id = format!( "{}+{}+{}", dependency.name, dependency.req, dependency.path.as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );

      let mut temp_vis = visited.clone();
      let dependency_rep = process_dependency( workspace, dependency, args, &mut temp_vis );

      match dependency.kind
      {
        DependencyKind::Normal if args.dependency_categories.contains( &DependencyCategory::Primary ) => dep_rep.normal_dependencies.push( dependency_rep ),
        DependencyKind::Development if args.dependency_categories.contains( &DependencyCategory::Dev ) => dep_rep.dev_dependencies.push( dependency_rep ),
        DependencyKind::Build if args.dependency_categories.contains( &DependencyCategory::Build ) => dep_rep.build_dependencies.push( dependency_rep ),
        _ => { visited.remove( &dep_id ); std::mem::swap( &mut temp_vis, visited ); }
      }

      *visited = std::mem::take( &mut temp_vis );
    }
  }

  fn process_dependency( workspace : &Workspace, dep: &Dependency, args : &ListArgs, visited : &mut HashSet< String > ) -> ListNodeReport
  {
    let mut dep_rep = ListNodeReport
    {
      name : dep.name.clone(),
      version : Some( dep.req.to_string() ),
      path : dep.path.as_ref().map( | p | p.clone().into_std_path_buf() ),
      normal_dependencies : vec![],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    };

    let dep_id = format!( "{}+{}+{}", dep.name, dep.req, dep.path.as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );
    // if this is a cycle (we have visited this node before)
    if visited.contains( &dep_id )
    {
      dep_rep.name = format!( "{} (*)", dep_rep.name );

      return dep_rep;
    }

    // if we have not visited this node before, mark it as visited
    visited.insert( dep_id );
    if let Some( path ) = &dep.path
    {
      if let Some( package ) = workspace.package_find_by_manifest( path.as_std_path().join( "Cargo.toml" ) )
      {
        process_package_dependency( workspace, package, args, &mut dep_rep, visited );
      }
    }

    dep_rep
  }

  trait ErrWith< T, T1, E >
  {
    fn err_with( self, v : T ) -> std::result::Result< T1, ( T, E ) >;
  }

  impl< T, T1, E > ErrWith< T, T1, E > for Result< T1, E >
  {
    fn err_with( self, v : T ) -> Result< T1, ( T, E ) >
    {
      self.map_err( | e | ( v, e ) )
    }
  }

  /// -
  pub fn listv2( args : ListArgs ) -> Result< ListReportV2, ( ListReportV2, Error ) >
  {
    let mut report = ListReportV2::default();

    let manifest = manifest::open( args.path_to_manifest.absolute_path() ).context( "List of packages by specified manifest path" ).err_with( report.clone() )?;
    let metadata = Workspace::with_crate_dir( manifest.crate_dir() ).err_with( report.clone() )?;

    let is_package = manifest.package_is().context( "try to identify manifest type" ).err_with( report.clone() )?;

    let tree_package_report = | path : AbsolutePath, report : &mut ListReportV2, visited : &mut HashSet< String > |
    {
      let package = metadata.package_find_by_manifest( path ).unwrap();
      let mut package_report = ListNodeReport
      {
        name: package.name.clone(),
        version: Some( package.version.to_string() ),
        path: Some( package.manifest_path.clone().into_std_path_buf() ),
        normal_dependencies: vec![],
        dev_dependencies: vec![],
        build_dependencies: vec![],
      };

      process_package_dependency( &metadata, package, &args, &mut package_report, visited );

      *report = match report
      {
        ListReportV2::Tree( ref mut v ) => ListReportV2::Tree( { v.extend([ package_report ]); v.clone() } ),
        ListReportV2::Empty => ListReportV2::Tree( vec![ package_report ] ),
      };
    };
    match args.format
    {
      ListFormat::Tree if is_package =>
      {
        let mut visited = HashSet::new();
        tree_package_report( manifest.manifest_path, &mut report, &mut visited )
      }
      ListFormat::Tree =>
      {
        let packages = metadata.packages_get().context( "workspace packages" ).err_with( report.clone() )?;
        let mut visited = packages.iter().map( | p | format!( "{}+{}+{}", p.name, p.version.to_string(), p.manifest_path ) ).collect();
        for package in packages
        {
          tree_package_report( package.manifest_path.as_path().try_into().unwrap(), &mut report, &mut visited )
        }
      }
      _ => { unimplemented!() }
    }

    Ok( report )
  }

  ///
  /// List workspace packages.
  ///

  pub fn list( args : ListArgs ) -> Result< ListReport, ( ListReport, Error ) >
  {
    let mut report = ListReport::default();

    let manifest = manifest::open( args.path_to_manifest.absolute_path() ).context( "List of packages by specified manifest path" ).map_err( | e | ( report.clone(), e.into() ) )?;
    let mut metadata = Workspace::with_crate_dir( manifest.crate_dir() ).map_err( | err | ( report.clone(), format_err!( err ) ) )?;

    let root_crate = manifest
    .manifest_data
    .as_ref()
    .and_then( | m | m.get( "package" ) )
    .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
    .unwrap_or_default();

    // let packages_map =  metadata.packages.iter().map( | p | ( p.name.clone(), p ) ).collect::< HashMap< _, _ > >();

    let dep_filter = move | _p: &Package, d: &Dependency |
    {
      (
        args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind == DependencyKind::Normal
        || args.dependency_categories.contains( &DependencyCategory::Dev ) && d.kind == DependencyKind::Development
        || args.dependency_categories.contains( &DependencyCategory::Build ) && d.kind == DependencyKind::Build
      )
      &&
      (
        args.dependency_sources.contains( &DependencySource::Remote ) && d.path.is_none()
        || args.dependency_sources.contains( &DependencySource::Local ) && d.path.is_some()
      )
    };

    let packages_map =  packages::filter
    (
      &metadata
      .load()
      .map_err( | err | ( report.clone(), format_err!( err ) ) )?
      .packages_get()
      .map_err( | err | ( report.clone(), format_err!( err ) ) )?,
      FilterMapOptions{ dependency_filter: Some( Box::new( dep_filter ) ), ..Default::default() }
    );

    let graph = graph::construct( &packages_map );

    let sorted = toposort( &graph, None ).map_err( | e | { use std::ops::Index; ( report.clone(), err!( "Failed to process toposort for package: {:?}", graph.index( e.node_id() ) ) ) } )?;

    match args.format
    {
      // all crates
      ListFormat::Tree if root_crate.is_empty() =>
      {
        let mut names = vec![ sorted[ 0 ] ];
        for node in sorted.iter().skip( 1 )
        {
          if names.iter().all( | name | !has_path_connecting( &graph, *name, *node, None ) ) && !names.contains( node )
          {
            names.push( *node );
          }
        }
        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      },
      // specific crate as root
      ListFormat::Tree =>
      {
        let names = sorted
        .iter()
        .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &&root_crate { Some( *idx ) } else { None } )
        .collect::< Vec< _ > >();

        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      }
      // all crates
      ListFormat::Topological if root_crate.is_empty() =>
      {
        let names = sorted
        .iter()
        .rev()
        .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
        .collect::< Vec< String > >();

        report = ListReport::List( names );
      },
      // specific crate as root
      ListFormat::Topological =>
      {
        let node = graph.node_indices().find( | n | graph.node_weight( *n ).unwrap() == &&root_crate ).unwrap();
        let mut dfs = Dfs::new( &graph, node );
        let mut subgraph = Graph::new();
        let mut node_map = std::collections::HashMap::new();
        while let Some( n )= dfs.next( &graph )
        {
          node_map.insert( n, subgraph.add_node( graph[ n ] ) );
        }

        for e in graph.edge_references()
        {
          if let ( Some( &s ), Some( &t ) ) = ( node_map.get( &e.source() ), node_map.get( &e.target() ) )
          {
            subgraph.add_edge( s, t, () );
          }
        }

        let mut topo = Topo::new( &subgraph );
        let mut names = Vec::new();
        while let Some( n ) = topo.next( &subgraph )
        {
          names.push( subgraph[ n ].clone() );
        }
        names.reverse();

        report = ListReport::List( names );
      }
    }

    Ok( report )
  }
}

//

crate::mod_interface!
{
  /// Arguments for `list` endpoint.
  protected use ListArgs;
  /// Represents where a dependency located.
  protected use DependencySource;
  /// Represents the category of a dependency.
  protected use DependencyCategory;
  /// Argument for `list` endpoint. Sets the output format.
  protected use ListFormat;
  /// Argument for `list` endpoint. Sets filter(local or all) packages should be in the output.
  protected use ListFilter;
  /// Contains output of the endpoint.
  protected use ListReport;
  /// List packages in workspace.
  orphan use list;
  orphan use listv2;
  /// Wrapper to redirect output from `io::Write` to `fmt::Write`
  protected use Io2FmtWrite;
}
