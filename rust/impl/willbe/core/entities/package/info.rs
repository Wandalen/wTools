/// Internal namespace.
pub( crate ) mod private
{
  use toml::Value;

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
  }

  impl From< Package > for PackageInfo
  {
    fn from( package : Package ) -> Self
    {
      let config_str = std::fs::read_to_string( package.path().join( "Cargo.toml" ) ).unwrap();
      let toml = config_str.parse::< Value >().unwrap();

      if let Some( package ) = toml.get( "package" )
      {
        Self
        {
          name : package[ "name" ].as_str().unwrap_or( "" ).to_owned(),
          version : package[ "version" ].as_str().unwrap_or( "" ).to_owned(),
          dependencies : vec![], // ! implement me
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
