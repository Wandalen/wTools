mod private
{
  use crate::*;

  // use std::collections::BTreeMap;
  use std::path::Path;
  // use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };
  use petgraph::Graph;
  // use serde::Deserialize;
  // use serde_json::Value;
  use wtools::error::
  {
    for_app::Context,
    for_lib::Error,
    Result
  };
  // use path::AbsolutePath;

  // qqq : for Bohdan : for Petro : what crate_dir is?

  /// Stores information about the current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    // qqq : for Bohdan : for Petro : describe all fields
    // qqq : for Bohdan : for Petro : is Option required?
    /// Metadata of the workspace, containing detailed information about the packages, dependencies, and other workspace-related data.
    /// This field is optional and may be `None` if the metadata has not been loaded yet.
    pub metadata : Option< cargo_metadata::Metadata >,

    /// The directory containing the manifest file (`Cargo.toml`) of the workspace.
    pub crate_dir : CrateDir,
  }

  /// Represents errors related to workspace operations.
  #[ derive( Debug, Error ) ]
  pub enum WorkspaceError
  {
    /// Metadata is non.
    #[ error( "Metadata is non " ) ]
    MetadataError,
  }

  impl Workspace
  {

    // qqq : typed errors
    /// Load data from current directory
    pub fn from_current_path() -> Result< Self >
    {
      let current_path = AbsolutePath::try_from( std::env::current_dir().unwrap_or_default() )?;
      let metadata = cargo_metadata::MetadataCommand::new()
      .no_deps()
      .exec()
      .context( "fail to load CargoMetadata" )?;
      Ok( Self
      {
        metadata : Some( metadata ),
        crate_dir : CrateDir::try_from( current_path )?,
      })
    }

    // qqq : typed errors
    /// Load data from current directory
    pub fn with_crate_dir( crate_dir : CrateDir ) -> Result< Self >
    {
      Ok
      (
        Self
        {
          metadata : Some
          (
            cargo_metadata::MetadataCommand::new()
            .current_dir( crate_dir.as_ref() )
            .no_deps()
            .exec()
            .context( "fail to load CargoMetadata" )?
          ),
          crate_dir : crate_dir,
        }
      )
    }
  }

  impl From< cargo_metadata::Metadata > for Workspace
  {
    fn from( value : cargo_metadata::Metadata ) -> Self
    {
      let path = value.workspace_root.as_std_path().parent().unwrap().to_path_buf();
      let path = AbsolutePath::try_from( path ).unwrap();
      Self
      {
        metadata : Some( value ),
        crate_dir : CrateDir::try_from( path ).unwrap(),
      }
    }
  }

  impl Workspace
  {

    /// Load data from the current location or from cache
    // qqq : Maybe unsafe. Take metadata of workspace in current dir.
    #[ inline( always ) ]
    pub fn load( &mut self ) -> Result< () >
    {
      if self.metadata.is_none()
      {
        let metadata = Self::with_crate_dir( self.crate_dir.clone() )?.metadata.unwrap();
        _ = self.metadata.insert( metadata );
      }
      Ok( () )
    }

    /// Force loads data from the current location
    // qqq : Maybe unsafe. Take metadata of workspace in current dir.
    // qqq : for Petro : for Bohdan : why is it necessary?
    #[ inline( always ) ]
    pub fn force_reload( &mut self ) -> Result< () >
    {
      let metadata = Self::with_crate_dir( self.crate_dir.clone() )?.metadata.unwrap();
      _ = self.metadata.insert( metadata );
      Ok( () )
    }

  }

  impl Workspace
  {

    // qqq : replace all Vec by Iterators over refs

    /// Returns list of all packages
    pub fn packages< 'a >( &'a self )
    ->
    std::result::Result
    <
      core::iter::Map
      <
        std::slice::Iter< 'a, cargo_metadata::Package >,
        impl Fn( &'a cargo_metadata::Package ) -> WorkspacePackageRef< 'a > + Clone,
      >,
      WorkspaceError,
    >
    {
      self
      .metadata
      .as_ref()
      .ok_or_else( || WorkspaceError::MetadataError )
      .map( move | p | p.packages.iter().map( WorkspacePackageRef::from ) )
    }

    // /// Returns list of all packages
    // pub fn packages( &self ) -> Result< Vec< WorkspacePackageRef< '_ > >, WorkspaceError >
    // {
    //   self
    //   .metadata
    //   .as_ref()
    //   .ok_or_else( || WorkspaceError::MetadataError )
    //   .map( | metadata | metadata.packages.clone() )
    //   .map( | p | p.into_iter().map( WorkspacePackageRef::from ).collect() )
    // }

    /// Returns the path to workspace root
    pub fn workspace_root( &self ) -> Result< &Path, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.workspace_root.as_std_path() )
    }

    /// Returns the path to target directory
    pub fn target_directory( &self ) -> Result< &Path, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.target_directory.as_std_path() )
    }

    // aaa : bad : for Petro : that should not be here as it's very task specific
    // aaa : done
    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< 'a, P >( &'a self, manifest_file : P ) -> Option< WorkspacePackageRef< 'a > >
    where
      P : AsRef< Path >,
    {
      self
      .packages()
      .ok()
      .and_then
      (
        move | mut packages |
        packages
        // .iter()
        // .find( | &p | p.manifest_file().as_std_path() == manifest_file.as_ref() )
        .find( | &p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref() )
        // .cloned()
      )
    }

    // xxx : qqq : for Bohdan : should not be here entity/workspace-graph.rs

    /// Returns a graph of packages.
    pub( crate ) fn graph( &self ) -> Graph< String, String >
    {
      let packages = self.packages().unwrap();
      let module_package_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ > ) -> bool > > = Some
      (
        Box::new( move | p | p.publish().is_none() )
      );
      let module_dependency_filter : Option< Box< dyn Fn( WorkspacePackageRef< '_ >, DependencyRef< '_ > ) -> bool > > = Some
      (
        Box::new
        (
          move | _, d | d.crate_dir().is_some() && d.kind() != DependencyKind::Development
        )
      );
      let module_packages_map = packages::filter
      (
        // packages.as_slice(),
        packages,
        packages::FilterMapOptions { package_filter : module_package_filter, dependency_filter : module_dependency_filter },
      );

      graph::construct( &module_packages_map ).map( | _, x | x.to_string(), | _, x | x.to_string() )
    }
  }
}

//

crate::mod_interface!
{
  exposed use WorkspaceError;
  exposed use Workspace;
}
