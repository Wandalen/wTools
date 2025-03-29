/// Define a private namespace for all its items.
mod private
{
  #[ allow( unused_imports, clippy::wildcard_imports ) ]
  use crate::tool::*;

  use error::untyped::
  {
    format_err,
    // Result,
  };

  /// Extracts the repository URL from a full URL.
  #[ must_use ]
  pub fn repo_url_extract( full_url : &str ) -> Option< String >
  {
    let parts : Vec< &str > = full_url.split( '/' ).collect();

    if parts.len() >= 4 && parts[ 0 ] == "https:" && parts[ 1 ].is_empty() && parts[ 2 ] == "github.com"
    {
      let user = parts[ 3 ];
      let repo = parts[ 4 ];
      let repo_url = format!( "https://github.com/{user}/{repo}" );
      Some( repo_url )
    }
    else
    {
      None
    }
  }

  /// Extracts the username and repository name from a given URL.
  /// # Errors
  /// qqq: doc
  // qqq : use typed error
  pub fn git_info_extract( url : &str ) -> error::untyped::Result< String >
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
  own use repo_url_extract;
  own use git_info_extract;
}
