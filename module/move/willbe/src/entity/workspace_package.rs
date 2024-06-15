mod private
{
  use crate::*;
  use std::collections::BTreeMap;

  // use std::path::Path;
  use cargo_metadata::camino::{ Utf8Path };
  // use petgraph::Graph;
  // use serde::Serialize;
  // use serde::Deserialize;
  // use serde::{ serde };
  use serde_json::Value;
  // use wtools::error::
  // {
  //   for_app::Context,
  //   for_lib::Error,
  //   Result
  // };
  // use _path::AbsolutePath;

  /// Facade for cargo_metadata::Package
  #[ derive( Debug, Clone, Copy ) ]
  #[ repr( transparent ) ]
  pub struct WorkspacePackageRef< 'a >
  {
    // #[ serde( flatten ) ]
    inner : &'a cargo_metadata::Package
    // qqq : xxx : why no CrateDir is here?
  }

  impl< 'a > From< &'a cargo_metadata::Package > for WorkspacePackageRef< 'a >
  {
    fn from( inner : &'a cargo_metadata::Package ) -> Self
    {
      Self
      {
        inner
      }
    }
  }

  impl< 'a > WorkspacePackageRef< 'a >
  {
    /// The name field as given in the Cargo.toml
    pub fn name( &'a self ) -> &'a str
    {
      &self.inner.name
    }

    /// List of dependencies of this particular package
    pub fn dependencies( &'a self )
    -> core::iter::Map
    <
      core::slice::Iter< 'a, cargo_metadata::Dependency >,
      fn( &'a cargo_metadata::Dependency ) -> DependencyRef< 'a >,
    >
    {
      fn dependency_from( dependency : &cargo_metadata::Dependency ) -> DependencyRef< '_ >
      {
        dependency.into()
        // Dependency::from( dependency )
      }
      self.inner.dependencies.iter().map( dependency_from )
    }

    // /// List of dependencies of this particular package
    // // pub fn dependencies( &self ) -> Vec< Dependency< '_ > >
    // pub fn dependencies< 'a >( &'a self )
    // // -> core::slice::Iter< '_, Dependency< '_ > >
    // -> core::iter::Map
    // <
    //   core::slice::Iter< 'a, &'a cargo_metadata::Dependency >,
    //   fn( &'a cargo_metadata::Dependency ) -> Dependency< 'a >,
    // >
    // {
    //   self.inner.dependencies.iter().map( Dependency::from )
    //   // self.inner.dependencies.iter().cloned().map( Dependency::from ).collect()
    // }

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

}

//

crate::mod_interface!
{
  exposed use WorkspacePackageRef;
}
