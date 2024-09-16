mod private
{
  use crate::*;

  use wca::VerifiedCommand;
  use error::{ untyped::Context };
  use action::deploy_renew::*;

  ///
  /// Create new deploy.
  ///

  // xxx : qqq : typed error
  pub fn deploy_renew( o : VerifiedCommand ) -> error::untyped::Result< () >
  {
    let current_dir = std::env::current_dir()?;

    let mut template = DeployTemplate::default();
    _ = template.load_existing_params( &current_dir );
    let parameters = template.parameters();
    let mut values = parameters.values_from_props( &o.props );
    for mandatory in template.get_missing_mandatory()
    {
      values.interactive_if_empty( mandatory );
    }
    template.set_values( values );
    action::deploy_renew( &current_dir, template )
    .context( "Fail to create deploy template" )
  }

}

crate::mod_interface!
{
  /// Create deploy from template.
  orphan use deploy_renew;
}

