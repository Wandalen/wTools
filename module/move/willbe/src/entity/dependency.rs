mod private
{

  use crate::*;
  // use cargo_metadata::camino::{ Utf8PathBuf };

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
    // pub fn path( &self ) -> Option< Utf8PathBuf >
    pub fn crate_dir( &self ) -> Option< CrateDir >
    {
      panic!( "xxx : test me" );
      match self.inner.path
      {
        // Some( path ) => AsRef::< Utf8Path >::as_ref( &path ).try_into().ok(),
        Some( path ) => path.try_into().ok(),
        None => None,
      }
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

    /// Required version
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

}

//

crate::mod_interface!
{
  exposed use DependencyRef;
  exposed use DependencyKind;
}
