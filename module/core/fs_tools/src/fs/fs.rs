/// Internal namespace.
pub( crate ) mod private
{

//   #[ derive( Debug ) ]
//   pub struct TempDir
//   {
//     pub base_path : std::path::PathBuf,
//     pub prefix_path : std::path::PathBuf,
//     pub postfix_path : std::path::PathBuf,
//   }
//
//   impl Drop for TempDir
//   {
//
//     fn drop( &mut self )
//     {
//       self.clean();
//     }
//
//   }
//
//   impl TempDir
//   {
//     pub fn new() -> Self
//     {
//       Self
//       {
//         base_path : "".into(),
//         prefix_path : "".into(),
//         postfix_path : "".into(),
//       }
//     }
//
//     pub fn clean( &self ) -> Result< (), &'static str >
//     {
//       let result = std::fs::remove_dir_all( &self.test_path );
//       result.or_else( | err | format!( "Cannot remove temporary directory {}.", &self.test_path.display() ) );
//       Ok( () )
//     }
//
//     pub fn path_dir_for( &self, file_path : AsRef< &str > ) -> std::path::PathBuf
//     {
//       let result = std::path::PathBuf::new();
//       result::push( self.base_path );
//       result::push( format!( "{}", self.prefix_path, file_path.as_str(), self.postfix_path );
//       result
//     }
//
//   }

}

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use super::prelude::*;
  // use super::private::TempDir;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
