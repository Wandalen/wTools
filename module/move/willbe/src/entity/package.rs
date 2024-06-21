mod private
{
  use crate::*;

  use std::
  {
    path::Path,
    collections::{ HashMap, HashSet },
    fmt,
    hash::Hash,
    path,
  };
  // aaa : for Petro : for Bohdan : group uses
  // aaa : done

  use process_tools::process;
  use manifest::{ Manifest, ManifestError };
  use crates_tools::CrateArchive;
  use
  {
    iter::Itertools,
    error::
    {
      Result,
      typed::Error,
      untyped::{ format_err, Context, Error },
    }
  };
  use error_with::ErrWith;

  // aaa : fro Bohdan : write better description : is it better?
  /// A wrapper type for representing the name of a package.
  ///
  /// This struct encapsulates a `String` that holds the name of a package.
  #[ derive
  (
    Debug, Default, Clone, Hash, Ord, PartialOrd, Eq, PartialEq,
    derive_tools::Display, derive_tools::Deref, derive_tools::From
  ) ]
  pub struct PackageName( String );

  // aaa : fro Bohdan : write description : done
  //
  /// Represents different types of packages in a Cargo workspace.
  ///
  /// It is designed to accommodate the two primary types of package
  /// representations within a Cargo workspace.
  #[ derive( Debug, Clone ) ]
  pub enum Package< 'a >
  {
    /// `Cargo.toml` file.
    Manifest( Manifest ),
    /// Cargo package package.
    WorkspacePackageRef( WorkspacePackageRef< 'a > ),
  }

  /// Represents errors related to package handling.
  #[ derive( Debug, Error ) ]
  pub enum PackageError
  {
    /// Manifest error.
    #[ error( "Manifest error. Reason : {0}." ) ]
    Manifest( #[ from ] ManifestError ),
    /// Fail to load package.
    #[ error( "Fail to load package." ) ]
    WorkspacePackageRef,
    /// Fail to load remote package.
    #[ error( "Fail to load remote package." ) ]
    LoadRemotePackage,
    /// Fail to get crate local path.
    #[ error( "Fail to get crate local path." ) ]
    LocalPath,
    /// Fail to read archive
    #[ error( "Fail to read archive" ) ]
    ReadArchive,
    /// Try to identify something as a package.
    #[ error( "Not a package" ) ]
    NotAPackage,
  }

  impl< 'a > TryFrom< ManifestFile > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : ManifestFile ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( package ) )
    }
  }

  impl< 'a > TryFrom< CrateDir > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : CrateDir ) -> Result< Self, Self::Error >
    {
      let package = Manifest::try_from( value )?;
      if !package.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( package ) )
    }
  }

  impl< 'a > TryFrom< Manifest > for Package< 'a >
  {
    type Error = PackageError;

    fn try_from( value : Manifest ) -> Result< Self, Self::Error >
    {
      if !value.package_is()
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( value ) )
    }
  }

  impl< 'a > From< WorkspacePackageRef< 'a > > for Package< 'a >
  {
    fn from( value : WorkspacePackageRef< 'a > ) -> Self
    {
      Self::WorkspacePackageRef( value )
    }
  }

  impl< 'a > Package< 'a >
  {

    /// Path to `Cargo.toml`
    pub fn manifest_file( &self ) -> ManifestFile
    {
      match self
      {
        Self::Manifest( package ) => package.manifest_file.clone(),
        Self::WorkspacePackageRef( package ) => package.manifest_file().unwrap(),
      }
    }

    /// Path to folder with `Cargo.toml`
    pub fn crate_dir( &self ) -> CrateDir
    {
      match self
      {
        Self::Manifest( package ) => package.crate_dir(),
        Self::WorkspacePackageRef( package ) => package.crate_dir().unwrap(),
      }
    }

    /// Package version
    pub fn version( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( package ) =>
        {
          // let data = package.data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;
          let data = &package.data;

          // Unwrap safely because of the `Package` type guarantee
          Ok( data[ "package" ][ "version" ].as_str().unwrap().to_string() )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok( package.version().to_string() )
        }
      }
    }

    /// Check that module is local.
    pub fn local_is( &self ) -> bool
    {
      match self
      {
        Self::Manifest( package ) =>
        {
          // verify that package not empty
          package.local_is()
        }
        Self::WorkspacePackageRef( package ) =>
        {
          !( package.publish().is_none() || package.publish().as_ref().is_some_and( | p | p.is_empty() ) )
          // Ok( !( package.publish().is_none() || package.publish().as_ref().is_some_and( | p | p.is_empty() ) ) )
        }
      }
    }

    /// Returns the `Manifest`
    pub fn manifest( &self ) -> Result< Manifest, PackageError >
    {
      match self
      {
        Package::Manifest( package ) => Ok( package.clone() ),
        Package::WorkspacePackageRef( package ) => Manifest::try_from
        (
          package.manifest_file().map_err( | _ | PackageError::LocalPath )? // qqq : use trait
        )
        .map_err( | _ | PackageError::WorkspacePackageRef ),
      }
    }

  }

  // qqq : for Bohdan : should not be here
  #[ derive( Debug, Default, Clone ) ]
  pub struct ExtendedGitReport
  {
    pub add : Option< process::Report >,
    pub commit : Option< process::Report >,
    pub push : Option< process::Report >,
  }

  impl std::fmt::Display for ExtendedGitReport
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> std::fmt::Result
    {
      let Self { add, commit, push } = &self;

      if let Some( add ) = add { writeln!( f, "{add}" )? }
      if let Some( commit ) = commit { writeln!( f, "{commit}" )? }
      if let Some( push ) = push { writeln!( f, "{push}" )? }

      Ok( () )
    }
  }

  // qqq : for Bohdan : should not be here
  // qqq : for Bohdan : documentation
  #[ derive( Debug, Clone ) ]
  pub struct GitOptions
  {
    pub git_root : AbsolutePath,
    pub items : Vec< AbsolutePath >,
    pub message : String,
    pub dry : bool,
  }

  // qqq : for Bohdan : should not be here
  // qqq : for Bohdan : documentation
  fn perform_git_commit( o : GitOptions ) -> Result< ExtendedGitReport >
  {
    let mut report = ExtendedGitReport::default();
    if o.items.is_empty() { return Ok( report ); }
    let items = o
    .items
    .iter()
    .map
    (
      | item | item.as_ref().strip_prefix( o.git_root.as_ref() ).map( Path::to_string_lossy )
      .with_context( || format!("git_root: {}, item: {}", o.git_root.as_ref().display(), item.as_ref().display() ) )
    )
    .collect::< Result< Vec< _ > > >()?;
    let res = git::add( &o.git_root, &items, o.dry ).map_err( | e | format_err!( "{report}\n{e}" ) )?;
    report.add = Some( res );
    let res = git::commit( &o.git_root, &o.message, o.dry ).map_err( | e | format_err!( "{report}\n{e}" ) )?;
    report.commit = Some( res );

    Ok( report )
  }

  // qqq : for Bohdan : should not be here
  // qqq : for Bohdan : documentation
  #[ derive( Debug, Clone ) ]
  pub struct PackagePublishInstruction
  {
    pub package_name : String,
    pub pack : cargo::PackOptions,
    pub bump : version::BumpOptions,
    pub git_options : GitOptions,
    pub publish : cargo::PublishOptions,
    pub dry : bool,
  }

  // qqq : for Bohdan : should not be here
  // qqq : for Bohdan : documentation

  /// Represents a planner for publishing a single package.
  #[ derive( Debug, former::Former ) ]
  #[ perform( fn build() -> PackagePublishInstruction ) ]
  pub struct PublishSinglePackagePlanner< 'a >
  {
    workspace_dir : CrateDir,
    package : Package< 'a >,
    channel : channel::Channel,
    base_temp_dir : Option< path::PathBuf >,
    #[ former( default = true ) ]
    dry : bool,
  }

  impl< 'a > PublishSinglePackagePlanner< 'a >
  {
    fn build( self ) -> PackagePublishInstruction
    {
      let crate_dir = self.package.crate_dir();
      let workspace_root : AbsolutePath = self.workspace_dir.clone().absolute_path();
      let pack = cargo::PackOptions
      {
        path : crate_dir.clone().absolute_path().inner(),
        channel : self.channel,
        allow_dirty : self.dry,
        checking_consistency : !self.dry,
        temp_path : self.base_temp_dir.clone(),
        dry : self.dry,
      };
      let old_version : Version = self.package.version().as_ref().unwrap().try_into().unwrap();
      let new_version = old_version.clone().bump();
      // bump the package version in dependents (so far, only workspace)
      let dependencies = vec![ CrateDir::try_from( workspace_root.clone() ).unwrap() ];
      let bump = version::BumpOptions
      {
        crate_dir : crate_dir.clone(),
        old_version : old_version.clone(),
        new_version : new_version.clone(),
        dependencies : dependencies.clone(),
        dry : self.dry,
      };
      let git_options = GitOptions
      {
        git_root : workspace_root,
        items : dependencies.iter().chain([ &crate_dir ]).map( | d | d.clone().absolute_path().join( "Cargo.toml" ) ).collect(),
        message : format!( "{}-v{}", self.package.name().unwrap(), new_version ),
        dry : self.dry,
      };
      let publish = cargo::PublishOptions
      {
        path : crate_dir.clone().absolute_path().inner(),
        temp_path : self.base_temp_dir.clone(),
        retry_count : 2,
        dry : self.dry,
      };

      PackagePublishInstruction
      {
        package_name : self.package.name().unwrap().into(),
        pack,
        bump,
        git_options,
        publish,
        dry : self.dry,
      }
    }
  }

  /// Performs package publishing based on the given arguments.
  ///
  /// # Arguments
  ///
  /// * `args` - The package publishing instructions.
  ///
  /// # Returns
  ///
  /// * `Result<PublishReport>` - The result of the publishing operation, including information about the publish, version bump, and git operations.
  pub fn perform_package_publish( instruction : PackagePublishInstruction ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();
    let PackagePublishInstruction
    {
      package_name: _,
      mut pack,
      mut bump,
      mut git_options,
      mut publish,
      dry,
    } = instruction;
    pack.dry = dry;
    bump.dry = dry;
    git_options.dry = dry;
    publish.dry = dry;

    report.get_info = Some( cargo::pack( pack ).err_with( || report.clone() )? );
    // aaa : redundant field? // aaa : removed
    let bump_report = version::bump( bump ).err_with( || report.clone() )?;
    report.bump = Some( bump_report.clone() );
    let git_root = git_options.git_root.clone();
    let git = match perform_git_commit( git_options )
    {
      Ok( git ) => git,
      Err( e ) =>
      {
        version::revert( &bump_report )
        .map_err( | le | format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) ) )
        .err_with( || report.clone() )?;
        return Err(( report, e ));
      }
    };
    report.add = git.add;
    report.commit = git.commit;
    report.publish = match cargo::publish( publish )
    {
      Ok( publish ) => Some( publish ),
      Err( e ) =>
      {
        git::reset( git_root.as_ref(), true, 1, false )
        .map_err( | le |
        format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) ) )
        .err_with( || report.clone() )?;
        return Err(( report, e ));
      }
    };

    let res = git::push( &git_root, dry ).err_with( || report.clone() )?;
    report.push = Some( res );

    Ok( report )
  }

  // qqq : bad : move out to publish.rs
  // zzz : watch

  /// `PublishPlan` manages the overall publication process for multiple packages.
  /// It organizes the necessary details required for publishing each individual package.
  /// This includes the workspace root directory, any temporary directories used during the process,
  /// and the set of specific instructions for publishing each package.
  #[ derive( Debug, former::Former, Clone ) ]
  pub struct PublishPlan
  {
    /// `workspace_dir` - This is the root directory of your workspace, containing all the Rust crates
    /// that make up your package. It is used to locate the packages within your workspace that are meant
    /// to be published. The value here is represented by `CrateDir` which indicates the directory of the crate.
    pub workspace_dir : CrateDir,

    /// `base_temp_dir` - This is used for any temporary operations during the publication process, like
    /// building the package or any other processes that might require the storage of transient data. It's
    /// optional as not all operations will require temporary storage. The type used is `PathBuf` which allows
    /// manipulation of the filesystem paths.
    pub base_temp_dir : Option< path::PathBuf >,

    /// Release channels for rust.
    pub channel : channel::Channel,

    /// `dry` - A boolean value indicating whether to do a dry run. If set to `true`, the application performs
    /// a simulated run without making any actual changes. If set to `false`, the operations are actually executed.
    /// This property is optional and defaults to `true`.
    #[ former( default = true ) ]
    pub dry : bool,

    /// Required for tree view only
    pub roots : Vec< CrateDir >,

    /// `plans` - This is a vector containing the instructions for publishing each package. Each item
    /// in the `plans` vector indicates a `PackagePublishInstruction` set for a single package. It outlines
    /// how to build and where to publish the package amongst other instructions. The `#[setter( false )]`
    /// attribute indicates that there is no setter method for the `plans` variable and it can only be modified
    /// within the struct.
    #[ scalar( setter = false ) ]
    pub plans : Vec< PackagePublishInstruction >,
  }

  impl PublishPlan
  {
    /// Displays a tree-like structure of crates and their dependencies.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a `Formatter` used for writing the output.
    ///
    /// # Errors
    ///
    /// Returns a `std::fmt::Error` if there is an error writing to the formatter.
    pub fn write_as_tree< W >( &self, f : &mut W ) -> std::fmt::Result
    where
      W : std::fmt::Write
    {
      let name_bump_report : HashMap< _, _ > = self
      .plans
      .iter()
      .map( | x | ( &x.package_name, ( x.bump.old_version.to_string(), x.bump.new_version.to_string() ) ) )
      .collect();
      for wanted in &self.roots
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

        fn callback( name_bump_report : &HashMap< &String, ( String, String ) >, mut r : tool::ListNodeReport ) -> tool::ListNodeReport
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
        let printer = list;
        let rep : Vec< tool::ListNodeReport > = printer.iter().map( | printer | printer.info.clone() ).collect();
        let list: Vec< tool::ListNodeReport > = rep.into_iter().map( | r | callback( &name_bump_report, r ) ).collect();
        let printer : Vec< tool::TreePrinter > = list.iter().map( | rep | tool::TreePrinter::new( rep ) ).collect();

        let list = action::list::ListReport::Tree( printer );
        writeln!( f, "{}", list )?;
      }

      Ok( () )
    }

    /// Format and display the list of packages and their version bumps in a formatted way.
    ///
    /// # Arguments
    ///
    /// - `f`: A mutable reference to a `Formatter` where the output will be written to.
    ///
    /// # Errors
    ///
    /// Returns a `std::fmt::Error` if there is an error writing to the formatter.
    pub fn write_as_list< W >( &self, f : &mut W ) -> std::fmt::Result
    where
      W : std::fmt::Write
    {
      for ( idx, package ) in self.plans.iter().enumerate()
      {
        let bump = &package.bump;
        writeln!( f, "[{idx}] {} ({} -> {})", package.package_name, bump.old_version, bump.new_version )?;
      }

      Ok( () )
    }
  }

  impl< 'a > PublishPlanFormer
  {
    pub fn option_base_temp_dir( mut self, path : Option< path::PathBuf > ) -> Self
    {
      self.storage.base_temp_dir = path;
      self
    }

    pub fn package< IntoPackage >( mut self, package : IntoPackage ) -> Self
    where
      IntoPackage : Into< Package< 'a > >,
    {
      let channel = self.storage.channel.unwrap_or_default();
      let mut plan = PublishSinglePackagePlanner::former();
      if let Some( workspace ) = &self.storage.workspace_dir
      {
        plan = plan.workspace_dir( workspace.clone() );
      }
      if let Some( base_temp_dir ) = &self.storage.base_temp_dir
      {
        plan = plan.base_temp_dir( base_temp_dir.clone() );
      }
      if let Some( dry ) = self.storage.dry
      {
        plan = plan.dry( dry );
      }
      let plan = plan
      .channel( channel )
      .package( package )
      .perform();
      let mut plans = self.storage.plans.unwrap_or_default();
      plans.push( plan );

      self.storage.plans = Some( plans );

      self
    }

    pub fn packages< IntoPackageIter, IntoPackage >( mut self, packages : IntoPackageIter ) -> Self
    where
      IntoPackageIter : IntoIterator< Item = IntoPackage >,
      IntoPackage : Into< Package< 'a > >,
    {
      for package in packages
      {
        self = self.package( package );
      }

      self
    }

  }

  /// Perform publishing of multiple packages based on the provided publish plan.
  ///
  /// # Arguments
  ///
  /// * `plan` - The publish plan with details of packages to be published.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a vector of `PublishReport` if successful, else an error.
  pub fn perform_packages_publish( plan : PublishPlan ) -> Result< Vec< PublishReport > >
  {
    let mut report = vec![];
    for package in plan.plans
    {
      let res = perform_package_publish( package ).map_err( |( current_rep, e )| format_err!( "{}\n{current_rep}\n{e}", report.iter().map( | r | format!( "{r}" ) ).join( "\n" ) ) )?;
      report.push( res );
    }

    Ok( report )
  }

  // qqq : bad : for Bohdan : publish is not part of entity package

  /// Holds information about the publishing process.
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    /// Retrieves information about the package.
    pub get_info : Option< process::Report >,
    /// Bumps the version of the package.
    pub bump : Option< version::ExtendedBumpReport >,
    /// Report of adding changes to the Git repository.
    pub add : Option< process::Report >,
    /// Report of committing changes to the Git repository.
    pub commit : Option< process::Report >,
    /// Report of pushing changes to the Git repository.
    pub push : Option< process::Report >,
    /// Report of publishes the package using the `cargo publish` command.
    pub publish : Option< process::Report >,
  }

  impl std::fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      let PublishReport
      {
        get_info,
        bump,
        add,
        commit,
        push,
        publish,
      } = self;

      if get_info.is_none()
      {
        f.write_str( "Empty report" )?;
        return Ok( () )
      }
      let info = get_info.as_ref().unwrap();
      write!( f, "{}", info )?;

      if let Some( bump ) = bump
      {
        writeln!( f, "{}", bump )?;
      }
      if let Some( add ) = add
      {
        write!( f, "{add}" )?;
      }
      if let Some( commit ) = commit
      {
        write!( f, "{commit}" )?;
      }
      if let Some( push ) = push
      {
        write!( f, "{push}" )?;
      }
      if let Some( publish ) = publish
      {
        write!( f, "{publish}" )?;
      }

      Ok( () )
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

  //

  /// Identifier of any crate (local and remote).
  #[ derive( Debug, Clone, Hash, Eq, PartialEq ) ]
  pub struct CrateId
  {
    /// The name of the crate.
    pub name : String,
    /// The absolute path to the crate, if available.
    pub crate_dir : Option< CrateDir >,
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
        name : value.name().into(),
        crate_dir : value.crate_dir(),
        // path : value.path().clone().map( | path | AbsolutePath::try_from( path ).unwrap() ),
      }
    }
  }

  // qqq : for Bohdan : move out
  /// Recursive implementation of the `dependencies` function
  pub fn _dependencies< 'a >
  (
    workspace : &Workspace, // aaa : for Bohdan : no mut // aaa : no mut
    package : &Package< 'a >,
    graph : &mut HashMap< CrateId, HashSet< CrateId > >,
    opts : DependenciesOptions
  ) -> Result< CrateId >
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
    .package_find_by_manifest( &manifest_file )
    .ok_or( format_err!( "Package not found in the workspace with path : `{}`", manifest_file.as_ref().display() ) )?;

    let deps : HashSet< _ > = package
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
          _dependencies
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
  pub fn dependencies< 'a >
  (
    workspace : &mut Workspace,
    package : &Package< 'a >,
    opts : DependenciesOptions
  )
  -> Result< Vec< CrateId > >
  {
    let mut graph = HashMap::new();
    let root = _dependencies( workspace, package, &mut graph, opts.clone() )?;

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
        graph::toposort( graph::construct( &graph ) ).map_err( | err | format_err!( "{}", err ) )?.into_iter().filter( | x | x != &root ).collect()
      },
    };

    Ok( output )
  }

  //

  /// Determines whether a package needs to be published by comparing `.crate` files from the local and remote package.
  ///
  /// This function requires the local package to be previously packed.
  ///
  /// # Returns :
  /// - `true` if the package needs to be published.
  /// - `false` if there is no need to publish the package.
  ///
  /// Panics if the package is not loaded or local package is not packed.

  pub fn publish_need< 'a >( package : &Package< 'a >, path : Option< path::PathBuf > ) -> Result< bool, PackageError >
  {
    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = path
    .map( | p | p.join( format!( "package/{0}-{1}.crate", name, version ) ) )
    .unwrap_or( packed_crate::local_path( &name, &version, package.crate_dir() ).map_err( | _ | PackageError::LocalPath )? );

    let local_package = CrateArchive::read( local_package_path ).map_err( | _ | PackageError::ReadArchive )?;
    let remote_package = match CrateArchive::download_crates_io( name, version )
    {
      Ok( archive ) => archive,
      // qqq : fix. we don't have to know about the http status code
      Err( ureq::Error::Status( 403, _ ) ) => return Ok( true ),
      _ => return Err( PackageError::LoadRemotePackage ),
    };

    Ok( diff::crate_diff( &local_package, &remote_package ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes() )
  }
}

//

crate::mod_interface!
{

  protected use PublishSinglePackagePlanner;
  protected use PublishPlan;
  protected use perform_package_publish;
  protected use perform_packages_publish;

  protected use PublishReport;
  protected use Package;
  protected use PackageName;
  protected use PackageError;

  protected use publish_need;

  protected use CrateId;
  protected use DependenciesSort;
  protected use DependenciesOptions;
  protected use dependencies;

}
