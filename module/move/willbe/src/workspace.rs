mod private
{
  use std::path::{ Path, PathBuf };
  use cargo_metadata::*;

  /// Stores information about current workspace.
  #[ derive( Debug, Default, Clone ) ]
  pub struct Workspace
  {
    metadata : Option< Metadata >,
    manifest_dir : PathBuf,
  }

  impl Workspace
  {
    /// Load data from current directory
    pub fn from_current_path() -> Self
    {
      Self
      {
        metadata : Some( MetadataCommand::new().no_deps().exec().unwrap() ),
        manifest_dir : std::env::current_dir().unwrap_or_default(),
      }
    }

    /// Load data from current directory
    pub fn with_manifest_path< P >( path : P ) -> Self
    where
      P : Into< PathBuf >,
    {
      let path = path.into();

      Self
      {
        metadata : Some( MetadataCommand::new().manifest_path( path.join( "Cargo.toml" ) ).no_deps().exec().unwrap() ),
        manifest_dir : path,
      }
    }
  }

  impl From< Metadata > for Workspace
  {
    fn from( value : Metadata ) -> Self
    {
      let path = value.workspace_root.as_std_path().parent().unwrap().to_path_buf();

      Self
      {
        metadata : Some( value ),
        manifest_dir : path,
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
        self.metadata.get_or_insert_with( || Self::with_manifest_path( &self.manifest_dir ).metadata.unwrap() );
      }

      self
    }

    /// Force loads data from the current location
    // FIX: Maybe unsafe. Take metadata of workspace in current dir.
    pub fn force_reload( &mut self ) -> &mut Self
    {
      _ = self.metadata.insert( Self::with_manifest_path( &self.manifest_dir ).metadata.unwrap() );

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
