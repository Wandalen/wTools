
//!
//! Test asset helper.
//!

/// Internal namespace.
// #[ cfg( not( feature = "no_std" ) ) ]
pub( crate ) mod private
{

//   use std::
//   {
//     env::consts::EXE_EXTENSION,
//     path::{ Path, PathBuf },
//     process::Command,
//   };
//
//   // xxx : qqq : ?
//   /// poorly described function
//   pub fn path_to_exe( temp_path : &Path, name : &Path,  ) -> PathBuf
//   {
//
//     _ = Command::new( "rustc" )
//     .current_dir( temp_path )
//     .arg( name )
//     .status()
//     .unwrap();
//
//     PathBuf::from( temp_path )
//     .join( name.file_name().unwrap() )
//     .with_extension( EXE_EXTENSION )
//   }

}


//
// #[ cfg( not( feature = "no_std" ) ) ]
crate::mod_interface!
{

  // exposed use super;
  exposed use super::super::asset;

  // own use path_to_exe;

}
