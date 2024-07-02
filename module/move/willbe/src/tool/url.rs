/// Internal namespace.
mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use error::untyped::
  {
    format_err,
    Result,
  };

  /// Extracts the repository URL from a full URL.
  pub fn repo_url_extract( full_url : &str ) -> Option< String >
  {
    let parts : Vec< &str > = full_url.split( '/' ).collect();

    if parts.len() >= 4 && parts[ 0 ] == "https:" && parts[ 1 ] == "" && parts[ 2 ] == "github.com"
    {
      let user = parts[ 3 ];
      let repo = parts[ 4 ];
      let repo_url = format!( "https://github.com/{}/{}", user, repo );
      Some( repo_url )
    }
    else
    {
      None
    }
  }

  /// Extracts the username and repository name from a given URL.
  pub fn git_info_extract( url : &String ) -> Result< String >
  {
    let parts : Vec< &str > = url.split( '/' ).collect();
    if parts.len() >= 2
    {
      Ok( format!( "{}/{}", parts[ parts.len() - 2 ], parts[ parts.len() - 1 ] ) )
    }
    else
    {
      Err( format_err!( "Fail to extract git username and repository name" ) )
    }
  }
}

crate::mod_interface!
{
  protected use repo_url_extract;
  protected use git_info_extract;
}
