mod private
{
  use crate::*;

  use std::fmt;
  use process_tools::process;
  use
  {
    iter::Itertools,
    error::
    {
      // Result,
      untyped::{ format_err, Error },
    }
  };
  use error::ErrWith;

  /// Represents instructions for publishing a package.
  #[ derive( Debug, Clone ) ]
  pub struct PackagePublishInstruction
  {
    /// The name of the package.
    pub package_name : package::PackageName,
    /// Options for packing the package using Cargo.
    pub pack : cargo::PackOptions,
    /// Options for bumping the package version.
    pub bump : version::BumpOptions,
    /// Git options related to the package.
    pub git_options : Option< entity::git::GitOptions >,
    /// Options for publishing the package using Cargo.
    pub publish : cargo::PublishOptions,
    /// Indicates whether the process should be dry-run (no actual publishing).
    pub dry : bool,
  }

  /// Represents a planner for publishing a single package.
  #[ derive( Debug, former::Former ) ]
  #[ perform( fn build() -> PackagePublishInstruction ) ]
  pub struct PublishSinglePackagePlanner< 'a >
  {
    workspace_dir : CrateDir,
    package : package::Package< 'a >,
    channel : channel::Channel,
    base_temp_dir : Option< path::PathBuf >,
    exclude_dev_dependencies : bool,
    #[ former( default = true ) ]
    commit_changes : bool,
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
        exclude_dev_dependencies : self.exclude_dev_dependencies,
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
      let git_options = if self.commit_changes
      {
        Some( entity::git::GitOptions
        {
          git_root : workspace_root,
          items : dependencies.iter().chain([ &crate_dir ]).map( | d | d.clone().absolute_path().join( "Cargo.toml" ) ).collect(),
          message : format!( "{}-v{}", self.package.name().unwrap(), new_version ),
          dry : self.dry,
        })
      } else { None };
      let publish = cargo::PublishOptions
      {
        path : crate_dir.clone().absolute_path().inner(),
        temp_path : self.base_temp_dir.clone(),
        exclude_dev_dependencies : self.exclude_dev_dependencies,
        retry_count : 2,
        dry : self.dry,
      };

      PackagePublishInstruction
      {
        package_name : self.package.name().unwrap().to_string().into(),
        pack,
        bump,
        git_options,
        publish,
        dry : self.dry,
      }
    }
  }

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

    /// Setting this option to true will temporarily remove development dependencies before executing the command, then restore them afterward.
    #[ allow( dead_code ) ] // former related
    pub exclude_dev_dependencies : bool,

    /// Indicates whether changes should be committed.
    #[ former( default = true ) ]
    pub commit_changes : bool,

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
    pub fn write_as_tree< W >( &self, f : &mut W ) -> fmt::Result
    where
      W : fmt::Write
    {
      let name_bump_report : collection::HashMap< _, _ > = self
      .plans
      .iter()
      .map( | x | ( x.package_name.as_ref(), ( x.bump.old_version.to_string(), x.bump.new_version.to_string() ) ) )
      .collect();
      for wanted in &self.roots
      {
        let list = action::list_all
        (
          action::list::ListOptions::former()
          .path_to_manifest( wanted.clone() )
          .format( action::list::ListFormat::Tree )
          .dependency_sources([ action::list::DependencySource::Local ])
          .dependency_categories([ action::list::DependencyCategory::Primary ])
          .form()
        )
        .map_err( |( _, _e )| fmt::Error )?;
        let action::list::ListReport::Tree( list ) = list else { unreachable!() };

        fn callback( name_bump_report : &collection::HashMap< &String, ( String, String ) >, mut r : tool::ListNodeReport ) -> tool::ListNodeReport
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
    pub fn write_as_list< W >( &self, f : &mut W ) -> fmt::Result
    where
      W : fmt::Write
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
      IntoPackage : Into< package::Package< 'a > >,
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
      if let Some( exclude_dev_dependencies ) = &self.storage.exclude_dev_dependencies
      {
        plan = plan.exclude_dev_dependencies( *exclude_dev_dependencies )
      }
      if let Some( commit_changes ) = &self.storage.commit_changes
      {
        plan = plan.commit_changes( *commit_changes )
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
      IntoPackage : Into< package::Package< 'a > >,
    {
      for package in packages
      {
        self = self.package( package );
      }

      self
    }

  }

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

  impl fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
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

  /// Performs package publishing based on the given arguments.
  ///
  /// # Arguments
  ///
  /// * `args` - The package publishing instructions.
  ///
  /// # Returns
  ///
  /// * `Result<PublishReport>` - The result of the publishing operation, including information about the publish, version bump, and git operations.

  pub fn perform_package_publish( instruction : PackagePublishInstruction ) -> ResultWithReport< PublishReport, Error >
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
    git_options.as_mut().map( | d | d.dry = dry );
    publish.dry = dry;

    report.get_info = Some( cargo::pack( pack ).err_with_report( &report )? );
    // aaa : redundant field? // aaa : removed
    let bump_report = version::bump( bump ).err_with_report( &report )?;
    report.bump = Some( bump_report.clone() );

    let git_root = git_options.as_ref().map( | g | g.git_root.clone() );
    if let Some( git_options ) = git_options
    {
      let git = match entity::git::perform_git_commit( git_options )
      {
        Ok( git ) => git,
        Err( e ) =>
        {
          version::revert( &bump_report )
          .map_err( | le | format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) ) )
          .err_with_report( &report )?;
          return Err(( report, e ));
        }
      };
      report.add = git.add;
      report.commit = git.commit;
    }
    report.publish = match cargo::publish( publish )
    {
      Ok( publish ) => Some( publish ),
      Err( e ) =>
      {
        if let Some( git_root ) = git_root.as_ref()
        {
          tool::git::reset( git_root.as_ref(), true, 1, false )
          .map_err
          (
            | le |
            format_err!( "Base error:\n{}\nRevert error:\n{}", e.to_string().replace( '\n', "\n\t" ), le.to_string().replace( '\n', "\n\t" ) )
          )
          .err_with_report( &report )?;
        }
        return Err(( report, e ));
      }
    };

    if let Some( git_root ) = git_root.as_ref()
    {
      let res = tool::git::push( &git_root, dry ).err_with_report( &report )?;
      report.push = Some( res );
    }

    Ok( report )
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
  pub fn perform_packages_publish( plan : PublishPlan ) -> error::untyped::Result< Vec< PublishReport > >
  // qqq : use typed error
  {
    let mut report = vec![];
    for package in plan.plans
    {
      let res = perform_package_publish( package ).map_err( |( current_rep, e )| format_err!( "{}\n{current_rep}\n{e}", report.iter().map( | r | format!( "{r}" ) ).join( "\n" ) ) )?;
      report.push( res );
    }

    Ok( report )
  }

}

//

crate::mod_interface!
{
  own use PublishPlan;
  own use PackagePublishInstruction;
  own use PublishSinglePackagePlanner;
  own use PublishReport;
  own use perform_package_publish;
  own use perform_packages_publish;
}
