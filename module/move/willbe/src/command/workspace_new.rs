mod private
{
  use former::Former;
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::Result;

  #[ derive( Former ) ]
  struct WorkspaceNewProperties
  {
    repository_url : String,
    branches : Vec< String >,
  }
  
  ///
  /// Create new workspace.
  ///
  
  pub fn workspace_new( ( _, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let WorkspaceNewProperties { repository_url, branches } = WorkspaceNewProperties::try_from( properties )?;
    match endpoint::workspace_new(&std::env::current_dir()?, repository_url, branches )
    {
      Ok( report ) =>
        {
          println!( "{report} ");

          Ok( () )
        }
      Err( ( report, e ) ) =>
        {
          eprintln!( "{report}" );
          Err( e.into() )
        }
    }
  }

  impl TryFrom< Props > for WorkspaceNewProperties
  {
    type Error = wtools::error::for_app::Error;

    fn try_from( value: Props ) -> std::result::Result< Self, Self::Error > 
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
  exposed use workspace_new;
}

