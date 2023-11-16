/// Internal namespace.
mod private
{
  use std::path::Path;
  use toml_edit::value;
  use crate::manifest;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Bump version as a string.
  pub fn bump_from_str( version : &str ) -> Result< String >
  {
    let mut splits : Vec< &str > = version.split( '.' ).collect();
    let patch_version = splits[ 2 ].parse::< u32 >()? + 1;
    let v = &patch_version.to_string();
    splits[ 2 ] = v;

    Ok( splits.join( "." ) )
  }

  /// Bump package version by manifest path.
  pub fn bump< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >,
  {
    let mut manifest = manifest::get( path.as_ref() )?;
    let mut report = CmdReport
    {
      command : "bump".to_string(),
      path : path.as_ref().to_path_buf(),
      out : String::new(),
      err : String::new(),
    };
    let ( name, version ) =
    {
      let data = manifest.manifest_data.as_ref().unwrap();
      if manifest.package_is()
      {
        report.err = format!( "`{}` - not a package", manifest.manifest_path.display() );

        return Ok( report );
      }
      let package = data.get( "package" ).unwrap();

      let name = package.get( "name" ).unwrap().to_string();
      let version = package.get( "version" );
      if version.is_none()
      {
        report.err = format!( "`{}` - can not read the version", manifest.manifest_path.display() );

        return Ok( report );
      }
      let version = version.unwrap().to_string();

      ( name, version )
    };

    let new_version = bump_from_str( &version )?;

    if !dry
    {
      let manifest = manifest.manifest_data.as_mut().unwrap();
      manifest[ "package" ][ "version" ] = value( new_version );
    }

    report.out = format!( "`{name}` bumped from `{version}` to `{new_version}`" );


    Ok( report )
  }
}

//

crate::mod_interface!
{
  /// Bump version.
  protected use bump;
}
