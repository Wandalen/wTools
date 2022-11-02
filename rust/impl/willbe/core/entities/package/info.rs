/// Internal namespace.
pub( crate ) mod private
{
  use toml::Value;
  use std::path::PathBuf;

  use crate::Package;

  /// Information about package
  #[ derive( Debug, Default, Clone ) ]
  pub struct PackageInfo
  {
    /// Package name
    pub name : String,
    /// Package version
    pub version : String,
    /// List of dependencies
    pub dependencies : Vec< String >,
    /// Package location
    pub location : PathBuf,
  }

  impl From< Package > for PackageInfo
  {
    fn from( package : Package ) -> Self
    {
      let path = package.path();
      let config_str = std::fs::read_to_string( path.join( "Cargo.toml" ) ).unwrap();
      let toml = config_str.parse::< Value >().unwrap();

      if let Some( package ) = toml.get( "package" )
      {
        Self
        {
          name : package[ "name" ].as_str().unwrap_or( "" ).to_owned(),
          version : package[ "version" ].as_str().unwrap_or( "" ).to_owned(),
          dependencies : vec![], // ! implement me
          location : path.to_owned(),
        }
      }
      else
      {
        Default::default()
      }
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use PackageInfo;
}
