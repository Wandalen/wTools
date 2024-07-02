/// Internal namespace.
mod private
{
  use crate::*;

  use std::{ fmt, str };
  use petgraph::
  {
    prelude::{ Dfs, EdgeRef },
    algo::toposort,
    visit::Topo,
    Graph,
  };
  use error::
  {
    ErrWith, err,
    untyped::{ Context, format_err },
  };
  use tool::{ TreePrinter, ListNodeReport };

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

  impl str::FromStr for ListFormat
  {
    type Err = error::untyped::Error;

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

  impl str::FromStr for ListFilter
  {
    type Err = error::untyped::Error;

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
  #[ derive( Debug, former::Former ) ]
  pub struct ListOptions
  {
    path_to_manifest : CrateDir,
    format : ListFormat,
    info : collection::HashSet< PackageAdditionalInfo >,
    dependency_sources : collection::HashSet< DependencySource >,
    dependency_categories : collection::HashSet< DependencyCategory >,
  }

  // struct Symbols
  // {
  //   down : &'static str,
  //   tee : &'static str,
  //   ell : &'static str,
  //   right : &'static str,
  // }

  // // qqq : for Mykyta : make facade, abstract and move out tree printing. or reuse ready solution for tree printing
  // // stick to single responsibility
  // const UTF8_SYMBOLS : Symbols = Symbols
  // {
  //   down : "│",
  //   tee  : "├",
  //   ell  : "└",
  //   right : "─",
  // };

  // /// Represents a node in a dependency graph.
  // /// It holds essential information about the project dependencies. It is also capable
  // /// of holding any nested dependencies in a recursive manner, allowing the modeling
  // /// of complex dependency structures.
  // #[ derive( Debug, Clone, Eq, PartialEq ) ]
  // pub struct ListNodeReport
  // {
  //   /// This could be the name of the library or crate.
  //   pub name : String,
  //   /// Ihe version of the crate.
  //   pub version : Option< String >,
  //   /// The path to the node's source files in the local filesystem. This is
  //   /// optional as not all nodes may have a local presence (e.g., nodes representing remote crates).
  //   pub crate_dir : Option< CrateDir >,
  //   /// This field is a flag indicating whether the Node is a duplicate or not.
  //   pub duplicate : bool,
  //   /// A list that stores normal dependencies.
  //   /// Each element in the list is also of the same 'ListNodeReport' type to allow
  //   /// storage of nested dependencies.
  //   pub normal_dependencies : Vec< ListNodeReport >,
  //   /// A list that stores dev dependencies(dependencies required for tests or examples).
  //   /// Each element in the list is also of the same 'ListNodeReport' type to allow
  //   /// storage of nested dependencies.
  //   pub dev_dependencies : Vec< ListNodeReport >,
  //   /// A list that stores build dependencies.
  //   /// Each element in the list is also of the same 'ListNodeReport' type to allow
  //   /// storage of nested dependencies.
  //   pub build_dependencies : Vec< ListNodeReport >,
  // }

  // impl ListNodeReport
  // {
  //   /// Displays the name, version, path, and dependencies of a package with appropriate indentation and spacing.
  //   ///
  //   /// # Arguments
  //   ///
  //   /// * `spacer` - A string used for indentation.
  //   ///
  //   /// # Returns
  //   ///
  //   /// * A `Result` containing the formatted string or a `std::fmt::Error` if formatting fails.
  //   pub fn display_with_spacer( &self, spacer : &str ) -> Result< String, std::fmt::Error >
  //   {
  //     let mut f = String::new();

  //     write!( f, "{}", self.name )?;
  //     if let Some( version ) = &self.version { write!( f, " {version}" )? }
  //     if let Some( crate_dir ) = &self.crate_dir { write!( f, " {}", crate_dir )? }
  //     if self.duplicate { write!( f, "(*)" )? }
  //     write!( f, "\n" )?;

  //     let mut new_spacer = format!( "{spacer}{}  ", if self.normal_dependencies.len() < 2 { " " } else { UTF8_SYMBOLS.down } );
  //     let mut normal_dependencies_iter = self.normal_dependencies.iter();
  //     let last = normal_dependencies_iter.next_back();

