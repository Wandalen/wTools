/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  use std::
  {
    // io::{ self, Read },
    // fs,
    path::{ Path, PathBuf },
  };
  use wtools::error::
  {
    Result,
    thiserror,
    for_lib::Error,
    // for_app::format_err,
  };
  use _path::AbsolutePath;
  // xxx

  /// `CrateDirError` enum represents errors when creating a `CrateDir` object.
  #[ derive( Debug, Error ) ]
  pub enum CrateDirError
  {
    /// Indicates a validation error with a descriptive message.
    #[ error( "Failed to create a `CrateDir` object due to `{0}`" ) ]
    Validation( String ),
  }

  // xxx : move out
  /// Path to crate directory
  #[ derive( Debug, Clone ) ]
  pub struct CrateDir( AbsolutePath );

  impl AsRef< Path > for CrateDir
  {
    fn as_ref( &self ) -> &Path
    {
      self.0.as_ref()
    }
  }

  // impl AsRef< AbsolutePath > for CrateDir
  // {
  //   fn as_ref( &self ) -> &AbsolutePath
  //   {
  //     &self.0
  //   }
  // }

  impl TryFrom< AbsolutePath > for CrateDir
  {
    type Error = CrateDirError;

    #[ inline( always ) ]
    fn try_from( crate_dir_path : AbsolutePath ) -> Result< Self, Self::Error >
    {
      if !crate_dir_path.as_ref().join( "Cargo.toml" ).exists()
      {
        return Err( CrateDirError::Validation( "The path is not a crate directory path".into() ) );
      }
      Ok( Self( crate_dir_path ) )
    }
  }

  impl TryFrom< PathBuf > for CrateDir
  {
    type Error = CrateDirError;

    #[ inline( always ) ]
    fn try_from( crate_dir_path : PathBuf ) -> Result< Self, Self::Error >
    {
      if !crate_dir_path.join( "Cargo.toml" ).exists()
      {
        return Err( CrateDirError::Validation( "The path is not a crate directory path".into() ) );
      }

      Ok( Self( AbsolutePath::try_from( crate_dir_path ).unwrap() ) )
    }
  }

  // xxx : remove
  impl CrateDir
  {
    // qqq : bad : for Petro : why clone?
    // /// Returns an absolute path.
    // pub fn absolute_path( &self ) -> AbsolutePath
    // {
    //   self.0.clone()
    // }

    /// Returns inner type what is an absolute path.
    #[ inline( always ) ]
    pub fn inner( self ) -> AbsolutePath
    {
      self.0
    }

    /// Returns path to manifest aka cargo file.
    #[ inline( always ) ]
    pub fn manifest_path( self ) -> AbsolutePath
    {
      self.inner().join( "Cargo.toml" )
    }

  }

}

//

crate::mod_interface!
{
  exposed use CrateDir;
  orphan use CrateDirError;
}
