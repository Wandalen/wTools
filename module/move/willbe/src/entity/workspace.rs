mod private
{
  use std::collections::BTreeMap;
  use crate::*;

  use std::path::Path;
  use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };
  use petgraph::Graph;
  use serde::Deserialize;
  use serde_json::Value;
  use wtools::error::
  {
    for_app::Context,
    for_lib::Error,
    Result
  };
  use _path::AbsolutePath;

  /// Facade for cargo_metadata::Package
  #[ derive( Debug, Clone, Deserialize ) ]
  pub struct WorkspacePackage
  {
    #[ serde( flatten ) ]
    inner : cargo_metadata::Package
  }
  
  impl From< cargo_metadata::Package > for WorkspacePackage
  {
    fn from( inner : cargo_metadata::Package) -> Self 
    {
      Self
      {
        inner
      }
    }
  }
  
  impl WorkspacePackage
  {
    /// The name field as given in the Cargo.toml
    pub fn name( &self ) -> &String
    {
      &self.inner.name
    }
    
    /// List of dependencies of this particular package
    pub fn dependencies( &self ) -> Vec< Dependency >
    {
      self.inner.dependencies.iter().cloned().map( Dependency::from ).collect()
    }
    
    /// Path containing the Cargo.toml
    pub fn manifest_path( &self ) -> &Utf8Path
    {
      self.inner.manifest_path.as_path()
    }
    
    /// The version field as specified in the Cargo.toml
    pub fn version( &self ) -> semver::Version
    {
      self.inner.version.clone()
    }
    
    /// List of registries to which this package may be published (derived from the publish field).
    /// Publishing is unrestricted if None, and forbidden if the Vec is empty.
    /// This is always None if running with a version of Cargo older than 1.39.
    pub fn publish( &self ) -> Option< &Vec< String > >
    {
      self.inner.publish.as_ref()
    }
    
    ///Contents of the free form package.metadata section.
    /// This contents can be serialized to a struct using serde:
    /// ``` rust
    /// use serde::Deserialize;
    /// use serde_json::json;
    /// 
    /// #[ derive( Debug, Deserialize ) ]
    /// struct SomePackageMetadata 
    /// {
    ///   some_value : i32,
    /// }
    /// 
    /// fn main() 
    /// {
    ///   let value = json!
    ///   ({
    ///     "some_value" : 42,
    ///   });
    /// 
    ///   let package_metadata : SomePackageMetadata = serde_json::from_value( value ).unwrap();
    ///   assert_eq!( package_metadata.some_value, 42 );
    /// }
    /// ```
    pub fn metadata( &self ) -> &Value
    {
      &self.inner.metadata
    }
    
    /// The repository URL as specified in the Cargo.toml
    pub fn repository( &self ) -> Option< &String >
    {
      self.inner.repository.as_ref()
    }
    
    /// Features provided by the crate, mapped to the features required by that feature.
    pub fn features( &self ) -> &BTreeMap< String, Vec< String > >
    {
      &self.inner.features
    }
  }
  
  /// A dependency of the main crate
  #[ derive( Debug ) ]
  pub struct Dependency
  {
    inner : cargo_metadata::Dependency,
  }
  
  impl Dependency
  {
    /// The file system path for a local path dependency.
    /// Only produced on cargo 1.51+
    pub fn path( &self ) -> Option< Utf8PathBuf >
    {
      self.inner.path.clone()
    }
    
    /// Name as given in the Cargo.toml.
    pub fn name( &self ) -> String
    {
      self.inner.name.clone()
    }
    
    /// The kind of dependency this is.
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
    
    /// he required version
    pub fn req( &self ) -> semver::VersionReq
    {
      self.inner.req.clone()
    }
  }
  
  impl From< cargo_metadata::Dependency > for Dependency
  {
    fn from( inner : cargo_metadata::Dependency ) -> Self 
    {
      Self
      {
        inner
      }
    }
  }

  /// Dependencies can come in three kinds
  #[ derive( Eq, PartialEq, Debug ) ]
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
    
  /// Stores information about current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    metadata : Option< cargo_metadata::Metadata >,
    manifest_dir : CrateDir,
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
      let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec().context("fail to load CargoMetadata")?;
      Ok( Self
      {
        metadata : Some( metadata ),
        manifest_dir : CrateDir::try_from( current_path )?,
      })
    }

    /// Load data from current directory
    pub fn with_crate_dir( crate_dir : CrateDir ) -> Result< Self >
    {
      Ok
      (
        Self
        {
          metadata : Some( cargo_metadata::MetadataCommand::new().current_dir( crate_dir.as_ref() ).no_deps().exec().context( "fail to load CargoMetadata" )? ),
          manifest_dir : crate_dir,
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
        manifest_dir : CrateDir::try_from( path ).unwrap(),
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
    pub fn packages( &self ) -> Result< Vec< WorkspacePackage >, WorkspaceError > 
    {
      self
      .metadata
      .as_ref()
      .ok_or_else( || WorkspaceError::MetadataError )
      .map( | metadata | metadata.packages.clone() )
      .map( | p | p.into_iter().map( WorkspacePackage::from ).collect() )
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
    pub fn package_find_by_manifest< P >( &self, manifest_path : P ) -> Option< WorkspacePackage >
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
        .cloned()
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
      let module_dependency_filter : Option< Box< dyn Fn( &WorkspacePackage, &Dependency ) -> bool > > = Some
      (
        Box::new
        (
          move | _, d | d.path().is_some() && d.kind() != DependencyKind::Development
        )
      );
      let module_packages_map = packages::filter
      (
        packages.as_slice(),
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
  protected use Dependency;
  protected use DependencyKind;
}
