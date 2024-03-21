mod private
{
  use std::collections::BTreeMap;
  use crate::*;

  use std::path::Path;
  use cargo_metadata::{Dependency, Metadata, MetadataCommand, Package};
  use cargo_metadata::camino::Utf8Path;
  use petgraph::Graph;
  use semver::Version;
  use serde_json::Value;
  use wtools::error::{ for_app::Context, for_lib::Error, Result };
  use _path::AbsolutePath;

  #[ derive( Debug, Clone ) ]
  pub struct WorkspacePackage
  {
    inner : Package
  }
  
  impl From< Package > for WorkspacePackage
  {
    fn from( inner : Package) -> Self 
    {
      Self
      {
        inner
      }
    }
  }
  
  impl WorkspacePackage
  {
    pub fn name( &self ) -> &String
    {
      &self.inner.name
    }
    
    pub fn dependencies( &self ) -> &[ Dependency ]
    {
      self.inner.dependencies.as_slice()
    }
    
    pub fn manifest_path( &self ) -> &Utf8Path
    {
      self.inner.manifest_path.as_path()
    }
    
    pub fn version( &self ) -> Version
    {
      self.inner.version.clone()
    }
    
    pub fn publish( &self ) -> Option< &Vec< String > >
    {
      self.inner.publish.as_ref()
    }
    
    pub fn metadata( &self ) -> &Value
    {
      &self.inner.metadata
    }
    
    pub fn repository( &self ) -> Option< &String >
    {
      self.inner.repository.as_ref()
    }
    
    pub fn features( &self ) -> &BTreeMap< String, Vec< String > >
    {
      &self.inner.features
    }
    
  }
  
  /// Stores information about current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    metadata : Option< Metadata >,
    manifest_dir : CrateDir,
    packages : Option< Vec< WorkspacePackage > >,
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
    /// Load data from current directory
    pub fn from_current_path() -> Result< Self >
    {
      let current_path = AbsolutePath::try_from( std::env::current_dir().unwrap_or_default() )?;
      let metadata = MetadataCommand::new().no_deps().exec().context("fail to load CargoMetadata")?;
      let packages = metadata.packages.iter().map( | p | p.clone().into() ).collect();
      Ok( Self
      {
        metadata : Some( metadata ),
        manifest_dir : CrateDir::try_from( current_path )?,
        packages : Some( packages ),
      })
    }

    /// Load data from current directory
    pub fn with_crate_dir( crate_dir : CrateDir ) -> Result< Self >
    {
      let metadata = MetadataCommand::new().no_deps().exec().context("fail to load CargoMetadata")?;
      let packages = metadata.packages.iter().map( | p | p.clone().into() ).collect();
      Ok
      (
        Self
        {
        metadata : Some( MetadataCommand::new().current_dir( crate_dir.as_ref() ).no_deps().exec().context( "fail to load CargoMetadata" )? ),
        manifest_dir : crate_dir,
        packages : Some( packages ),
        }
      )
    }
  }

  impl From< Metadata > for Workspace
  {
    fn from( value : Metadata ) -> Self
    {
      let path = value.workspace_root.as_std_path().parent().unwrap().to_path_buf();
      let path = AbsolutePath::try_from( path ).unwrap();
      let packages = value.packages.iter().map( | p | p.clone().into() ).collect();
      Self
      {
        metadata : Some( value ),
        manifest_dir : CrateDir::try_from( path ).unwrap(),
        packages : Some( packages ),
      }
    }
  }

  impl Workspace
  {
    /// Load data from the current location or from cache
    // FIX : Maybe unsafe. Take metadata of workspace in current dir.
    pub fn load( &mut self ) -> Result< &mut Self >
    {
      if self.metadata.is_none()
      {
        let metadata = Self::with_crate_dir( self.manifest_dir.clone() )?.metadata.unwrap();
        _ = self.metadata.insert( metadata );
      }

      Ok( self )
    }

    /// Force loads data from the current location
    // FIX : Maybe unsafe. Take metadata of workspace in current dir.
    pub fn force_reload( &mut self ) -> Result< &mut Self >
    {
      let metadata = Self::with_crate_dir( self.manifest_dir.clone() )?.metadata.unwrap();
      _ = self.metadata.insert( metadata );

      Ok( self )
    }
  }

  impl Workspace
  {
    /// Returns list of all packages
    pub fn packages( &self ) -> Result< &[ WorkspacePackage], WorkspaceError> {
      self
      .packages
      .as_ref()
      .ok_or_else( || WorkspaceError::MetadataError )
      .map( | metadata | metadata.as_slice() )
    }


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

    /// Return discord url
    pub fn discord_url( &self ) -> Result< Option< String >, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.workspace_metadata[ "discord_url" ].as_str().map( | url | url.to_string() ) )
    }

    /// Return the master branch
    pub fn master_branch( &self ) -> Result< Option< String >, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.workspace_metadata.get( "master_branch" ).and_then( | b | b.as_str() ).map( | b | b.to_string() ) )
    }

    /// Return the repository url
    pub fn repository_url( &self ) -> Result< Option< String >, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.workspace_metadata.get( "repo_url" ).and_then( | b | b.as_str() ).map( | b | b.to_string() ) )
    }

    /// Return the workspace_name
    pub fn workspace_name( &self ) -> Result< Option< String >, WorkspaceError >
    {
      Ok( self.metadata.as_ref().ok_or_else( || WorkspaceError::MetadataError )?.workspace_metadata.get( "workspace_name" ).and_then( | b | b.as_str() ).map( | b | b.to_string() ) )
    }

    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< P >( &self, manifest_path : P ) -> Option< &WorkspacePackage >
    where
      P : AsRef< Path >,
    {
      self
      .packages()
      .ok()
      .and_then
      (
        | packages |
        packages
        .iter()
        .find( | &p | p.manifest_path().as_std_path() == manifest_path.as_ref() )
      )
    }

    /// Returns a graph of packages.
    pub( crate ) fn graph( &self ) -> Graph< String, String >
    {
      let packages = self.packages().unwrap();
      let module_package_filter : Option< Box< dyn Fn( &WorkspacePackage ) -> bool > > = Some
      (
        Box::new( move | p | p.publish().is_none() )
      );
      let module_dependency_filter : Option< Box< dyn Fn( &WorkspacePackage, &cargo_metadata::Dependency) -> bool > > = Some
      (
        Box::new
        (
          move | _, d | d.path.is_some() && d.kind != cargo_metadata::DependencyKind::Development
        )
      );
      let module_packages_map = packages::filter
      (
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
  exposed use Workspace;
  orphan use WorkspaceError;
  protected use WorkspacePackage;
}
