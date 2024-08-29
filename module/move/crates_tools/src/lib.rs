#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/crates_tools/latest/crates_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
mod private
{
  use std::collections::HashMap;
  use std::fmt::Formatter;
  use std::io::Read;
  use std::path::{ Path, PathBuf };
  use std::time::Duration;
  use ureq::{ Agent, AgentBuilder };

  /// Represents a `.crate` archive, which is a collection of files and their contents.
  #[ derive( Default, Clone, PartialEq ) ]
  pub struct CrateArchive( HashMap< PathBuf, Vec< u8 > > );

  impl std::fmt::Debug for CrateArchive
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "CrateArchive" ).field( "files", &self.0.keys() ).finish()
    }
  }

  impl CrateArchive
  {
    /// Reads and decode a `.crate` archive from a given path.
    pub fn read< P >( path : P ) -> std::io::Result< Self >
    where
      P : AsRef< Path >,
    {
      let mut file = std::fs::File::open( path )?;
      let mut buf = vec![];
      file.read_to_end( &mut buf )?;

      Self::decode( buf )
    }

    #[ cfg( feature = "network" ) ]
    /// Downloads and decodes a `.crate` archive from a given url.
    pub fn download< Url >( url : Url ) -> Result< Self, ureq::Error >
    where
      Url : AsRef< str >,
    {
      let agent: Agent = AgentBuilder::new()
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
    #[ cfg( feature = "network" ) ]
    pub fn download_crates_io< N, V >( name : N, version : V ) -> Result< Self, ureq::Error >
    where
      N : std::fmt::Display,
      V : std::fmt::Display,
    {
      Self::download( format!( "https://static.crates.io/crates/{name}/{name}-{version}.crate" ) )
    }

    /// Decodes a bytes that represents a `.crate` file.
    pub fn decode< B >( bytes : B ) -> std::io::Result< Self >
    where
      B : AsRef<[ u8 ]>,
    {
      use std::io::prelude::*;
      use flate2::bufread::GzDecoder;
      use tar::Archive;

      let bytes = bytes.as_ref();
      if bytes.is_empty()
      {
        return Ok( Self::default() )
      }

      let gz = GzDecoder::new( bytes );
      let mut archive = Archive::new( gz );

      let mut output = HashMap::new();

      for file in archive.entries()?
      {
        let mut file = file?;

        let mut contents = vec![];
        file.read_to_end( &mut contents )?;

        output.insert( file.path()?.to_path_buf(), contents );
      }

      Ok( Self( output ) )
    }
  }

  impl CrateArchive
  {
    /// Returns a list of files from the `.crate` file.
    pub fn list( &self ) -> Vec< &Path >
    {
      self.0.keys().map( PathBuf::as_path ).collect()
    }

    /// Returns content of file by specified path from the `.crate` file in bytes representation.
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
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::CrateArchive;
}
