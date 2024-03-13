mod private
{
  use crate::*;

  use wca::Props;
  use wtools::error::{ anyhow::Context, Result };
  use tool::template::Template;
  use action::deploy_renew::*;

  ///
  /// Create new deploy.
  ///

  pub fn deploy_renew( properties : Props ) -> Result< () >
  {
    let mut template = DeployTemplate::default();
    let parameters = template.parameters();
    let mut values = parameters.values_from_props( &properties );
    for mandatory in parameters.get_mandatory()
    {
      values.interactive_if_empty( mandatory );
    }
    template.set_values( values );
    action::deploy_renew( &std::env::current_dir()?, template ).context( "Fail to create deploy template" )
  }

}

crate::mod_interface!
{
  /// Create deploy from template.
  orphan use deploy_renew;
}

