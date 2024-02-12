/// Internal namespace.
mod private
{
  use crate::*;
  use std::
  {
    fmt::Formatter,
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
      ListFormat::Tree =>
      {
        let names = sorted
        .iter()
        .filter_map( | idx | if graph.node_weight( *idx ).unwrap() == &&root_crate { Some( *idx ) } else { None } )
        .collect::< Vec< _ > >();

        report = ListReport::Tree { graph : graph.map( | _, &n | String::from( n ), | _, &e | String::from( e ) ), names };
      }
      ListFormat::Topological if root_crate.is_empty() =>
      {
        let names = sorted
        .iter()
        .rev()
        .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
        .collect::< Vec< String > >();

        report = ListReport::List( names );
      },
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
  /// Wrapper to redirect output from `io::Write` to `fmt::Write`
  protected use Io2FmtWrite;
}
