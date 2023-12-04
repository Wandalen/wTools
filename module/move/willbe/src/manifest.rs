/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use crate::process::start_sync;
  use std::fs::File;
  use std::io::Read;
  use std::path::Path;
  use std::
  { 
    fs,
    env,
    process,
    path::PathBuf
  };
  use wtools::error;
  use wtools::error::for_app::
  { 
    anyhow,
    Context
   };

  ///
  /// Hold manifest data.
  ///

  #[ derive( Debug ) ]
  pub struct Manifest
  {
    /// Path to `Cargo.toml`
    pub manifest_path : PathBuf,
    /// Strict type of `Cargo.toml` manifest.
    pub manifest_data : Option< toml_edit::Document >,
  }

  impl Manifest
  {
    /// Create instance.
    pub fn new() -> Self
    {
      Manifest
      {
        manifest_path : PathBuf::default(),
        manifest_data : None,
      }
    }

    /// Join manifest path.
    // qqq : for Bohdan : bad, poor description
    // qqq : for Bohdan : bad, what is argument? introduce newtype
    // qqq : for Bohdan : introduce newtype AbsolutePath for all paths
    pub fn manifest_path_from_str( &mut self, path : impl Into< PathBuf > ) -> error::for_app::Result< PathBuf >
    {
      let mut path_buf : PathBuf = path.into();

      // qqq : for Bohdan : bad, should throw error on relative
      if path_buf.is_relative()
      {
        let mut current_dir = env::current_dir().context( "Try to take current dir for relative manifest" )?;
        current_dir.push( path_buf );
        path_buf = current_dir;
      }

      // qqq : for Bohdan : bad, use newtypes
      if !path_buf.ends_with( "Cargo.toml" )
      {
        path_buf.push( "Cargo.toml" );
      }
      self.manifest_path = path_buf.clone();

      Ok( path_buf )
    }

    /// Load manifest from path.
    pub fn load( &mut self ) -> error::for_app::Result< () >
    {
      let read = fs::read_to_string( &self.manifest_path ).context( "Read manifest" )?;
      let result = read.parse::< toml_edit::Document >().context( "Pars manifest" )?;
      self.manifest_data = Some( result );

      Ok( () )
    }

    // qqq : for Bohdan : don't abuse anyhow
    /// Store manifest.
    pub fn store( &self ) -> error::for_app::Result< () >
    {
      let data = self.manifest_data.as_ref().ok_or( anyhow!( "Manifest data wasn't loaded" ) )?.to_string();
      println!( "Saved manifest data to {:?}\n", &self.manifest_path );
      println!( "{}", &data );

      // qqq : for Bohdan : make proper errors handling
      fs::write( &self.manifest_path, &data ).unwrap_or_else
      (
        | err |
        {
          eprintln!( "{}", err );
          process::exit( -1 );
        }
      );

      Ok( () )
    }

    /// Check that current manifest is manifest for a package.
    // qqq : for Bohdan : poor description, what else could it be?
    pub fn package_is( &self ) -> bool
    {
      let data = self.manifest_data.as_ref().expect( "Manifest data wasn't loaded" );
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        return true;
      }
      false
    }

    /// Check that module is local.
    // qqq : for Bohdan : poor description, how?
    pub fn local_is( &self ) -> bool
    {
      let data = self.manifest_data.as_ref().unwrap();
      if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
      {
        let remote = data[ "package" ].get( "publish" ).is_none()
                     || data[ "package" ][ "publish" ].as_bool().unwrap();
        return !remote;
      }
      true
    }
  }

  /// Create and load manifest by specified path
  // qqq : for Bohdan : use newtype, add proper errors handing
  pub fn open( path : impl Into< PathBuf > ) -> error::for_app::Result< Manifest >
  {
    let mut manifest = Manifest::new();
    manifest.manifest_path_from_str( path )?;
    manifest.load()?;
    Ok( manifest )
  }

  /// Retrieves the repository URL of a package from its `Cargo.toml` file.
  pub fn repo_url( package_path: &Path ) -> error::for_app::Result< String >
  {
    let path = package_path.join( "Cargo.toml" );
    if path.exists() 
    {
      let mut contents = String::new();
      File::open( path )?.read_to_string( &mut contents )?;
      let doc = contents.parse::< toml_edit::Document >()?;

      let repo_url = doc
      .get( "package" )
      .and_then( | package | package.get( "repository" ) )
      .and_then( | i | i.as_str() );
      if let Some( repo_url ) = repo_url 
      {
        url::extract_repo_url( repo_url ).ok_or_else( || anyhow!( "Fail to extract repository url ") ) 
      }
      else 
      {
        let report = start_sync( "git ls-remote --get-url", package_path )?;
        url::extract_repo_url( &report.out.trim() ).ok_or_else( || anyhow!( "Fail to extract repository url from git remote.") )
      }
    }
    else
    {
      Err( anyhow!( "No Cargo.toml found" ) )
    }
  }

}

//

crate::mod_interface!
{
  orphan use Manifest;
  protected use open;
  protected use repo_url;
}
