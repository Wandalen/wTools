#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/crates_tools/latest/crates_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Define a private namespace for all its items.
#[ cfg( feature = "enabled" ) ]
mod private
{
  use std::collections::HashMap;
  use core::fmt::Formatter;
  use std::io::Read;
  use std::path::{ Path, PathBuf };
  use core::time::Duration;
  use ureq::AgentBuilder;

  /// Represents a `.crate` archive, which is a collection of files and their contents.
  #[ derive( Default, Clone, PartialEq ) ]
  pub struct CrateArchive( HashMap< PathBuf, Vec< u8 > > );

  impl core::fmt::Debug for CrateArchive
  {
    #[ allow( clippy::implicit_return, clippy::min_ident_chars ) ]
    #[ inline]
    fn fmt( &self, f : &mut Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "CrateArchive" ).field( "files", &self.0.keys() ).finish()
    }
  }

  impl CrateArchive
  {
    /// Reads and decode a `.crate` archive from a given path.
    /// # Errors
    /// qqq: doc
    #[ allow( clippy::question_mark_used, clippy::implicit_return ) ]
    #[ inline ]
    pub fn read< P >( path : P ) -> std::io::Result< Self >
    where
      P : AsRef< Path >,
    {
      let mut file = std::fs::File::open( path )?;
      let mut buf = vec![];
      #[ allow( clippy::verbose_file_reads ) ]
      file.read_to_end( &mut buf )?;

      Self::decode( buf )
    }

    #[ cfg( feature = "network" ) ]
    #[ allow( clippy::question_mark_used, clippy::implicit_return, clippy::result_large_err ) ]
    /// Downloads and decodes a `.crate` archive from a given url.
    /// # Errors
    /// qqq: docs
    #[ inline ]
    pub fn download< Url >( url : Url ) -> Result< Self, ureq::Error >
    where
      Url : AsRef< str >,
    {
      let agent = AgentBuilder::new()
      .timeout_read( Duration::from_secs( 5 ) )
      .timeout_write( Duration::from_secs( 5 ) )
      .build();

      let resp = agent.get( url.as_ref() ).call()?;

      let mut buf = vec![];
      resp.into_reader().read_to_end( &mut buf )?;

      Ok( Self::decode( buf )? )
    }

    /// Downloads and decodes a `.crate` archive from `crates.io` repository by given name and version of the package.
    /// Requires the full version of the package, in the format of `"x.y.z"`
    ///
    /// Returns error if the package with specified name and version - not exists.
    /// # Errors
    /// qqq: doc
    #[ cfg( feature = "network" ) ]
    #[ allow( clippy::implicit_return, clippy::result_large_err ) ]
    #[ inline ]
    pub fn download_crates_io< N, V >( name : N, version : V ) -> Result< Self, ureq::Error >
    where
      N : core::fmt::Display,
      V : core::fmt::Display,
    {
      Self::download( format!( "https://static.crates.io/crates/{name}/{name}-{version}.crate" ) )
    }

    /// Decodes a bytes that represents a `.crate` file.
    /// # Errors
    /// qqq: doc
    #[ allow( clippy::question_mark_used, unknown_lints, clippy::implicit_return ) ]
    #[ inline ]
    pub fn decode< B >( bytes : B ) -> std::io::Result< Self >
    where
      B : AsRef<[ u8 ]>,
    {
      use std::io::prelude::*;
      use flate2::bufread::GzDecoder;
      use tar::Archive;

      let bytes_slice = bytes.as_ref();
      if bytes_slice.is_empty()
      {
        return Ok( Self::default() )
      }

      let gz = GzDecoder::new( bytes_slice );
      let mut archive = Archive::new( gz );

      let mut output = HashMap::new();

      for file in archive.entries()?
      {
        let mut archive_file = file?;

        let mut contents = vec![];
        archive_file.read_to_end( &mut contents )?;

        output.insert( archive_file.path()?.to_path_buf(), contents );
      }

      Ok( Self( output ) )
    }

    /// Returns a list of files from the `.crate` file.
    #[ allow( clippy::implicit_return ) ]
    #[ inline ]
    pub fn list( &self ) -> Vec< &Path >
    {
      self.0.keys().map( PathBuf::as_path ).collect()
    }

    /// Returns content of file by specified path from the `.crate` file in bytes representation.
    #[ allow( clippy::implicit_return ) ]
    #[ inline ]
    pub fn content_bytes< P >( &self, path : P ) -> Option< &[ u8 ] >
      where
        P : AsRef< Path >,
    {
      self.0.get( path.as_ref() ).map( Vec::as_ref )
    }
  }
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports, clippy::pub_use ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::orphan;
  #[ doc( inline ) ]
  #[ allow( unused_imports, clippy::pub_use ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::exposed;
  #[ doc( inline ) ]
  #[ allow( unused_imports, clippy::pub_use ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::prelude;
  #[ doc( inline ) ]
  #[ allow( unused_imports, clippy::pub_use ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::private;
  #[ doc( inline ) ]
  #[ allow( unused_imports, clippy::pub_use ) ]
  pub use private::CrateArchive;
}
