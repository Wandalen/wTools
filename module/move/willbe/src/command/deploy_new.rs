mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::{ anyhow::Context, Result };
  use tools::template::Template;
  use endpoint::deploy_new::*;
  
  ///
  /// Create new deploy.
  ///
  
  pub fn deploy_new( ( _, _properties ) : ( Args, Props ) ) -> Result< () >
  {
    let mut template = DeployTemplate::default();
    let _parameters = template.parameters();
    // TODO: fetch values from props
    template.set_values(Default::default());
    endpoint::deploy_new( &std::env::current_dir()?, template ).context( "Fail to create deploy template" )
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use deploy_new;
}

