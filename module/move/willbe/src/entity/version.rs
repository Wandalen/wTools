/// Define a private namespace for all its items.
mod private
{
  use crate::*;

  use std::
  {
    fmt,
    str::FromStr,
  };
  use std::fmt::Formatter;
  use toml_edit::value;
  use semver::Version as SemVersion;

  use error::untyped::Result;
  use manifest::Manifest;
  use package::Package;
  use { error::untyped::format_err, iter::Itertools };

  /// Wrapper for a SemVer structure
  #[ derive( Debug, Clone, Eq, PartialEq, Ord, PartialOrd ) ]
  pub struct Version( SemVersion );

  impl FromStr for Version
  {
    type Err =  semver::Error;

    fn from_str( s : &str ) -> std::result::Result< Self, Self::Err >
    {
      Ok( Self( SemVersion::from_str( s )? ) )
    }
  }

  impl TryFrom< &str > for Version
  {
    type Error = semver::Error;

    fn try_from( value : &str ) -> Result< Self, Self::Error >
    {
      FromStr::from_str( value )
    }
  }

  impl TryFrom< &String > for Version
  {
    type Error = semver::Error;

    fn try_from( value : &String ) -> Result< Self, Self::Error >
    {
      Self::try_from( value.as_str() )
    }
  }

