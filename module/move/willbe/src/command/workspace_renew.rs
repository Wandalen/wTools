mod private
{
  use crate::*;
  use former::Former;

  use wca::Props;
  use wtools::error::{ anyhow::Context, Result };
  use action::WorkspaceTemplate;

  #[ derive( Former ) ]
  struct WorkspaceNewProperties
  {
    repository_url : String,
    branches : Vec< String >,
  }

  ///
  /// Create new workspace.
  ///

  pub fn workspace_renew( properties : Props ) -> Result< () >
  {
    let WorkspaceNewProperties { repository_url, branches } = WorkspaceNewProperties::try_from( properties )?;
    let template = WorkspaceTemplate::default();
    action::workspace_renew( &std::env::current_dir()?, template, repository_url, branches ).context( "Fail to create workspace" )
  }

  impl TryFrom< Props > for WorkspaceNewProperties
  {
    type Error = wtools::error::for_app::Error;

    fn try_from( value : Props ) -> std::result::Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value.get_owned( "repository_url" ) { this.repository_url::< String >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "branches" ) { this.branches::< Vec< String > >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use workspace_renew;
}

