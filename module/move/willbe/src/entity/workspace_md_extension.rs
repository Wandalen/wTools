/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  
  /// Md's extension for workspace
  pub trait WorkspaceMdExtension
  {
    /// Return discord url
    fn discord_url( &self ) -> Result< Option< String >, WorkspaceError >;

    /// Return the master branch
    fn master_branch( &self ) -> Result< Option< String >, WorkspaceError >;

    /// Return the repository url
    fn repository_url( &self ) -> Result< Option< String >, WorkspaceError >;

    /// Return the workspace_name
    fn workspace_name( &self ) -> Result< Option< String >, WorkspaceError >;
  }

  impl WorkspaceMdExtension for Workspace
  {
    fn discord_url( &self ) -> Result< Option< String >, WorkspaceError > 
    {
      Ok
      ( 
        self
        .metadata
        .as_ref()
        .ok_or_else( || WorkspaceError::MetadataError )?
        .workspace_metadata[ "discord_url" ]
        .as_str()
        .map( | url | url.to_string() ) 
      )
    }

    fn master_branch( &self ) -> Result< Option< String >, WorkspaceError > 
    {
      Ok
      ( 
        self
        .metadata
        .as_ref()
        .ok_or_else( || WorkspaceError::MetadataError )?
        .workspace_metadata
        .get( "master_branch" )
        .and_then( | b | b.as_str() )
        .map( | b | b.to_string() ) 
      )
    }

    fn repository_url( &self ) -> Result< Option< String >, WorkspaceError > 
    {
      Ok
      ( 
        self
        .metadata
        .as_ref()
        .ok_or_else( || WorkspaceError::MetadataError )?
        .workspace_metadata
        .get( "repo_url" )
        .and_then( | b | b.as_str() )
        .map( | b | b.to_string() ) 
      )
    }

    fn workspace_name( &self ) -> Result< Option< String >, WorkspaceError > 
    {
      Ok
      ( 
        self
        .metadata
        .as_ref()
        .ok_or_else( || WorkspaceError::MetadataError )?
        .workspace_metadata
        .get( "workspace_name" )
        .and_then( | b | b.as_str() )
        .map( | b | b.to_string() ) 
      )
    }
  }

}


crate::mod_interface!
{
  protected use WorkspaceMdExtension;
}
