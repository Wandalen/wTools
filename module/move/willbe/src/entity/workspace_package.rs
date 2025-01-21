#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;
  use macros::kw;
  use collection::BTreeMap;
  use serde_json::Value;

  use std::
  {
    borrow::Cow,
  };

  // xxx : qqq : Deref, DerefMut, AsRef, AsMut

  /// Facade for `cargo_metadata::Package`
  #[ derive( Debug, Clone, Copy ) ]
  #[ repr( transparent ) ]
  pub struct WorkspacePackageRef< 'a >
  {
    // #[ serde( flatten ) ]
    inner : &'a cargo_metadata::Package,
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
    #[ must_use ]
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
      }
      self.inner.dependencies.iter().map( dependency_from )
    }

    /// Path to the manifest Cargo.toml
    ///
    /// # Errors
    /// qqq: doc
    pub fn manifest_file( &self ) -> Result< ManifestFile, PathError >
    {
      self.inner.manifest_path.as_path().try_into()
    }

    /// Path to the directory with manifest Cargo.toml.
    ///
    /// # Errors
    /// qqq: doc
    ///
    /// # Panics
    /// qqq: docs
    pub fn crate_dir( &self ) -> Result< CrateDir, PathError >
    {
      // SAFE because `manifest_path containing the Cargo.toml`
      self.inner.manifest_path.as_path().parent().unwrap().try_into()
    }

    /// The version field as specified in the Cargo.toml
    #[ must_use ]
    pub fn version( &self ) -> semver::Version
    {
      self.inner.version.clone()
    }

    /// List of registries to which this package may be published (derived from the publish field).
    /// Publishing is unrestricted if None, and forbidden if the Vec is empty.
    /// This is always None if running with a version of Cargo older than 1.39.
    #[ must_use ]
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
    #[ must_use ]
    pub fn metadata( &self ) -> &Value
    {
      &self.inner.metadata
    }

    /// The repository URL as specified in the Cargo.toml
    #[ must_use ]
    pub fn repository( &self ) -> Option< &String >
    {
      self.inner.repository.as_ref()
    }

    /// Features provided by the crate, mapped to the features required by that feature.
    #[ must_use ]
    pub fn features( &self ) -> &BTreeMap< String, Vec< String > >
    {
      &self.inner.features
    }
  }

  impl< 'a > Entries for WorkspacePackageRef< 'a >
  {
    fn entries( &self ) -> impl IterTrait< '_, SourceFile >
    {
      self.inner.targets.iter().map( | target |
      {
        let src_path = &target.src_path;
        let source : SourceFile = src_path.try_into().unwrap_or_else( | _ | panic!( "Illformed path to source file {src_path}" ) );
        // println!( " -- {:?} {:?}", source, target.kind );
        source
      })
    }
  }

  impl< 'a > Sources for WorkspacePackageRef< 'a >
  {
    fn sources( &self ) -> impl IterTrait< '_, SourceFile >
    {
      use walkdir::WalkDir;
      let crate_dir = self.crate_dir().unwrap();
      WalkDir::new( crate_dir )
      .into_iter()
      .filter_map( Result::ok )
      .filter( | e | e.path().extension().map_or( false, | ext | ext == "rs" ) )
      .map( | e | SourceFile::try_from( e.path() ).unwrap() )
      .collect::< Vec< _ > >()
      .into_iter()
    }
  }

  impl< 'a > CodeItems for WorkspacePackageRef< 'a >
  {
    fn items( &self ) -> impl IterTrait< '_, syn::Item >
    {
      self
      .sources()
      .flat_map( | source | source.items().collect::< Vec< _ > >().into_iter() )
      .collect::< Vec< _ > >().into_iter()
    }
  }

  impl< 'a > AsCode for WorkspacePackageRef< 'a >
  {
    fn as_code( &self ) -> std::io::Result< Cow< '_, str > >
    {
      let mut results : Vec< String > = Vec::new();
      // zzz : introduce formatter

      for source in self.sources()
      {
        let code = source.as_code()?.into_owned();
        let mut filename = source
        .as_ref()
        .with_extension( "" )
        .file_name()
        .unwrap_or_else( || panic!( "Cant get file name of path {}", source.as_ref().display() ) )
        .to_string_lossy()
        .replace( '.', "_" );

        if kw::is( &filename )
        {
          filename.push_str( "_rs" );
        }

        // qqq : xxx : use callbacks instead of expect

        results.push( format!( "// === Begin of File {}", source.as_ref().display() ) );
        results.push( format!( "mod {filename}\n{{\n" ) );
        results.push( code );
        results.push( "\n}".to_string() );
        results.push( format!( "// === End of File {}", source.as_ref().display() ) );

      }

      let joined = results.join( "\n" );
      Ok( Cow::Owned( joined ) )
    }
  }

}

//

crate::mod_interface!
{
  exposed use WorkspacePackageRef;
}
