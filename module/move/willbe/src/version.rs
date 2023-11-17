/// Internal namespace.
mod private
{
  use std::fmt::Formatter;
  use std::path::Path;
  use toml_edit::value;
  use semver::Version;
  use crate::manifest;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Bump report.
  #[ derive( Debug, Clone ) ]
  pub struct BumpReport
  {
    /// Bumped package name.
    pub package_name : Option< String >,
    /// Old package version.
    pub old_version : Option< String >,
    /// New package version.
    pub new_version : Option< String >,
    /// Actual result.
    pub report : CmdReport,
  }

  impl BumpReport
  {
    fn new( report : CmdReport ) -> Self
    {
      BumpReport { package_name : None, old_version : None, new_version : None, report }
    }
  }

  impl std::fmt::Display for BumpReport
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result
    {
      f
      .debug_struct( "BumpReport" )
      .field( "stdout", &self.report.out )
      .field( "stderr", &self.report.err )
      .finish()
    }
  }

  /// Bump version as a string.
  pub fn bump_from_str( version : &str ) -> Result< String >
  {
    let mut version : Version = version.parse()?;

    if version.major != 0
    {
      version.major += 1;
      version.minor = 0;
      version.patch = 0;
    }
    else if version.minor != 0
    {
      version.minor += 1;
      version.patch = 0;
    }
    else
    {
      version.patch += 1;
    }

    Ok( version.to_string() )
  }

  /// Bump package version by manifest path.
  pub fn bump< P >( path : P, dry : bool ) -> Result< BumpReport >
  where
    P : AsRef< Path >,
  {
    let mut manifest = manifest::get( path.as_ref() )?;
    let internal_report = CmdReport
    {
      command : "bump".to_string(),
      path : path.as_ref().to_path_buf(),
      out : String::new(),
      err : String::new(),
    };
    let mut report = BumpReport::new( internal_report );
    let ( name, version ) =
    {
      let data = manifest.manifest_data.as_ref().unwrap();
      if !manifest.package_is()
      {
        report.report.err = format!( "`{}` - not a package", manifest.manifest_path.display() );

        return Ok( report );
      }
      let package = data.get( "package" ).unwrap();

      let name = package.get( "name" ).unwrap().as_str().unwrap().to_string();
      let version = package.get( "version" );
      if version.is_none()
      {
        report.report.err = format!( "`{}` - can not read the version", manifest.manifest_path.display() );

        return Ok( report );
      }
      let version = version.unwrap().as_str().unwrap().to_string();

      ( name, version )
    };

    let new_version = bump_from_str( &version )?;

    if !dry
    {
      let manifest = manifest.manifest_data.as_mut().unwrap();
      manifest[ "package" ][ "version" ] = value( &new_version );
    }

    report.report.out = format!( "`{name}` bumped from `{version}` to `{new_version}`" );

    report.package_name = Some( name );
    report.old_version = Some( version );
    report.new_version = Some( new_version );

    Ok( report )
  }
}

#[ cfg( test ) ]
mod tests
{
  mod bump_str
  {
    use crate::version::private::bump_from_str;

    #[ test ]
    fn patch()
    {
      // Arrange
      let version = "0.0.0";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "0.0.1", &new_version );
    }

    #[ test ]
    fn minor_without_patches()
    {
      // Arrange
      let version = "0.1.0";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "0.2.0", &new_version );
    }

    #[ test ]
    fn minor_with_patch()
    {
      // Arrange
      let version = "0.1.1";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "0.2.0", &new_version );
    }

    #[ test ]
    fn major_without_patches()
    {
      // Arrange
      let version = "1.0.0";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "2.0.0", &new_version );
    }

    #[ test ]
    fn major_with_minor()
    {
      // Arrange
      let version = "1.1.0";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "2.0.0", &new_version );
    }

    #[ test ]
    fn major_with_patches()
    {
      // Arrange
      let version = "1.1.1";

      // Act
      let new_version = bump_from_str( version ).unwrap();

      // Assert
      assert_eq!( "2.0.0", &new_version );
    }
  }
}

//

crate::mod_interface!
{
  /// Bump report.
  protected use BumpReport;
  /// Bump version.
  protected use bump;
}