  impl fmt::Display for Version
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "{}", self.0.to_string() )
    }
  }

  impl Version
  {
    /// Bump a version with default strategy
    ///
    /// This function increases first not 0 number
    pub fn bump( self ) -> Self
    {
      let mut ver = self.0;
      // we shouldn't change the major part of a version yet
      if ver.minor != 0 || ver.major != 0
      {
        ver.minor += 1;
        ver.patch = 0;
      }
      else
      {
        ver.patch += 1;
      }

      Self( ver )
    }
  }

  /// A structure that represents a bump report, which contains information about a version bump.
  #[ derive( Debug, Default, Clone ) ]
  pub struct BumpReport
  {
    /// Pacakge name.
    pub name : Option< String >,
    /// Package old version.
    pub old_version : Option< String >,
    /// Package new version.
    pub new_version : Option< String >,
  }

  impl fmt::Display for BumpReport
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      let Self { name, old_version, new_version } = self;
      match ( name, old_version, new_version )
      {
        ( Some( name ), Some( old_version ), Some( new_version ) )
        => f.write_fmt( format_args!( "`{name}` bumped from {old_version} to {new_version}" ) ),
        _ => f.write_fmt( format_args!( "Bump failed" ) )
      }
    }
  }

  // qqq : we have to replace the implementation above with the implementation below, don't we?
  // qqq : for Bohdan : duplication?

  /// `BumpOptions` manages the details necessary for the version bump process for crates.
  /// This includes the directory of the crate whose version is being bumped, the old and new version numbers,
  /// and the set of dependencies of that crate.
  #[ derive( Debug, Clone ) ]
  pub struct BumpOptions
  {
    /// `crate_dir` - The directory of the crate which you want to bump the version of. This value is
    /// represented by `CrateDir` which indicates the directory of the crate.
    pub crate_dir : CrateDir,

    /// `old_version` - The version of the crate before the bump. It's represented by `Version` which
    /// denotes the old version number of the crate.
    pub old_version : Version,

    /// `new_version` - The version number to assign to the crate after the bump. It's also represented
    /// by `Version` which denotes the new version number of the crate.
    pub new_version : Version,

    /// `dependencies` - This is a vector containing the directories of all the dependencies of the crate.
    /// Each item in the `dependencies` vector indicates a `CrateDir` directory of a single dependency.
    pub dependencies : Vec< CrateDir >,

    /// `dry` - A boolean indicating whether to do a "dry run". If set to `true`, a simulated run is performed
    /// without making actual changes. If set to `false`, the operations are actually executed. This is
    /// useful for validating the process of bumping up the version or for testing and debugging.
    pub dry : bool,
  }

  /// Report about a changing version.
  #[ derive( Debug, Default, Clone ) ]
  pub struct ExtendedBumpReport
  {
    /// Pacakge name.
    pub name : Option< String >,
    /// Package old version.
    pub old_version : Option< String >,
    /// Package new version.
    pub new_version : Option< String >,
    /// Files that should(already) changed for bump.
    pub changed_files : Vec< ManifestFile >
  }

  impl std::fmt::Display for ExtendedBumpReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      let Self { name, old_version, new_version, changed_files } = self;
      if self.changed_files.is_empty()
      {
        write!( f, "Files were not changed during bumping the version" )?;
        return Ok( () )
      }

      let files = changed_files.iter().map( | f | f.as_ref().display() ).join( ",\n    " );
      match ( name, old_version, new_version )
      {
        ( Some( name ), Some( old_version ), Some( new_version ) )
        => writeln!( f, "`{name}` bumped from {old_version} to {new_version}\n  changed files :\n    {files}" ),
        _ => writeln!( f, "Bump failed" )
      }?;

      Ok( () )
    }
  }


  /// Bumps the version of a package and its dependencies.
  ///
  /// # Arguments
  ///
  /// * `o` - The options for version bumping.
  ///
  /// # Returns
  ///
  /// Returns a result containing the extended bump report if successful.
  ///
  // qqq : should be typed error, apply err_with
  // qqq : don't use 1-prameter Result
  pub fn bump( o : BumpOptions ) -> Result< ExtendedBumpReport >
  {
    let mut report = ExtendedBumpReport::default();
    // let manifest_file = o.crate_dir.inner().join( "Cargo.toml" );
    let manifest_file = o.crate_dir.manifest_file();
    let package = Package::try_from( manifest_file.clone() ).map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
    let name = package.name().map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
    report.name = Some( name.into() );
    let package_version = package.version().map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
    let current_version = version::Version::try_from( package_version.as_str() ).map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
    if current_version > o.new_version
    {
      return Err( format_err!( "{report:?}\nThe current version of the package is higher than need to be set\n\tpackage: {name}\n\tcurrent_version: {current_version}\n\tnew_version: {}", o.new_version ) );
    }
    report.old_version = Some( o.old_version.to_string() );
    report.new_version = Some( o.new_version.to_string() );

    let mut package_manifest = package.manifest().map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
    if !o.dry
    {
      // let data = package_manifest.data.as_mut().unwrap();
      let data = &mut package_manifest.data;
      data[ "package" ][ "version" ] = value( &o.new_version.to_string() );
      package_manifest.store()?;
    }
    report.changed_files = vec![ manifest_file ];
    let new_version = &o.new_version.to_string();
    for dep in &o.dependencies
    {
      // let manifest_file = dep.absolute_path().join( "Cargo.toml" );
      let manifest_file = dep.clone().manifest_file();
      let mut manifest = Manifest::try_from( manifest_file.clone() ).map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?;
      // let data = manifest.data.as_mut().unwrap();
      let data = &mut manifest.data;
      let item = if let Some( item ) = data.get_mut( "package" ) { item }
      else if let Some( item ) = data.get_mut( "workspace" ) { item }
      else { return Err( format_err!( "{report:?}\nThe manifest nor the package and nor the workspace" ) ); };
      if let Some( dependency ) = item.get_mut( "dependencies" ).and_then( | ds | ds.get_mut( &name ) )
      {
        if let Some( previous_version ) = dependency.get( "version" ).and_then( | v | v.as_str() ).map( | v | v.to_string() )
        {
          if previous_version.starts_with('~')
          {
            dependency[ "version" ] = value( format!( "~{new_version}" ) );
          }
          else
          {
            dependency[ "version" ] = value( new_version.clone() );
          }
        }
      }
      if !o.dry { manifest.store().map_err( | e | format_err!( "{report:?}\n{e:#?}" ) )?; }
      report.changed_files.push( manifest_file );
    }

    Ok( report )
  }

  /// Reverts the version of a package in the provided `ExtendedBumpReport`.
  ///
  /// # Arguments
  ///
  /// * `report` - The `ExtendedBumpReport` containing the bump information.
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` if the version is reverted successfully. Returns `Err` with an error message if there is any issue with reverting the version.
  // qqq : don't use 1-prameter Result
  pub fn revert( report : &ExtendedBumpReport ) -> error::untyped::Result< () > // qqq : use typed error
  {
    let Some( name ) = report.name.as_ref() else { return Ok( () ) };
    let Some( old_version ) = report.old_version.as_ref() else { return Ok( () ) };
    let Some( new_version ) = report.new_version.as_ref() else { return Ok( () ) };

    let dependencies = | item_maybe_with_dependencies : &mut toml_edit::Item |
    {
      if let Some( dependency ) = item_maybe_with_dependencies.get_mut( "dependencies" ).and_then( | ds | ds.get_mut( name ) )
      {
        if let Some( current_version ) = dependency.get( "version" ).and_then( | v | v.as_str() ).map( | v | v.to_string() )
        {
          let version = &mut dependency[ "version" ];
          if let Some( current_version ) = current_version.strip_prefix( '~' )
          {
            if current_version != new_version { return Err( format_err!( "The current version of the package does not match the expected one. Expected: `{new_version}` Current: `{}`", version.as_str().unwrap_or_default() ) ); }
            *version = value( format!( "~{}", old_version ) );
          }
          else
          {
            if version.as_str().unwrap() != new_version { return Err( format_err!( "The current version of the package does not match the expected one. Expected: `{new_version}` Current: `{}`", version.as_str().unwrap_or_default() ) ); }
            *version = value( old_version.clone() );
          }
        }
      }

      Ok( () )
    };

    for path in &report.changed_files
    {
      let mut manifest = Manifest::try_from( path.clone() )?;
      let data = manifest.data();
      if let Some( workspace ) = data.get_mut( "workspace" )
      {
        dependencies( workspace )?;
      }
      if let Some( package ) = data.get_mut( "package" )
      {
        if package.get_mut( "name" ).unwrap().as_str().unwrap() == name
        {
          let version = &mut package[ "version" ];
          if version.as_str().unwrap() != new_version { return Err( format_err!( "The current version of the package does not match the expected one. Expected: `{new_version}` Current: `{}`", version.as_str().unwrap_or_default() ) ); }
          *version = value( old_version.clone() );
        }
        else
        {
          dependencies( package )?;
        }
      }
      manifest.store()?;
    }

    Ok( () )
  }

  // qqq : for Bohdan : not used? why is it needed?
  /// Bump version by manifest.
  /// It takes data from the manifest and increments the version number according to the semantic versioning scheme.
  /// It then writes the updated manifest file back to the same path, unless the flag is set to true, in which case it only returns the new version number as a string.
  ///
  /// # Args :
  /// - `manifest` - a manifest mutable reference
  /// - `dry` - a flag that indicates whether to apply the changes or not
  ///         - `true` - does not modify the manifest file, but only returns the new version;
  ///         - `false` - overwrites the manifest file with the new version.
  ///
  /// # Returns :
  /// - `Ok` - the new version number as a string;
  /// - `Err` - if the manifest file cannot be read, written, parsed.
  pub fn manifest_bump( manifest : &mut Manifest, dry : bool ) -> Result< BumpReport, manifest::ManifestError >
  {
    let mut report = BumpReport::default();

    let version=
    {
      let data = &manifest.data;
      if !manifest.package_is()
      {
        return Err( manifest::ManifestError::NotAPackage );
      }
      let package = data.get( "package" ).unwrap();

      let version = package.get( "version" );
      if version.is_none()
      {
        return Err( manifest::ManifestError::CannotFindValue( "version".into() ) );
      }
      let version = version.unwrap().as_str().unwrap();
      report.name = Some( package[ "name" ].as_str().unwrap().to_string() );
      report.old_version = Some( version.to_string() );

      Version::from_str( version ).map_err( | e | manifest::ManifestError::InvalidValue( e.to_string() ) )?
    };

    let new_version = version.bump().to_string();
    report.new_version = Some( new_version.clone() );

    if !dry
    {
      let data = &mut manifest.data;
      data[ "package" ][ "version" ] = value( &new_version );
      manifest.store()?;
    }

    Ok( report )
  }
}

//

crate::mod_interface!
{
  /// Version entity.
  exposed use Version;

  /// Report for bump operation.
  own use BumpReport;

  /// Options for version bumping.
  own use BumpOptions;
  /// Report about a changing version with list of files that was changed.
  own use ExtendedBumpReport;

  /// Bumps the version of a package and its dependencies.
  own use manifest_bump;
  /// Bump version.
  own use bump;

  /// Reverts the version of a package.
  own use revert;
}
