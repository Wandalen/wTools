mod private
{
  use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };

  /// A dependency of the main crate
  #[ derive( Debug, Clone, Copy ) ]
  #[ repr( transparent ) ]
  pub struct Dependency< 'a >
  {
    inner : &'a cargo_metadata::Dependency,
  }

  impl< 'a > Dependency< 'a >
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

  impl< 'a > From< &'a cargo_metadata::Dependency > for &'a Dependency< 'a >
  {
    fn from( inner : &'a cargo_metadata::Dependency ) -> Self
    {
      // xxx
      // SAFETY :
      // `Dependency` is transperent type and as so has exactly the same layout as argo_metadata::Dependency.
      // The types have identical memory layouts and lifetimes so it's safe.
      #[ allow( unsafe_code ) ]
      unsafe { core::mem::transmute( &inner ) }
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
  exposed use Dependency;
  exposed use DependencyKind;
}
