mod private
{
  use crate::*;
  use former::Former;

  use wca::VerifiedCommand;
  use error::{ untyped::Context, Result };
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

  pub fn workspace_renew( o : VerifiedCommand ) -> Result< () >
  {
    let WorkspaceNewProperties { repository_url, branches } = o.props.try_into()?;
    let template = WorkspaceTemplate::default();
    action::workspace_renew
    ( 
      &std::env::current_dir()?, 
      template, 
      repository_url, 
      branches 
    )
    .context( "Fail to create workspace" )
  }

  impl TryFrom< wca::Props > for WorkspaceNewProperties
  {
    type Error = error::untyped::Error;

    fn try_from( value : wca::Props ) -> std::result::Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value
      .get_owned( "repository_url" ) { this.repository_url::< String >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "branches" ) { this.branches::< Vec< String > >( v ) } else { this };

      Ok( this.form() )
    }
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use workspace_renew;
}