  //     for dep in normal_dependencies_iter
  //     {
  //       write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
  //     }
  //     if let Some( last ) = last
  //     {
  //       new_spacer = format!( "{spacer}   " );
  //       write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.display_with_spacer( &new_spacer )? )?;
  //     }
  //     if !self.dev_dependencies.is_empty()
  //     {
  //       let mut dev_dependencies_iter = self.dev_dependencies.iter();
  //       let last = dev_dependencies_iter.next_back();
  //       write!( f, "{spacer}[dev-dependencies]\n" )?;
  //       for dep in dev_dependencies_iter
  //       {
  //         write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
  //       }
  //       // unwrap - safe because `is_empty` check
  //       write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
  //     }
  //     if !self.build_dependencies.is_empty()
  //     {
  //       let mut build_dependencies_iter = self.build_dependencies.iter();
  //       let last = build_dependencies_iter.next_back();
  //       write!( f, "{spacer}[build-dependencies]\n" )?;
  //       for dep in build_dependencies_iter
  //       {
  //         write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.tee, UTF8_SYMBOLS.right, dep.display_with_spacer( &new_spacer )? )?;
  //       }
  //       // unwrap - safe because `is_empty` check
  //       write!( f, "{spacer}{}{} {}", UTF8_SYMBOLS.ell, UTF8_SYMBOLS.right, last.unwrap().display_with_spacer( &new_spacer )? )?;
  //     }

  //     Ok( f )
  //   }
  // }

  // impl std::fmt::Display for ListNodeReport
  // {
  //   fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
  //   {
  //     write!( f, "{}", self.display_with_spacer( "" )? )?;

  //     Ok( () )
  //   }
  // }

  /// Represents the different report formats for the `list` action.
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// Represents a tree-like report format.
    Tree( Vec< tool::TreePrinter > ),
    /// Represents a standard list report format in topological order.
    List( Vec< String > ),
    /// Represents an empty report format.
    #[ default ]
    Empty,
  }

  impl fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      match self
      {
        Self::Tree( v ) =>
        write!
        (
          f,
          "{}",
          v.iter().map( | l | l.to_string() ).collect::< Vec< _ > >().join( "\n" )
        ),

        Self::List( v ) =>
        write!
        (
          f,
          "{}",
          v.iter().enumerate().map( |( i, v )| format!( "[{i}] {v}" ) ).collect::< Vec< _ > >().join( "\n" )
        ),

