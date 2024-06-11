mod private
{
  use crate::*;

  use std::
  {
    path::Path,
    collections::{ HashMap, HashSet },
  };
  use std::fmt::Formatter;
  use std::hash::Hash;
  use std::path::PathBuf;

  use process_tools::process;
  use manifest::{ Manifest, ManifestError };
  use crates_tools::CrateArchive;

  use workspace::Workspace;
  use _path::AbsolutePath;

  use wtools::
  {
    iter::Itertools,
    error::
    {
      thiserror,
      Result,
      for_lib::Error,
      for_app::{ format_err, Context },
    }
  };
  use action::readme_health_table_renew::Stability;
  use former::Former;
  use workspace::WorkspacePackage;
  use diff::crate_diff;
  use version::version_revert;
  use error_tools::for_app::Error;
  use channel::Channel;

  ///
  #[ derive( Debug, Clone ) ]
  pub enum Package
  {
    /// `Cargo.toml` file.
    Manifest( Manifest ),
    /// Cargo metadata package.
    Metadata( WorkspacePackage ),
  }

  /// Represents errors related to package handling.
  #[ derive( Debug, Error ) ]
  pub enum PackageError
  {
    /// Manifest error.
    #[ error( "Manifest error. Reason : {0}." ) ]
    Manifest( #[ from ] ManifestError ),
    /// Fail to load metadata.
    #[ error( "Fail to load metadata." ) ]
    Metadata,
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

  impl TryFrom< AbsolutePath > for Package
  {
    // aaa : make better errors
    // aaa : return `PackageError` instead of `anohow` message
    type Error = PackageError;

    fn try_from( value : AbsolutePath ) -> Result< Self, Self::Error >
    {
      let manifest =  manifest::open( value.clone() )?;
      if !manifest.package_is()?
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( manifest ) )
    }
  }

  impl TryFrom< CrateDir > for Package
  {
    type Error = PackageError;

    fn try_from( value : CrateDir ) -> Result< Self, Self::Error >
    {
      let manifest =  manifest::open( value.absolute_path().join( "Cargo.toml" ) )?;
      if !manifest.package_is()?
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( manifest ) )
    }
  }

  impl TryFrom< Manifest > for Package
  {
    // aaa : make better errors
    // aaa : return `PackageError` instead of `anohow` message
    type Error = PackageError;

    fn try_from( value : Manifest ) -> Result< Self, Self::Error >
    {
      if !value.package_is()?
      {
        return Err( PackageError::NotAPackage );
      }

      Ok( Self::Manifest( value ) )
    }
  }

  impl From< WorkspacePackage > for Package
  {
    fn from( value : WorkspacePackage ) -> Self
    {
      Self::Metadata( value )
    }
  }

  impl Package
  {
    /// Path to `Cargo.toml`
    pub fn manifest_path( &self ) -> AbsolutePath
    {
      match self
      {
        Self::Manifest( manifest ) => manifest.manifest_path.clone(),
        Self::Metadata( metadata ) => AbsolutePath::try_from( metadata.manifest_path().as_std_path().to_path_buf() ).unwrap(),
      }
    }

    /// Path to folder with `Cargo.toml`
    pub fn crate_dir( &self ) -> CrateDir
    {
      match self
      {
        Self::Manifest( manifest ) => manifest.crate_dir(),
        Self::Metadata( metadata ) =>
        {
          let path = metadata.manifest_path().parent().unwrap().as_std_path().to_path_buf();
          let absolute = AbsolutePath::try_from( path ).unwrap();

          CrateDir::try_from( absolute ).unwrap()
        },
      }
    }

    /// Package name
    pub fn name( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          // Unwrap safely because of the `Package` type guarantee
          Ok( data[ "package" ][ "name" ].as_str().unwrap().to_string() )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.name().clone() )
        }
      }
    }

    /// Package version
    pub fn version( &self ) -> Result< String, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          // Unwrap safely because of the `Package` type guarantee
          Ok( data[ "package" ][ "version" ].as_str().unwrap().to_string() )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.version().to_string() )
        }
      }
    }

    /// Stability
    pub fn stability( &self ) -> Result< Stability, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          // Unwrap safely because of the `Package` type guarantee
          Ok( data[ "package" ].get( "metadata" ).and_then( | m | m.get( "stability" ) ).and_then( | s | s.as_str() ).and_then( | s | s.parse::< Stability >().ok() ).unwrap_or( Stability::Experimental)  )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.metadata()[ "stability" ].as_str().and_then( | s | s.parse::< Stability >().ok() ).unwrap_or( Stability::Experimental) )
        }
      }
    }

    /// Repository
    pub fn repository( &self ) -> Result< Option< String >, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          // Unwrap safely because of the `Package` type guarantee
          Ok( data[ "package" ].get( "repository" ).and_then( | r | r.as_str() ).map( | r | r.to_string()) )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.repository().cloned() )
        }
      }
    }

    /// Discord url
    pub fn discord_url( &self ) -> Result< Option< String >, PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          let data = manifest.manifest_data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;

          Ok( data[ "package" ].get( "metadata" ).and_then( | m | m.get( "discord_url" ) ).and_then( | url | url.as_str() ).map( | r | r.to_string() ) )
        }
        Self::Metadata( metadata ) =>
        {
          Ok( metadata.metadata()[ "discord_url" ].as_str().map( | url | url.to_string() ) )
        }
      }
    }

    /// Check that module is local.
    pub fn local_is( &self ) -> Result< bool, ManifestError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          // verify that manifest not empty
          manifest.local_is()
        }
        Self::Metadata( metadata ) =>
        {
          Ok( !( metadata.publish().is_none() || metadata.publish().as_ref().is_some_and( | p | p.is_empty() ) ) )
        }
      }
    }

    /// Returns the `Manifest`
    pub fn manifest( &self ) -> Result< Manifest, PackageError >
    {
      match self
      {
        Package::Manifest( manifest ) => Ok( manifest.clone() ),
        Package::Metadata( metadata ) => manifest::open
        (
          AbsolutePath::try_from( metadata.manifest_path() ).map_err( | _ | PackageError::LocalPath )?
        )
        .map_err( | _ | PackageError::Metadata ),
      }
    }

    /// Returns the `Metadata`
    pub fn metadata( &self ) -> Result< WorkspacePackage, PackageError >
    {
      match self
      {
        Package::Manifest( manifest ) =>
        Workspace::with_crate_dir( manifest.crate_dir() ).map_err( | _ | PackageError::Metadata )?
        .package_find_by_manifest( &manifest.manifest_path )
        .ok_or_else( || PackageError::Metadata ),
        Package::Metadata( metadata ) => Ok( metadata.clone() ),
      }
    }
  }

  #[ derive( Debug, Default, Clone ) ]
  pub struct ExtendedGitReport
  {
    pub add : Option< process::Report >,
    pub commit : Option< process::Report >,
    pub push : Option< process::Report >,
  }

  impl std::fmt::Display for ExtendedGitReport
  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      let Self { add, commit, push } = &self;
      if let Some( add ) = add { writeln!( f, "{add}" )? }
      if let Some( commit ) = commit { writeln!( f, "{commit}" )? }
      if let Some( push ) = push { writeln!( f, "{push}" )? }

      Ok( () )
    }
  }

  #[ derive( Debug, Clone ) ]
  pub struct GitOptions
  {
    pub git_root : AbsolutePath,
    pub items : Vec< AbsolutePath >,
    pub message : String,
    pub dry : bool,
  }

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

  #[ derive( Debug, Clone ) ]
  pub struct PackagePublishInstruction
  {
    pub package_name : String,
    pub pack : cargo::PackOptions,
    pub version_bump : version::BumpOptions,
    pub git_options : GitOptions,
    pub publish : cargo::PublishOptions,
    pub dry : bool,
  }

  /// Represents a planner for publishing a single package.
  #[ derive( Debug, Former ) ]
  #[ perform( fn build() -> PackagePublishInstruction ) ]
  pub struct PublishSinglePackagePlanner
  {
    workspace_dir : CrateDir,
    package : Package,
    channel : Channel,
    base_temp_dir : Option< PathBuf >,
    #[ former( default = true ) ]
    dry : bool,
  }

  impl PublishSinglePackagePlanner
  {
    fn build( self ) -> PackagePublishInstruction
    {
      let crate_dir = self.package.crate_dir();
      let workspace_root : AbsolutePath = self.workspace_dir.absolute_path();
      let pack = cargo::PackOptions
      {
        path : crate_dir.as_ref().into(),
        channel : self.channel,
        allow_dirty : self.dry,
        no_verify : self.dry,
        temp_path : self.base_temp_dir.clone(),
        dry : self.dry,
      };
      let old_version : version::Version = self.package.version().as_ref().unwrap().try_into().unwrap();
      let new_version = old_version.clone().bump();
      // bump the package version in dependents (so far, only workspace)
      let dependencies = vec![ CrateDir::try_from( workspace_root.clone() ).unwrap() ];
      let version_bump = version::BumpOptions
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
        items : dependencies.iter().chain([ &crate_dir ]).map( | d | d.absolute_path().join( "Cargo.toml" ) ).collect(),
        message : format!( "{}-v{}", self.package.name().unwrap(), new_version ),
        dry : self.dry,
      };
      let publish = cargo::PublishOptions
      {
        path : crate_dir.as_ref().into(),
        temp_path : self.base_temp_dir.clone(),
        retry_count : 2,
        dry : self.dry,
      };

      PackagePublishInstruction
      {
        package_name : self.package.name().unwrap(),
        pack,
        version_bump,
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
      mut version_bump,
      mut git_options,
      mut publish,
      dry,
    } = instruction;
    pack.dry = dry;
    version_bump.dry = dry;
    git_options.dry = dry;
    publish.dry = dry;

    report.get_info = Some( cargo::pack( pack ).map_err( | e | ( report.clone(), e ) )? );
    // qqq : redundant field?
    report.publish_required = true;
    let bump_report = version::version_bump( version_bump ).map_err( | e | ( report.clone(), e ) )?;
    report.bump = Some( bump_report.clone() );
    let git_root = git_options.git_root.clone();
    let git = match perform_git_commit( git_options )
    {
      Ok( git ) => git,
      Err( e ) =>
      {
        version_revert( &bump_report )
        .map_err( | le |
        (
          report.clone(),
          format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) )
        ))?;
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
        (
          report.clone(),
          format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) )
        ))?;
        return Err(( report, e ));
      }
    };

    let res = git::push( &git_root, dry ).map_err( | e | ( report.clone(), e ) )?;
    report.push = Some( res );

    Ok( report )
  }

  /// `PublishPlan` manages the overall publication process for multiple packages.
  /// It organizes the necessary details required for publishing each individual package.
  /// This includes the workspace root directory, any temporary directories used during the process,
  /// and the set of specific instructions for publishing each package.
  #[ derive( Debug, Former, Clone ) ]
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
    pub base_temp_dir : Option< PathBuf >,
    
    /// Release channels for rust.
    pub channel : Channel,

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
      let name_bump_report = self
      .plans
      .iter()
      .map( | x | ( &x.package_name, ( x.version_bump.old_version.to_string(), x.version_bump.new_version.to_string() ) ) )
      .collect::< HashMap< _, _ > >();
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

        fn callback( name_bump_report : &HashMap< &String, ( String, String ) >, mut r : action::list::ListNodeReport ) -> action::list::ListNodeReport
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
        let bump = &package.version_bump;
        writeln!( f, "[{idx}] {} ({} -> {})", package.package_name, bump.old_version, bump.new_version )?;
      }

      Ok( () )
    }
  }

  impl PublishPlanFormer
  {
    pub fn option_base_temp_dir( mut self, path : Option< PathBuf > ) -> Self
    {
      self.storage.base_temp_dir = path;
      self
    }

    pub fn package< IntoPackage >( mut self, package : IntoPackage ) -> Self
    where
      IntoPackage : Into< Package >,
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
      IntoPackage : Into< Package >,
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

  /// Holds information about the publishing process.
  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    /// Retrieves information about the package.
    pub get_info : Option< process::Report >,
    /// Indicates whether publishing is required for the package.
    pub publish_required : bool,
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
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      let PublishReport
      {
        get_info,
        publish_required,
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

      if !publish_required
      {
        f.write_str( "The package has no changes, so no publishing is required" )?;
        return Ok( () )
      }

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

  /// Identifier of any crate(local and remote)
  #[ derive( Debug, Clone, Hash, Eq, PartialEq ) ]
  pub struct CrateId
  {
    /// TODO : make it private
    pub name : String,
    /// TODO : make it private
    pub path : Option< AbsolutePath >,
  }

  impl From< &WorkspacePackage > for CrateId
  {
    fn from( value : &WorkspacePackage ) -> Self
    {
      Self
      {
        name : value.name().clone(),
        path : Some( AbsolutePath::try_from( value.manifest_path().parent().unwrap() ).unwrap() ),
      }
    }
  }

  impl From< &workspace::Dependency > for CrateId
  {
    fn from( value : &workspace::Dependency ) -> Self
    {
      Self
      {
        name : value.name().clone(),
        path : value.path().clone().map( | path | AbsolutePath::try_from( path ).unwrap() ),
      }
    }
  }

  /// Recursive implementation of the `dependencies` function
  pub fn _dependencies
  (
    workspace : &mut Workspace,
    manifest : &Package,
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

    let manifest_path = &manifest.manifest_path();

    let package = workspace
    .load()?
    .package_find_by_manifest( &manifest_path )
    .ok_or( format_err!( "Package not found in the workspace with path : `{}`", manifest_path.as_ref().display() ) )?;

    let deps = package
    .dependencies()
    .iter()
    .filter( | dep | ( with_remote || dep.path().is_some() ) && ( with_dev || dep.kind() != workspace::DependencyKind::Development ) )
    .map( CrateId::from )
    .collect::< HashSet< _ > >();

    let package = CrateId::from( &package );
    graph.insert( package.clone(), deps.clone() );

    if recursive
    {
      for dep in deps
      {
        if graph.get( &dep ).is_none()
        {
          // unwrap because `recursive` + `with_remote` not yet implemented
          _dependencies( workspace, &dep.path.as_ref().unwrap().join( "Cargo.toml" ).try_into().unwrap(), graph, opts.clone() )?;
        }
      }
    }

    Ok( package )
  }

  /// Returns local dependencies of a specified package by its manifest path from a workspace.
  ///
  /// # Arguments
  ///
  /// - `workspace` - holds cached information about the workspace, such as the packages it contains and their dependencies. By passing it as a mutable reference, function can update the cache as needed.
  /// - `manifest` - The package manifest file contains metadata about the package such as its name, version, and dependencies.
  /// - `opts` - used to specify options or configurations for fetching local dependencies.
  ///
  /// # Returns
  ///
  /// If the operation is successful, returns a vector of `PathBuf` objects, where each `PathBuf` represents the path to a local dependency of the specified package.
  pub fn dependencies( workspace : &mut Workspace, manifest : &Package, opts : DependenciesOptions ) -> Result< Vec< CrateId > >
  {
    let mut graph = HashMap::new();
    let root = _dependencies( workspace, manifest, &mut graph, opts.clone() )?;

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
  /// Panics if the manifest is not loaded or local package is not packed.

  pub fn publish_need( package : &Package, path : Option< PathBuf > ) -> Result< bool, PackageError >
  {
    let name = package.name()?;
    let version = package.version()?;
    let local_package_path = path
    .map( | p | p.join( format!( "package/{0}-{1}.crate", name, version ) ) )
    .unwrap_or( packed_crate::local_path( &name, &version, package.crate_dir() ).map_err( | _ | PackageError::LocalPath )? );

    // aaa : for Bohdan : bad, properly handle errors
    // aaa : return result instead of panic
    let local_package = CrateArchive::read( local_package_path ).map_err( | _ | PackageError::ReadArchive )?;
    let remote_package = match CrateArchive::download_crates_io( name, version )
    {
      Ok( archive ) => archive,
      // qqq : fix. we don't have to know about the http status code
      Err( ureq::Error::Status( 403, _ ) ) => return Ok( true ),
      _ => return Err( PackageError::LoadRemotePackage ),
    };

    Ok( crate_diff( &local_package, &remote_package ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes() )
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
  protected use PackageError;

  protected use publish_need;

  protected use CrateId;
  protected use DependenciesSort;
  protected use DependenciesOptions;
  protected use dependencies;

}
