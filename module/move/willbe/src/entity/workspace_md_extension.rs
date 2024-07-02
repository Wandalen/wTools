/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Md's extension for workspace
  pub trait WorkspaceMdExtension
  {
    /// Return discord url
    fn discord_url( &self ) -> Option< String >;

    /// Return the master branch
    fn master_branch( &self ) -> Option< String >;

    /// Return the repository url
    fn repository_url( &self ) -> Option< String >;

    /// Return the workspace_name
    fn workspace_name( &self ) -> Option< String >;
  }

  impl WorkspaceMdExtension for Workspace
  {
    fn discord_url( &self ) -> Option< String >
    {
      self
      .metadata
      .workspace_metadata[ "discord_url" ]
      .as_str()
      .map( | url | url.to_string() )
    }

    fn master_branch( &self ) -> Option< String >
    {
      self
      .metadata
      .workspace_metadata
      .get( "master_branch" )
      .and_then( | b | b.as_str() )
      .map( | b | b.to_string() )
    }

    fn repository_url( &self ) -> Option< String >
    {
      self
      .metadata
      .workspace_metadata
      .get( "repo_url" )
      .and_then( | b | b.as_str() )
      .map( | b | b.to_string() )
    }

    fn workspace_name( &self ) -> Option< String >
    {
      self
      .metadata
      .workspace_metadata
      .get( "workspace_name" )
      .and_then( | b | b.as_str() )
      .map( | b | b.to_string() )
    }
  }

}


crate::mod_interface!
{
  own use WorkspaceMdExtension;
}