        Self::Empty => write!( f, "Nothing" ),
      }
    }
  }

  // aaa : for Bohdan : descirption // aaa : done
  /// The `DependencyId` struct encapsulates the essential attributes of a dependency,
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct DependencyId
  {
    /// The name of the dependency.
    ///
    /// This is typically the name of the library or package that the package relies on.
    pub name : String,
    /// The version requirements for the dependency.
    ///
    /// Note: This will be compared to other dependencies and packages to build the tree
    pub version : semver::VersionReq,
    /// An optional path to the manifest file of the dependency.
    ///
    /// This field may contain a path to the manifest file when the dependency is a local package
    /// or when specific path information is needed to locate the dependency's manifest.
    pub path : Option< ManifestFile >,
  }

  fn process_package_dependency< 'a >
  (
    workspace : &Workspace,
    package : &WorkspacePackageRef< 'a >,
    args : &ListOptions,
    dep_rep : &mut tool::ListNodeReport,
    visited : &mut collection::HashSet< DependencyId >
  )
  {
    for dependency in package.dependencies()
    {

      // aaa : for Bohdan : bad : suboptimal
      // aaa : Is that what you had in mind?
      let dep_crate_dir = dependency.crate_dir();
      if dep_crate_dir.is_some() && !args.dependency_sources.contains( &DependencySource::Local ) { continue; }
      if dep_crate_dir.is_none() && !args.dependency_sources.contains( &DependencySource::Remote ) { continue; }

      // aaa : extend test coverage. NewType. Description
      // aaa : NewType ✅ Description ✅ test coverage ❌ how to test structure without logic?
      // qqq : extend test coverage. NewType. Description
      let dep_id = DependencyId
      {
        name : dependency.name(),
        // unwrap should be safe because of `semver::VersionReq`
        version : dependency.req(),
        path : dependency.crate_dir().map( | p | p.manifest_file() ),
      };
      // format!( "{}+{}+{}", dependency.name(), dependency.req(), dependency.crate_dir().unwrap().manifest_file() );
      // let dep_id = format!( "{}+{}+{}", dependency.name(), dependency.req(), dependency.path().as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );

      let mut temp_vis = visited.clone();
      let dependency_rep = process_dependency
      (
        workspace,
        dependency,
        args,
        &mut temp_vis
      );
      match dependency.kind()
      {
        DependencyKind::Normal if args.dependency_categories.contains( &DependencyCategory::Primary ) =>
        dep_rep.normal_dependencies.push( dependency_rep ),
        DependencyKind::Development if args.dependency_categories.contains( &DependencyCategory::Dev ) =>
        dep_rep.dev_dependencies.push( dependency_rep ),
        DependencyKind::Build if args.dependency_categories.contains( &DependencyCategory::Build ) =>
        dep_rep.build_dependencies.push( dependency_rep ),
        _ => { visited.remove( &dep_id ); std::mem::swap( &mut temp_vis, visited ); }
      }

      *visited = std::mem::take( &mut temp_vis );
    }
  }

  fn process_dependency
  (
    workspace : &Workspace,
    dep : DependencyRef< '_ >,
    args : &ListOptions,
    visited : &mut collection::HashSet< DependencyId >
  )
  -> tool::ListNodeReport
  {
    let mut dep_rep = tool::ListNodeReport
    {
      name : dep.name().clone(),
      version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( dep.req().to_string() ) } else { None },
      // manifest_file : if args.info.contains( &PackageAdditionalInfo::Path ) { dep.manifest_file().as_ref().map( | p | p.clone().into_std_path_buf() ) } else { None },
      crate_dir : if args.info.contains( &PackageAdditionalInfo::Path ) { dep.crate_dir() } else { None },
      duplicate : false,
      normal_dependencies : vec![],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    };

    // let dep_id = format!( "{}+{}+{}", dep.name(), dep.req(), dep.crate_dir().as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );
    let dep_id = DependencyId
    {
      name : dep.name(),
      // unwrap should be safe because of `semver::VersionReq`
      version : dep.req(),
      path : dep.crate_dir().map( | p | p.manifest_file() ),
    };
    // if this is a cycle (we have visited this node before)
    if visited.contains( &dep_id )
    {
      dep_rep.duplicate = true;

      return dep_rep;
    }

    // if we have not visited this node before, mark it as visited
    visited.insert( dep_id );
    if let Some( crate_dir ) = &dep.crate_dir()
    {
      if let Some( package ) = workspace.package_find_by_manifest( crate_dir.clone().manifest_file() )
      {
        process_package_dependency( workspace, &package, args, &mut dep_rep, visited );
      }
    }

    dep_rep
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
  pub fn list( args : ListOptions )
  ->
  ResultWithReport< ListReport, error::untyped::Error > // qqq : should be specific error
  // qqq : use typed error
  {
    let mut report = ListReport::default();

    let manifest = Manifest::try_from( args.path_to_manifest.clone() )
    .context( "List of packages by specified manifest path" )
    .err_with_report( &report )?;

    let workspace = Workspace::try_from( manifest.crate_dir() )
    .context( "Reading workspace" )
    .err_with_report( &report )?;

    let is_package = manifest.package_is();
    // let is_package = manifest.package_is().context( "try to identify manifest type" ).err_with( report.clone() )?;

    let tree_package_report =
    | manifest_file : ManifestFile, report : &mut ListReport, visited : &mut HashSet< DependencyId > |
    {

      let package = workspace
      .package_find_by_manifest( manifest_file )
      .ok_or_else( || format_err!( "Package not found in the workspace" ) )
      .err_with_report( report )?;
      let mut package_report = tool::ListNodeReport
      {
        name : package.name().to_string(),
        // qqq : for Bohdan : too long lines
        version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( package.version().to_string() ) } else { None },
        // qqq : for Bohdan : don't put multiline if into struct constructor
        crate_dir : if args.info.contains( &PackageAdditionalInfo::Path )
        { Some( package.crate_dir() ).transpose() }
        else
        { Ok( None ) }
        .err_with_report( report )?,
        duplicate : false,
        normal_dependencies : vec![],
        dev_dependencies : vec![],
        build_dependencies : vec![],
      };

      process_package_dependency( &workspace, &package, &args, &mut package_report, visited );

      let printer = TreePrinter::new( &package_report );
      *report = match report
      {
        ListReport::Tree( ref mut v ) => ListReport::Tree
        ( { v.extend([ printer ]); v.clone() } ),
        ListReport::Empty => ListReport::Tree( vec![ printer ] ),
        ListReport::List( _ ) => unreachable!(),
      };
      Ok( () )
    };

    match args.format
    {
      ListFormat::Tree if is_package =>
      {
        let mut visited = collection::HashSet::new();
        tree_package_report( manifest.manifest_file, &mut report, &mut visited )?;
        let ListReport::Tree( tree ) = report else { unreachable!() };
        let printer = merge_build_dependencies( tree );
        let rep : Vec< ListNodeReport > = printer
        .iter()
        .map( | printer | printer.info.clone() )
        .collect();
        let tree = rearrange_duplicates( rep );
        report = ListReport::Tree( tree );
      }
      ListFormat::Tree =>
      {
        let packages = workspace.packages();
        let mut visited = packages
        .clone()
        .map
        (
          // aaa : is it safe to use unwrap here
          // unwrap is safe because Version has less information than VersionReq
          | p |
          DependencyId
          {
            name : p.name().into(),
            version : semver::VersionReq::parse( &p.version().to_string() ).unwrap(),
            path : p.manifest_file().ok()
          }
        )
        .collect();
        for package in packages
        {
          tree_package_report( package.manifest_file().unwrap(), &mut report, &mut visited )?
        }
        let ListReport::Tree( tree ) = report else { unreachable!() };
        let printer = merge_build_dependencies( tree );
        let rep : Vec< ListNodeReport > = printer
        .iter()
        .map( | printer | printer.info.clone() )
        .collect();
        let tree = merge_dev_dependencies( rep );
        report = ListReport::Tree( tree );
      }
      ListFormat::Topological =>
      {

        let root_crate = manifest.data.get( "package" )
        .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
        .unwrap_or_default();

        // let root_crate = manifest
        // .data
        // // .as_ref()
        // .and_then( | m | m.get( "package" ) )
        // .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
        // .unwrap_or_default();

        let dep_filter = move | _p : WorkspacePackageRef< '_ >, d : DependencyRef< '_ > |
        {
          (
            args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind() == DependencyKind::Normal
            || args.dependency_categories.contains( &DependencyCategory::Dev ) && d.kind() == DependencyKind::Development
            || args.dependency_categories.contains( &DependencyCategory::Build ) && d.kind() == DependencyKind::Build
          )
          &&
          (
            args.dependency_sources.contains( &DependencySource::Remote ) && d.crate_dir().is_none()
            || args.dependency_sources.contains( &DependencySource::Local ) && d.crate_dir().is_some()
          )
        };

        let packages = workspace.packages();
        let packages_map : collection::HashMap< package::PackageName, collection::HashSet< package::PackageName > > = packages::filter
        (
          packages.clone(),
          packages::FilterMapOptions
          {
            dependency_filter : Some( Box::new( dep_filter ) ),
            ..Default::default()
          }
        );

        let graph = graph::construct( &packages_map );

        let sorted = toposort( &graph, None )
        .map_err
        (
          | e |
          {
            use std::ops::Index;
            format_err!
            (
              "Failed to process toposort for package : {:?}",
              graph.index( e.node_id() )
            )
          }
        )
        .err_with_report( &report )?;
        let packages_info : collection::HashMap< String, WorkspacePackageRef< '_ > > =
          packages.map( | p | ( p.name().to_string(), p ) ).collect();

        if root_crate.is_empty()
        {
          let names : Vec< String > = sorted
          .into_iter()
          .rev()
          .map( | dep_idx | graph.node_weight( dep_idx ).unwrap() )
          .map
          (
            | name : &&package::PackageName |
            {
              let mut name : String = name.to_string();
              if let Some( p ) = packages_info.get( &name[ .. ] )
              {
                if args.info.contains( &PackageAdditionalInfo::Version )
                {
                  name.push_str( " " );
                  name.push_str( &p.version().to_string() );
                }
                if args.info.contains( &PackageAdditionalInfo::Path )
                {
                  name.push_str( " " );
                  name.push_str( &p.manifest_file()?.to_string() );
                  // aaa : is it safe to use unwrap here? // aaa : should be safe, but now returns an error
                }
              }
              Ok::< String, PathError >( name )
            }
          )
          .collect::< Result< _, _ >>()
          .err_with_report( &report )?;

          report = ListReport::List( names );
        }
        else
        {
          let node = graph
          .node_indices()
          .find( | n | graph.node_weight( *n ).unwrap().as_str() == root_crate )
          .unwrap();
          let mut dfs = Dfs::new( &graph, node );
          let mut subgraph = Graph::new();
          let mut node_map = collection::HashMap::new();
          while let Some( n )= dfs.next( &graph )
          {
            node_map.insert( n, subgraph.add_node( graph[ n ] ) );
          }

          for e in graph.edge_references()
          {
            if let ( Some( &s ), Some( &t ) ) =
            (
              node_map.get( &e.source() ),
              node_map.get( &e.target() )
            )
            {
              subgraph.add_edge( s, t, () );
            }
          }

          let mut topo = Topo::new( &subgraph );
          let mut names = Vec::new();
          while let Some( n ) = topo.next( &subgraph )
          {
            let mut name : String = subgraph[ n ].to_string();
            if let Some( p ) = packages_info.get( &name[ .. ] )
            {
              if args.info.contains( &PackageAdditionalInfo::Version )
              {
                name.push_str( " " );
                name.push_str( &p.version().to_string() );
              }
              if args.info.contains( &PackageAdditionalInfo::Path )
              {
                name.push_str( " " );
                name.push_str( &p.manifest_file().unwrap().to_string() );
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

  fn merge_build_dependencies( mut report: Vec< tool::TreePrinter > ) -> Vec< tool::TreePrinter >
  {
    let mut build_dependencies = vec![];
    for node_report in &mut report
    {
      build_dependencies = merge_build_dependencies_impl
      (
        &mut node_report.info,
        build_dependencies
      );
    }
    if let Some( last_report ) = report.last_mut()
    {
      last_report.info.build_dependencies = build_dependencies;
    }

    report
  }

  fn merge_build_dependencies_impl
  (
    report : &mut tool::ListNodeReport,
    mut build_deps_acc : Vec< tool::ListNodeReport >
  )
  -> Vec< tool::ListNodeReport >
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

  fn merge_dev_dependencies( mut report : Vec< tool::ListNodeReport > ) -> Vec< tool::TreePrinter >
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
    let printer : Vec< TreePrinter > = report
    .iter()
    .map( | rep | TreePrinter::new( rep ) )
    .collect();
    printer
  }

  fn merge_dev_dependencies_impl
  (
    report : &mut ListNodeReport,
    mut dev_deps_acc : Vec< ListNodeReport >
  ) -> Vec< ListNodeReport >
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

  fn rearrange_duplicates( mut report : Vec< tool::ListNodeReport > ) -> Vec< tool::TreePrinter >
  {
    let mut required_normal : collection::HashMap< usize, Vec< tool::ListNodeReport > > = collection::HashMap::new();
    for i in 0 .. report.len()
    {
      let ( required, exist ) : ( Vec< _ >, Vec< _ > ) = std::mem::take
      (
        &mut report[ i ].normal_dependencies
      )
      .into_iter()
      .partition( | d | d.duplicate );
      report[ i ].normal_dependencies = exist;
      required_normal.insert( i, required );
    }

    rearrange_duplicates_resolver( &mut report, &mut required_normal );
    for ( i, deps ) in required_normal
    {
      report[ i ].normal_dependencies.extend( deps );
    }

    let printer : Vec< TreePrinter > = report
    .iter()
    .map( | rep | TreePrinter::new( rep ) )
    .collect();

    printer
  }

  fn rearrange_duplicates_resolver
  (
    report : &mut [ ListNodeReport ],
    required : &mut HashMap< usize, Vec< ListNodeReport > >
  )
  {
    for node in report
    {
      rearrange_duplicates_resolver( &mut node.normal_dependencies, required );
      rearrange_duplicates_resolver( &mut node.dev_dependencies, required );
      rearrange_duplicates_resolver( &mut node.build_dependencies, required );

      if !node.duplicate
      {
        if let Some( r ) = required.iter_mut().flat_map( |( _, v )| v )
        .find
        (
          | r |
          r.name == node.name && r.version == node.version && r.crate_dir == node.crate_dir
        )
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
  // protected use ListNodeReport;
  /// List packages in workspace.
  orphan use list;
}
