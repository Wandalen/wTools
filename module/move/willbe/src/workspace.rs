mod private
{
  use std::path::Path;
  use cargo_metadata::*;
  use wca::wtools::error;
  use crate::CrateDir;
  use crate::path::AbsolutePath;

  /// Stores information about current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    metadata : Option< Metadata >,
    manifest_dir : CrateDir,
  }

  impl Workspace
  {
    /// Load data from current directory
    pub fn from_current_path() -> error::Result< Self >
    {
      let current_path = AbsolutePath::try_from( std::env::current_dir().unwrap_or_default() )?;
      Ok( Self
      {
        metadata : Some( MetadataCommand::new().no_deps().exec().unwrap() ),
        manifest_dir : CrateDir::try_from( current_path )?,
      })
    }

    /// Load data from current directory
    pub fn with_crate_dir( crate_dir : CrateDir ) -> Self
    {
      Self
      {
        metadata : Some( MetadataCommand::new().current_dir( crate_dir.as_ref() ).no_deps().exec().unwrap() ),
        manifest_dir : crate_dir,
      }
    }
  }

  impl From< Metadata > for Workspace
  {
    fn from( value : Metadata ) -> Self
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
    // FIX: Maybe unsafe. Take metadata of workspace in current dir.
    pub fn load( &mut self ) -> &mut Self
    {
      if self.metadata.is_none()
      {
        self.metadata.get_or_insert_with( || Self::with_crate_dir( self.manifest_dir.clone() ).metadata.unwrap() );
      }

      self
    }

    /// Force loads data from the current location
    // FIX: Maybe unsafe. Take metadata of workspace in current dir.
    pub fn force_reload( &mut self ) -> &mut Self
    {
      _ = self.metadata.insert( Self::with_crate_dir( self.manifest_dir.clone() ).metadata.unwrap() );

      self
    }
  }

  impl Workspace
  {
    /// Returns list of all packages
    pub fn packages_get( &self ) -> &[ Package ]
    {
      &self.metadata.as_ref().unwrap().packages
    }

    /// Returns the path to workspace root
    pub fn workspace_root( &self ) -> &Path
    {
      self.metadata.as_ref().unwrap().workspace_root.as_std_path()
    }

    /// Returns the path to target directory
    pub fn target_directory( &self ) -> &Path
    {
      self.metadata.as_ref().unwrap().target_directory.as_std_path()
    }

    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< P >( &self, manifest_path : P ) -> Option< &Package >
    where
      P : AsRef< Path >,
    {
      self.packages_get().iter().find( | &p | p.manifest_path.as_std_path() == manifest_path.as_ref() )
    }
  }
}

//

crate::mod_interface!
{
  orphan use Workspace;
}
