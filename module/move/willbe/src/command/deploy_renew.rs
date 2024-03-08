mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::{ anyhow::Context, Result };
  use tool::template::Template;
  use action::deploy_renew::*;

  ///
  /// Create new deploy.
  ///

  pub fn deploy_renew( ( _, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let mut template = DeployTemplate::default();
    let parameters = template.parameters();
    let values = parameters.values_from_props( &properties );
    template.set_values( values );
    action::deploy_renew( &std::env::current_dir()?, template ).context( "Fail to create deploy template" )
  }
}

crate::mod_interface!
{
  /// Create deploy from template.
  exposed use deploy_renew;
}

