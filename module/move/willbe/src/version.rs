/// Internal namespace.
mod private
{
  use crate::*;
  use std::fmt;
  use std::str::FromStr;
  use toml_edit::value;
  use semver::Version as SemVersion;
  use wtools::error::for_app::{ Result, anyhow };
  use manifest::Manifest;

  /// Wrapper for a SemVer structure
  #[ derive( Debug, Clone, Eq, PartialEq ) ]
  pub struct Version( SemVersion );

  impl FromStr for Version
  {
    type Err =  semver::Error;

    fn from_str( s : &str ) -> std::result::Result< Self, Self::Err >
    {
      Ok( Self( SemVersion::from_str( s )? ) )
    }
  }

  impl fmt::Display for Version
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
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

  // qqq : for Bohdan : should return report
  /// Bump version by manifest.
  /// It takes data from the manifest and increments the version number according to the semantic versioning scheme.
  /// It then writes the updated manifest file back to the same path, unless the flag is set to true, in which case it only returns the new version number as a string.
  ///
  /// # Args:
  /// - `manifest` - a manifest mutable reference
  /// - `dry` - a flag that indicates whether to apply the changes or not
  ///         - `true` - does not modify the manifest file, but only returns the new version;
  ///         - `false` - overwrites the manifest file with the new version.
  ///
  /// # Returns:
  /// - `Ok` - the new version number as a string;
  /// - `Err` - if the manifest file cannot be read, written, parsed.
  pub fn bump( manifest : &mut Manifest, dry : bool ) -> Result< String >
  {
    let version=
    {
      if manifest.manifest_data.is_none()
      {
        manifest.load()?;
      }
      let data = manifest.manifest_data.as_ref().unwrap();
      if !manifest.package_is()
      {
        // qqq : for Bohdan : rid off untyped errors, make proper errors handing
        // https://www.lpalmieri.com/posts/error-handling-rust/
        return Err( anyhow!( "`{}` - not a package", manifest.manifest_path().as_ref().display() ) );
      }
      let package = data.get( "package" ).unwrap();

      let version = package.get( "version" );
      if version.is_none()
      {
        return Err( anyhow!( "`{}` - can not read the version", manifest.manifest_path().as_ref().display() ) );
      }

      Version::from_str( version.unwrap().as_str().unwrap() )?
    };

    let new_version = version.bump().to_string();

    if !dry
    {
      let data = manifest.manifest_data.as_mut().unwrap();
      data[ "package" ][ "version" ] = value( &new_version );
      manifest.store()?;
    }

    Ok( new_version )
  }
}

#[ cfg( test ) ]
mod tests
{
  mod bump_str
  {
    use std::str::FromStr;
    use crate::version::private::Version;
    // qqq : for Bohdan : move to tests folder

    #[ test ]
    fn patch()
    {
      // Arrange
      let version = Version::from_str( "0.0.0" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "0.0.1", &new_version.to_string() );
    }

    #[ test ]
    fn minor_without_patches()
    {
      // Arrange
      let version = Version::from_str( "0.1.0" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "0.2.0", &new_version.to_string() );
    }

    #[ test ]
    fn minor_with_patch()
    {
      // Arrange
      let version = Version::from_str( "0.1.1" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "0.2.0", &new_version.to_string() );
    }

    #[ test ]
    fn major_without_patches()
    {
      // Arrange
      let version = Version::from_str( "1.0.0" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "2.0.0", &new_version.to_string() );
    }

    #[ test ]
    fn major_with_minor()
    {
      // Arrange
      let version = Version::from_str( "1.1.0" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "2.0.0", &new_version.to_string() );
    }

    #[ test ]
    fn major_with_patches()
    {
      // Arrange
      let version = Version::from_str( "1.1.1" ).unwrap();

      // Act
      let new_version = version.bump();

      // Assert
      assert_eq!( "2.0.0", &new_version.to_string() );
    }
  }
}

//

crate::mod_interface!
{
  /// Version entity.
  protected use Version;
  /// Bump version.
  protected use bump;
}
