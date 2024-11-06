
//!
//! Test asset helper.
//!

/// Define a private namespace for all its items.
// #[ cfg( not( feature = "no_std" ) ) ]
mod private
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


// //
// // #[ cfg( not( feature = "no_std" ) ) ]
// crate::mod_interface!
// {
//
//   // exposed use super;
//   exposed use super::super::asset;
//
//   // own use path_to_exe;
//
// }

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
  };

}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  pub use super::super::asset;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
  };

}
