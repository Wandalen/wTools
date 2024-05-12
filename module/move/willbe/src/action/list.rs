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
  use std::collections::HashMap;
  use petgraph::
  {
    prelude::*,
    algo::toposort,
    visit::Topo,
  };
  use std::str::FromStr;
  use packages::FilterMapOptions;
  use wtools::error::
  {
    for_app::{ Error, Context },
    err
  };
  // aaa : for Petro : don't use cargo_metadata and Package directly, use facade
  // aaa : ✅

  use petgraph::prelude::{ Dfs, EdgeRef };
  use former::Former;

  use workspace::Workspace;
  use _path::AbsolutePath;
  use workspace::WorkspacePackage;

  /// Args for `list` action.
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
        e => return Err( err!( "Unknown format '{}'. Available values : [tree, toposort]", e ))
      };

      Ok( value )
    }
  }

  /// Enum representing the different dependency categories.
  ///
  /// These categories include :
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
  /// This enum has the following values :
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

  /// Args for `list` action.
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
        e => return Err( err!( "Unknown filter '{}'. Available values : [nothing, local]", e ) )
      };

      Ok( value )
    }
  }

  /// Additional information to include in a package report.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum PackageAdditionalInfo
  {
    /// Include the version of the package, if possible.
    Version,
    /// Include the path to the package, if it exists.
    Path,
  }

  /// A struct representing the arguments for listing crates.
  ///
  /// This struct is used to pass the necessary arguments for listing crates. It includes the
  /// following fields :
  ///
  /// - `path_to_manifest`: A `CrateDir` representing the path to the manifest of the crates.
  /// - `format`: A `ListFormat` enum representing the desired format of the output.
  /// - `dependency_sources`: A `HashSet` of `DependencySource` representing the sources of the dependencies.
  #[ derive( Debug, Former ) ]
  pub struct ListOptions
  {
    path_to_manifest : CrateDir,
    format : ListFormat,
    info : HashSet< PackageAdditionalInfo >,
    dependency_sources : HashSet< DependencySource >,
    dependency_categories : HashSet< DependencyCategory >,
  }

  struct Symbols
  {
    down : &'static str,
    tee : &'static str,
    ell : &'static str,
    right : &'static str,
  }

  // qqq : fro Bohdan : abstract and move out tree printing. or reuse ready solution for tree printing
  // stick to single responsibility
  const UTF8_SYMBOLS : Symbols = Symbols
  {
    down : "│",
    tee  : "├",
    ell  : "└",
    right : "─",
  };

  /// Represents a node in a dependency graph.
  /// It holds essential information about the project dependencies. It is also capable
  /// of holding any nested dependencies in a recursive manner, allowing the modeling
  /// of complex dependency structures.
  #[ derive( Debug, Clone, Eq, PartialEq ) ]
  pub struct ListNodeReport
  {
    /// This could be the name of the library or crate.
    pub name : String,
    /// Ihe version of the crate.
    pub version : Option< String >,
    /// The path to the node's source files in the local filesystem. This is
    /// optional as not all nodes may have a local presence (e.g., nodes representing remote crates).
    pub path : Option< PathBuf >,
    /// This field is a flag indicating whether the Node is a duplicate or not.
    pub duplicate : bool,
    /// A list that stores normal dependencies.
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub normal_dependencies : Vec< ListNodeReport >,
    /// A list that stores dev dependencies(dependencies required for tests or examples).
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub dev_dependencies : Vec< ListNodeReport >,
    /// A list that stores build dependencies.
    /// Each element in the list is also of the same 'ListNodeReport' type to allow
    /// storage of nested dependencies.
    pub build_dependencies : Vec< ListNodeReport >,
  }

  impl ListNodeReport
  {
    /// Displays the name, version, path, and dependencies of a package with appropriate indentation and spacing.
    ///
    /// # Arguments
    ///
    /// * `spacer` - A string used for indentation.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the formatted string or a `std::fmt::Error` if formatting fails.
    pub fn display_with_spacer( &self, spacer : &str ) -> Result< String, std::fmt::Error >
    {
      let mut f = String::new();

      write!( f, "{}", self.name )?;
      if let Some( version ) = &self.version { write!( f, " {version}" )? }
      if let Some( path ) = &self.path { write!( f, " {}", path.display() )? }
      if self.duplicate { write!( f, "(*)" )? }
      write!( f, "\n" )?;

      let mut new_spacer = format!( "{spacer}{}  ", if self.normal_dependencies.len() < 2 { " " } else { UTF8_SYMBOLS.down } );

      let mut normal_dependencies_iter = self.normal_dependencies.iter();
      let last = normal_dependencies_iter.next_back();

      for dep in normal_dependencies_iter
      {
        write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
      }
      if let Some( last ) = last
      {
        new_spacer = format!( "{spacer}   " );
        write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.display_with_spacer( &new_spacer )? )?;
      }
      if !self.dev_dependencies.is_empty()
      {
        let mut dev_dependencies_iter = self.dev_dependencies.iter();
        let last = dev_dependencies_iter.next_back();
        write!( f, "{spacer}[dev-dependencies]\n" )?;
        for dep in dev_dependencies_iter
        {
          write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
        }
        // unwrap - safe because `is_empty` check
        write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
      }
      if !self.build_dependencies.is_empty()
      {
        let mut build_dependencies_iter = self.build_dependencies.iter();
        let last = build_dependencies_iter.next_back();
        write!( f, "{spacer}[build-dependencies]\n" )?;
        for dep in build_dependencies_iter
        {
          write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
        }
        // unwrap - safe because `is_empty` check
        write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
      }

      Ok( f )
    }
  }

  impl std::fmt::Display for ListNodeReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.display_with_spacer( "" )? )?;

      Ok( () )
    }
  }

  /// Represents the different report formats for the `list` action.
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// Represents a tree-like report format.
    Tree( Vec< ListNodeReport > ),
    /// Represents a standard list report format in topological order.
    List( Vec< String > ),
    /// Represents an empty report format.
    #[ default ]
    Empty,
  }

  impl std::fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        Self::Tree( v ) => write!( f, "{}", v.iter().map( | l | l.to_string() ).collect::< Vec< _ > >().join( "\n" ) ),
        Self::List( v ) => write!( f, "{}", v.iter().enumerate().map( |( i, v )| format!( "[{i}] {v}" ) ).collect::< Vec< _ > >().join( "\n" ) ),
        Self::Empty => write!( f, "Nothing" ),
      }
    }
  }

  fn process_package_dependency
  (
    workspace : &Workspace,
    package : &WorkspacePackage,
    args : &ListOptions,
    dep_rep : &mut ListNodeReport,
    visited : &mut HashSet< String >
  )
  {
    for dependency in &package.dependencies()
    {
      if dependency.path().is_some() && !args.dependency_sources.contains( &DependencySource::Local ) { continue; }
      if dependency.path().is_none() && !args.dependency_sources.contains( &DependencySource::Remote ) { continue; }
      let dep_id = format!( "{}+{}+{}", dependency.name(), dependency.req(), dependency.path().as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );

      let mut temp_vis = visited.clone();
      let dependency_rep = process_dependency( workspace, dependency, args, &mut temp_vis );

      match dependency.kind()
      {
        workspace::DependencyKind::Normal if args.dependency_categories.contains( &DependencyCategory::Primary ) => dep_rep.normal_dependencies.push( dependency_rep ),
        workspace::DependencyKind::Development if args.dependency_categories.contains( &DependencyCategory::Dev ) => dep_rep.dev_dependencies.push( dependency_rep ),
        workspace::DependencyKind::Build if args.dependency_categories.contains( &DependencyCategory::Build ) => dep_rep.build_dependencies.push( dependency_rep ),
        _ => { visited.remove( &dep_id ); std::mem::swap( &mut temp_vis, visited ); }
      }

      *visited = std::mem::take( &mut temp_vis );
    }
  }

  fn process_dependency( workspace : &Workspace, dep : &workspace::Dependency, args : &ListOptions, visited : &mut HashSet< String > ) -> ListNodeReport
  {
    let mut dep_rep = ListNodeReport
    {
      name : dep.name().clone(),
      version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( dep.req().to_string() ) } else { None },
      path : if args.info.contains( &PackageAdditionalInfo::Path ) { dep.path().as_ref().map( | p | p.clone().into_std_path_buf() ) } else { None },
      duplicate : false,
      normal_dependencies : vec![],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    };

    let dep_id = format!( "{}+{}+{}", dep.name(), dep.req(), dep.path().as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );
    // if this is a cycle (we have visited this node before)
    if visited.contains( &dep_id )
    {
      dep_rep.duplicate = true;

      return dep_rep;
    }

    // if we have not visited this node before, mark it as visited
    visited.insert( dep_id );
    if let Some( path ) = &dep.path()
    {
      if let Some( package ) = workspace.package_find_by_manifest( path.as_std_path().join( "Cargo.toml" ) )
      {
        process_package_dependency( workspace, &package, args, &mut dep_rep, visited );
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

  /// Retrieve a list of packages based on the given arguments.
  ///
  /// # Arguments
  ///
  /// - `args`: ListOptions - The arguments for listing packages.
  ///
  /// # Returns
  ///
  /// - `Result<ListReport, (ListReport, Error)>` - A result containing the list report if successful,
  ///   or a tuple containing the list report and error if not successful.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn list( args : ListOptions ) -> Result< ListReport, ( ListReport, Error ) >
  {
    let mut report = ListReport::default();

    let manifest = manifest::open( args.path_to_manifest.absolute_path() ).context( "List of packages by specified manifest path" ).err_with( report.clone() )?;
    let metadata = Workspace::with_crate_dir( manifest.crate_dir() ).err_with( report.clone() )?;

    let is_package = manifest.package_is().context( "try to identify manifest type" ).err_with( report.clone() )?;

    let tree_package_report = | path : AbsolutePath, report : &mut ListReport, visited : &mut HashSet< String > |
    {
      let package = metadata.package_find_by_manifest( path ).unwrap();
      let mut package_report = ListNodeReport
      {
        name : package.name().to_string(),
        version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( package.version().to_string() ) } else { None },
        path : if args.info.contains( &PackageAdditionalInfo::Path ) { Some( package.manifest_path().as_std_path().to_path_buf() ) } else { None },
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      };

      process_package_dependency( &metadata, &package, &args, &mut package_report, visited );

      *report = match report
      {
        ListReport::Tree( ref mut v ) => ListReport::Tree( { v.extend([ package_report ]); v.clone() } ),
        ListReport::Empty => ListReport::Tree( vec![ package_report ] ),
        ListReport::List( _ ) => unreachable!(),
      };
    };
    match args.format
    {
      ListFormat::Tree if is_package =>
      {
        let mut visited = HashSet::new();
        tree_package_report( manifest.manifest_path, &mut report, &mut visited );
        let ListReport::Tree( tree ) = report else { unreachable!() };
        let tree = rearrange_duplicates( merge_dev_dependencies( merge_build_dependencies( tree ) ) );
        report = ListReport::Tree( tree );
      }
      ListFormat::Tree =>
      {
        let packages = metadata.packages().context( "workspace packages" ).err_with( report.clone() )?;
        let mut visited = packages.iter().map( | p | format!( "{}+{}+{}", p.name(), p.version().to_string(), p.manifest_path() ) ).collect();
        for package in packages
        {
          tree_package_report( package.manifest_path().as_std_path().try_into().unwrap(), &mut report, &mut visited )
        }
        let ListReport::Tree( tree ) = report else { unreachable!() };
        let tree = merge_dev_dependencies( merge_build_dependencies( tree ) );
        report = ListReport::Tree( tree );
      }
      ListFormat::Topological =>
      {
        let root_crate = manifest
        .manifest_data
        .as_ref()
        .and_then( | m | m.get( "package" ) )
        .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
        .unwrap_or_default();

        let dep_filter = move | _p : &WorkspacePackage, d : &workspace::Dependency |
        {
          (
            args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind() == workspace::DependencyKind::Normal
            || args.dependency_categories.contains( &DependencyCategory::Dev ) && d.kind() == workspace::DependencyKind::Development
            || args.dependency_categories.contains( &DependencyCategory::Build ) && d.kind() == workspace::DependencyKind::Build
          )
          &&
          (
            args.dependency_sources.contains( &DependencySource::Remote ) && d.path().is_none()
            || args.dependency_sources.contains( &DependencySource::Local ) && d.path().is_some()
          )
        };

        let packages = metadata.packages().context( "workspace packages" ).err_with( report.clone() )?;
        let packages_map =  packages::filter
        (
          packages.as_slice(),
          FilterMapOptions { dependency_filter : Some( Box::new( dep_filter ) ), ..Default::default() }
        );

        let graph = graph::construct( &packages_map );

        let sorted = toposort( &graph, None ).map_err( | e | { use std::ops::Index; ( report.clone(), err!( "Failed to process toposort for package : {:?}", graph.index( e.node_id() ) ) ) } )?;
        let packages_info = packages.iter().map( | p | ( p.name().clone(), p ) ).collect::< HashMap< _, _ > >();

        if root_crate.is_empty()
        {
          let names = sorted
          .iter()
          .rev()
          .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
          .map
          (
            | mut name |
            {
              if let Some( p ) = packages_info.get( &name )
              {
                if args.info.contains( &PackageAdditionalInfo::Version )
                {
                  name.push_str( " " );
                  name.push_str( &p.version().to_string() );
                }
                if args.info.contains( &PackageAdditionalInfo::Path )
                {
                  name.push_str( " " );
                  name.push_str( &p.manifest_path().to_string() );
                }
              }
              name
            }
          )
          .collect::< Vec< String > >();

          report = ListReport::List( names );
        }
        else
        {
          let node = graph.node_indices().find( | n | graph.node_weight( *n ).unwrap() == &&root_crate ).unwrap();
          let mut dfs = Dfs::new( &graph, node );
          let mut subgraph = Graph::new();
          let mut node_map = HashMap::new();
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
            let mut name = subgraph[ n ].clone();
            if let Some( p ) = packages_info.get( &name )
            {
              if args.info.contains( &PackageAdditionalInfo::Version )
              {
                name.push_str( " " );
                name.push_str( &p.version().to_string() );
              }
              if args.info.contains( &PackageAdditionalInfo::Path )
              {
                name.push_str( " " );
                name.push_str( &p.manifest_path().to_string() );
              }
            }
            names.push( name );
          }
          names.reverse();

          report = ListReport::List( names );
        }
      }
    }

    Ok( report )
  }

  fn merge_build_dependencies( mut report: Vec< ListNodeReport > ) -> Vec< ListNodeReport >
  {
    let mut build_dependencies = vec![];
    for node_report in &mut report
    {
      build_dependencies = merge_build_dependencies_impl( node_report, build_dependencies );
    }
    if let Some( last_report ) = report.last_mut()
    {
      last_report.build_dependencies = build_dependencies;
    }

    report
  }
  
  fn merge_build_dependencies_impl( report : &mut ListNodeReport, mut build_deps_acc : Vec< ListNodeReport > ) -> Vec< ListNodeReport >
  {
    for dep in report.normal_dependencies.iter_mut()
      .chain( report.dev_dependencies.iter_mut() )
      .chain( report.build_dependencies.iter_mut() )
    {
      build_deps_acc = merge_build_dependencies_impl(dep, build_deps_acc );
    }

    for dep in std::mem::take( &mut report.build_dependencies )
    {
      if !build_deps_acc.contains( &dep )
      {
        build_deps_acc.push( dep );
      }
    }

    build_deps_acc
  }
  
  fn merge_dev_dependencies( mut report: Vec< ListNodeReport > ) -> Vec< ListNodeReport >
  {
    let mut dev_dependencies = vec![];
    for node_report in &mut report
    {
      dev_dependencies = merge_dev_dependencies_impl( node_report, dev_dependencies );
    }
    if let Some( last_report ) = report.last_mut()
    {
      last_report.dev_dependencies = dev_dependencies;
    }

    report
  }

  fn merge_dev_dependencies_impl( report : &mut ListNodeReport, mut dev_deps_acc : Vec< ListNodeReport > ) -> Vec< ListNodeReport >
  {
    for dep in report.normal_dependencies.iter_mut()
    .chain( report.dev_dependencies.iter_mut() )
    .chain( report.build_dependencies.iter_mut() )
    {
      dev_deps_acc = merge_dev_dependencies_impl( dep, dev_deps_acc );
    }

    for dep in std::mem::take( &mut report.dev_dependencies )
    {
      if !dev_deps_acc.contains( &dep )
      {
        dev_deps_acc.push( dep );
      }
    }

    dev_deps_acc
  }
  
  fn rearrange_duplicates( mut report : Vec< ListNodeReport > ) -> Vec< ListNodeReport >
  {
    let mut required_normal : HashMap< usize, Vec< ListNodeReport > > = HashMap::new();
    for i in 0 .. report.len()
    {
      let ( required, exist ) : ( Vec< _ >, Vec< _ > ) = std::mem::take( &mut report[ i ].normal_dependencies ).into_iter().partition( | d | d.duplicate );
      report[ i ].normal_dependencies = exist;
      required_normal.insert( i, required );
    }
    
    rearrange_duplicates_resolver( &mut report, &mut required_normal );
    for ( i, deps ) in required_normal
    {
      report[ i ].normal_dependencies.extend( deps );
    }
    
    report
  }
  
  fn rearrange_duplicates_resolver( report : &mut [ ListNodeReport ], required : &mut HashMap< usize, Vec< ListNodeReport > > )
  {
    for node in report
    {
      rearrange_duplicates_resolver( &mut node.normal_dependencies, required );
      rearrange_duplicates_resolver( &mut node.dev_dependencies, required );
      rearrange_duplicates_resolver( &mut node.build_dependencies, required );

      if !node.duplicate
      {
        if let Some( r ) = required.iter_mut().flat_map( |( _, v )| v )
        .find( | r | r.name == node.name && r.version == node.version && r.path == node.path )
        {
          std::mem::swap( r, node );
        }
      }
    }
  }
}

//

crate::mod_interface!
{
  /// Arguments for `list` action.
  protected use ListOptions;
  /// Additional information to include in a package report.
  protected use PackageAdditionalInfo;
  /// Represents where a dependency located.
  protected use DependencySource;
  /// Represents the category of a dependency.
  protected use DependencyCategory;
  /// Argument for `list` action. Sets the output format.
  protected use ListFormat;
  /// Argument for `list` action. Sets filter(local or all) packages should be in the output.
  protected use ListFilter;
  /// Contains output of the action.
  protected use ListReport;
  /// Contains output of a single node of the action.
  protected use ListNodeReport;
  /// List packages in workspace.
  orphan use list;
}
