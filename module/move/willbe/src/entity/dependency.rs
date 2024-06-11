mod private
{
//   use std::collections::BTreeMap;
//   use crate::*;
//
//   use std::path::Path;
  use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };
//   use petgraph::Graph;
//   use serde::Deserialize;
//   use serde_json::Value;
//   use wtools::error::
//   {
//     for_app::Context,
//     for_lib::Error,
//     Result
//   };
//   use _path::AbsolutePath;

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

}

//

crate::mod_interface!
{
  exposed use Dependency;
  exposed use DependencyKind;
}
