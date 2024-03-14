/// Internal namespace.
mod private
{
  use crate::*;

  use std::
  {
    fmt,
    str::FromStr,
  };
  use toml_edit::value;
  use semver::Version as SemVersion;

  use wtools::error::for_app::Result;
  use manifest::Manifest;

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
      if ver.major != 0
      {
        ver.major += 1;
        ver.minor = 0;
        ver.patch = 0;
      }
      else if ver.minor != 0
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
  pub fn bump( manifest : &mut Manifest, dry : bool ) -> Result< BumpReport, manifest::ManifestError >
  {
    let mut report = BumpReport::default();

    let version=
    {
      if manifest.manifest_data.is_none()
      {
        manifest.load()?;
      }
      let data = manifest.manifest_data.as_ref().unwrap();
      if !manifest.package_is()?
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
      let data = manifest.manifest_data.as_mut().unwrap();
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
  protected use Version;

  /// Report for bump operation.
  protected use BumpReport;

  /// Bump version.
  protected use bump;
}
