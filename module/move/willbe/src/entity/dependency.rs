mod private
{

  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;

  // use crates_tools::CrateArchive;
  // use workspace::Workspace;
  use error::
  {
    // untyped::Result,
    // typed::Error,
    untyped::format_err,
  };

  /// A dependency of the main crate
  #[ derive( Debug, Clone, Copy ) ]
  #[ repr( transparent ) ]
  pub struct DependencyRef< 'a >
  {
    inner : &'a cargo_metadata::Dependency,
  }

  impl< 'a > DependencyRef< 'a >
  {

    /// The file system path for a local path dependency.
    /// Only produced on cargo 1.51+
    #[ must_use ]
    pub fn crate_dir( &self ) -> Option< CrateDir >
    {
      match &self.inner.path
      {
        Some( path ) => path.as_path().try_into().ok(),
        None => None,
      }
    }

    /// Name as given in the Cargo.toml.
    #[ must_use ]
    pub fn name( &self ) -> String
    {
      self.inner.name.clone()
    }

    /// The kind of dependency this is.
    #[ must_use ]
    pub fn kind( &self ) -> DependencyKind
    {
      match self.inner.kind
      {
        cargo_metadata::DependencyKind::Normal => DependencyKind::Normal,
        cargo_metadata::DependencyKind::Development => DependencyKind::Development,
        cargo_metadata::DependencyKind::Build => DependencyKind::Build,
        cargo_metadata::DependencyKind::Unknown => DependencyKind::Unknown,
      }
    }

    /// Required version
    #[ must_use ]
    pub fn req( &self ) -> semver::VersionReq
    {
      self.inner.req.clone()
    }
  }

  impl< 'a > From< &'a cargo_metadata::Dependency > for DependencyRef< 'a >
  {
    #[ inline( always ) ]
    fn from( inner : &'a cargo_metadata::Dependency ) -> Self
    {
      Self { inner }
    }
  }

  /// Dependencies can come in three kinds
  #[ derive( Eq, PartialEq, Debug, Clone, Copy ) ]
  pub enum DependencyKind
  {
    /// The 'normal' kind
    Normal,
    /// Those used in tests only
    Development,
    /// Those used in build scripts only
    Build,
    /// The 'unknown' kind
    Unknown,
  }

  //

  /// Identifier of any crate (local and remote).
  #[ derive( Debug, Clone, Hash, Eq, PartialEq ) ]
  pub struct CrateId
  {
    /// The name of the crate.
    pub name : String, // qqq : that should be Arc< str >
    /// The absolute path to the crate, if available.
    pub crate_dir : Option< CrateDir >, // qqq : that should be Option< Arc< CrateDir > >
    // pub path : Option< AbsolutePath >,
  }

  impl< 'a > From< &WorkspacePackageRef< 'a > > for CrateId
  {
    fn from( value : &WorkspacePackageRef< 'a > ) -> Self
    {
      Self
      {
        name : value.name().into(),
        crate_dir : Some( value.crate_dir().unwrap() )
        // path : Some( AbsolutePath::try_from( value.manifest_file().parent().unwrap() ).unwrap() ),
      }
    }
  }

  impl From< &DependencyRef< '_ > > for CrateId
  {
    fn from( value : &DependencyRef< '_ > ) -> Self
    {
      Self
      {
        name : value.name(),
        crate_dir : value.crate_dir(),
        // path : value.path().clone().map( | path | AbsolutePath::try_from( path ).unwrap() ),
      }
    }
  }

  /// Sorting variants for dependencies.
  #[ derive( Debug, Copy, Clone ) ]
  pub enum DependenciesSort
  {
    /// List will be topologically sorted.
    Topological,
    /// List will be unsorted.
    Unordered,
  }

  #[ derive( Debug, Clone ) ]
  /// Args for `local_dependencies` function.
  pub struct DependenciesOptions
  {
    /// With dependencies of dependencies.
    pub recursive : bool,
    /// With sorting.
    pub sort : DependenciesSort,
    /// Include dev dependencies.
    pub with_dev : bool,
    /// Include remote dependencies.
    pub with_remote : bool,
  }

  impl Default for DependenciesOptions
  {
    fn default() -> Self
    {
      Self
      {
        recursive : true,
        sort : DependenciesSort::Unordered,
        with_dev : false,
        with_remote : false,
      }
    }
  }

  // qqq : for Bohdan : poor description
  /// Recursive implementation of the `list` function
  /// # Errors
  /// qqq: doc
  ///
  /// # Panics
  /// qqq: doc
  #[ allow( clippy::needless_pass_by_value, clippy::implicit_hasher ) ]
  pub fn _list
  (
    workspace : &Workspace, // aaa : for Bohdan : no mut // aaa : no mut
    package : &Package< '_ >,
    graph : &mut collection::HashMap< CrateId, collection::HashSet< CrateId > >,
    opts : DependenciesOptions
  )
  // qqq : use typed error
  -> error::untyped::Result< CrateId >
  {
    let DependenciesOptions
    {
      recursive,
      sort : _,
      with_dev,
      with_remote,
    } = opts;
    if recursive && with_remote { unimplemented!( "`recursive` + `with_remote` options") }

    let manifest_file = &package.manifest_file();

    let package = workspace
    .package_find_by_manifest( manifest_file )
    .ok_or( format_err!( "Package not found in the workspace with path : `{}`", manifest_file.as_ref().display() ) )?;

    let deps : collection::HashSet< _ > = package
    .dependencies()
    // .iter()
    .filter( | dep | ( with_remote || dep.crate_dir().is_some() ) && ( with_dev || dep.kind() != DependencyKind::Development ) )
    .map( | dep | CrateId::from( &dep ) )
    .collect();

    let package = CrateId::from( &package );
    graph.insert( package.clone(), deps.clone() );

    if recursive
    {
      for dep in deps
      {
        if graph.get( &dep ).is_none()
        {
          // unwrap because `recursive` + `with_remote` not yet implemented
          _list
          (
            workspace,
            &dep.crate_dir.unwrap().try_into()?,
            // &dep.path.as_ref().unwrap().join( "Cargo.toml" ).try_into().unwrap(),
            graph,
            opts.clone(),
          )?;
        }
      }
    }

    Ok( package )
  }

  /// Returns local dependencies of a specified package by its package path from a workspace.
  ///
  /// # Arguments
  ///
  /// - `workspace` - holds cached information about the workspace, such as the packages it contains and their dependencies. By passing it as a mutable reference, function can update the cache as needed.
  /// - `package` - The package package file contains package about the package such as its name, version, and dependencies.
  /// - `opts` - used to specify options or configurations for fetching local dependencies.
  ///
  /// # Returns
  ///
  /// If the operation is successful, returns a vector of `PathBuf` objects, where each `PathBuf` represents the path to a local dependency of the specified package.
  /// # Errors
  /// qqq: doc
  // qqq : typed error?
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn list
  (
    workspace : &mut Workspace,
    package : &Package< '_ >,
    opts : DependenciesOptions
  )
  // qqq : use typed error
  -> error::untyped::Result< Vec< CrateId > >
  {
    let mut graph = collection::HashMap::new();
    let root = _list( workspace, package, &mut graph, opts.clone() )?;

    let output = match opts.sort
    {
      DependenciesSort::Unordered =>
      {
        graph
        .into_iter()
        .flat_map( | ( id, dependency ) |
        {
          dependency
          .into_iter()
          .chain( Some( id ) )
        })
        .unique()
        .filter( | x | x != &root )
        .collect()
      }
      DependenciesSort::Topological =>
      {
        // aaa : too long line
        // aaa : splited
        graph::toposort( graph::construct( &graph ) )
        .map_err( | err | format_err!( "{}", err ) )?
        .into_iter()
        .filter( | x | x != &root )
        .collect()
      },
    };

    Ok( output )
  }

}

//

crate::mod_interface!
{

  exposed use DependencyRef;
  exposed use DependencyKind;

  own use CrateId;
  own use DependenciesSort;
  own use DependenciesOptions;
  own use _list;
  own use list;

}
