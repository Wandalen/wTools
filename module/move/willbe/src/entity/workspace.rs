mod private
{
  use crate::*;

  // qqq : for Bohdan : bad
  // use std::*;

  use std::slice;
  use former::{ Former };

  /// Stores information about the current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    /// Metadata of the workspace, containing detailed information about the packages, dependencies, and other workspace-related data.
    pub metadata : cargo_metadata::Metadata,
    /// The directory containing the manifest file (`Cargo.toml`) of the workspace.
    pub crate_dir : CrateDir,
  }

  /// Represents errors related to workspace operations.
  #[ derive( Debug, error::typed::Error ) ]
  pub enum WorkspaceInitError
  {
    /// Something went wrong with path to a workspace.
    #[ error( "Path error. Details: {0}" ) ]
    Path( #[ from ] PathError ),
    /// Something went wrong with the workspace' data
    #[ error( "Can not load workspace data. Details: {0}" ) ]
    Metadata( #[ from ] cargo_metadata::Error ),
    /// Files error
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
  }

  impl TryFrom< CrateDir > for Workspace
  {
    type Error = WorkspaceInitError;

    /// Load data from current directory
    fn try_from( mut crate_dir : CrateDir ) -> Result< Self, Self::Error >
    {
      let metadata = cargo_metadata::MetadataCommand::new()
      .current_dir( crate_dir.as_ref() )
      .no_deps()
      .exec()?;
      // inout crate dir may refer on crate's manifest dir, not workspace's manifest dir
      crate_dir = ( &metadata.workspace_root ).try_into()?;
      Ok( Self
      {
        metadata,
        crate_dir,
      })
    }

  }

  impl TryFrom< CurrentPath > for Workspace
  {
    type Error = WorkspaceInitError;

    /// Load data from current directory
    fn try_from( cd : CurrentPath ) -> Result< Self, Self::Error >
    {
      Self::try_from( CrateDir::transitive_try_from::< AbsolutePath >( cd )? )
    }

  }

  impl From< cargo_metadata::Metadata > for Workspace
  {
    fn from( metadata : cargo_metadata::Metadata ) -> Self
    {
      // SAFE: `workspace_root` is a path to a`Cargo.toml` file, therefor the parent is the directory
      let path = metadata.workspace_root.as_std_path().parent().unwrap().to_path_buf();
      let crate_dir = CrateDir::try_from( path ).unwrap();
      Self
      {
        metadata,
        crate_dir,
      }
    }
  }

  impl Workspace
  {

    /// Returns list of all packages
    pub fn packages< 'a >( &'a self )
    -> core::iter::Map
    <
      slice::Iter< 'a, cargo_metadata::Package >,
      impl Fn( &'a cargo_metadata::Package ) -> WorkspacePackageRef< 'a > + Clone,
    >
    {
      self.metadata.packages.iter().map( WorkspacePackageRef::from )
    }

    /// Returns the path to workspace root
    pub fn workspace_root( &self ) -> CrateDir
    {
      // Safe because workspace_root.as_std_path() is always a path to a directory
      CrateDir::try_from( self.metadata.workspace_root.as_std_path() ).unwrap()
    }

    /// Returns the path to target directory
    pub fn target_directory( &self ) -> &std::path::Path
    {
      self.metadata.target_directory.as_std_path()
    }

    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< 'a, P >( &'a self, manifest_file : P ) -> Option< WorkspacePackageRef< 'a > >
    where
      P : AsRef< std::path::Path >,
    {
      self
      .packages()
      .find( | &p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref() )
    }

    /// Filter of packages.
    pub fn packages_which< 'a >( &'a self ) -> PackagesFilterFormer< 'a >
    {
      // PackagesFilter::new( self )
      PackagesFilter::former().workspace( self )
    }

  }


  #[ derive( Former ) ]
  // #[ debug ]
  #[ allow( missing_debug_implementations ) ]
  pub struct PackagesFilter< 'a >
  {
    workspace : &'a Workspace,
    crate_dir : Box< dyn PackageFilter >,
    manifest_file : Box< dyn PackageFilter >,
  }

  pub trait PackageFilter
  {
    fn include( &self, package : WorkspacePackageRef< '_ > ) -> bool;
  }

  impl Default for Box< dyn PackageFilter >
  {
    fn default() -> Self
    {
      Box::new( PackageFilterAll )
    }
  }

  pub struct PackageFilterAll;
  impl PackageFilter for PackageFilterAll
  {
    #[ inline( always ) ]
    fn include( &self, _package : WorkspacePackageRef< '_ > ) -> bool
    {
      true
    }
  }

  pub struct PackageFilterCrateDir( CrateDir );
  impl PackageFilter for PackageFilterCrateDir
  {
    #[ inline( always ) ]
    fn include( &self, package : WorkspacePackageRef< '_ > ) -> bool
    {
      self.0 == package.crate_dir().unwrap()
    }
  }

  impl From< CrateDir > for Box< dyn PackageFilter >
  {
    #[ inline( always ) ]
    fn from( src : CrateDir ) -> Self
    {
      Box::new( PackageFilterCrateDir( src ) )
    }
  }

  pub struct PackageFilterManifestFile( ManifestFile );
  impl PackageFilter for PackageFilterManifestFile
  {
    #[ inline( always ) ]
    fn include( &self, package : WorkspacePackageRef< '_ > ) -> bool
    {
      self.0 == package.manifest_file().unwrap()
    }
  }

  impl From< ManifestFile > for Box< dyn PackageFilter >
  {
    #[ inline( always ) ]
    fn from( src : ManifestFile ) -> Self
    {
      Box::new( PackageFilterManifestFile( src ) )
    }
  }

  impl< 'a > PackagesFilter< 'a >
  {

    pub fn new( workspace : &'a Workspace ) -> Self
    {
      Self
      {
        workspace,
        crate_dir : Default::default(),
        manifest_file : Default::default(),
      }
    }

    #[ inline( always ) ]
    pub fn iter( &'a self ) -> impl Iterator< Item = WorkspacePackageRef< 'a > > + Clone
    {

      // self
      // .workspace
      // .packages()
      // .find( | &p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref() )

      // let filter_crate_dir = if Some( crate_dir ) = self.crate_dir
      // {
      //   | p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref()
      // }

      std::iter::empty()
    }

  }

  impl< 'a > PackagesFilterFormer< 'a >
  {
    #[ inline( always ) ]
    // pub fn find< 'a >( self ) -> impl Iterator< Item = WorkspacePackageRef< 'a > > + Clone
    pub fn find( self ) -> Option< WorkspacePackageRef< 'a > >
    {
      let formed = self.form();

      formed
      .workspace
      .packages()
      .find( | &p |
      {
        if !formed.crate_dir.include( p ) { return false };
        if !formed.manifest_file.include( p ) { return false };
        return true;
      })
      .clone()
      // .unwrap()

      // let filter_crate_dir = if Some( crate_dir ) = self.crate_dir
      // {
      //   | p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref()
      // }

      // std::iter::empty()
    }
  }

  impl Entries for Workspace
  {
    fn entries( &self ) -> impl IterTrait< '_, SourceFile >
    {
      self
      .packages()
      .flat_map( | package | package.entries().collect::< Vec< _ > >() )
      .collect::< Vec< _ > >()
      .into_iter()
    }
  }

  impl Sources for Workspace
  {
    fn sources( &self ) -> impl IterTrait< '_, SourceFile >
    {
      self
      .packages()
      .flat_map( | package | package.sources().collect::< Vec< _ > >() )
      .collect::< Vec< _ > >().into_iter()
      // .into_iter()
    }
  }

}

//

crate::mod_interface!
{
  exposed use WorkspaceInitError;
  exposed use Workspace;
}
