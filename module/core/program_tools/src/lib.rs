#![ allow( unused_imports, dead_code, missing_docs ) ] // xxx: rid of

#[ cfg( feature = "enabled" ) ]
pub mod program
{
  use mod_interface::mod_interface;
  use error_tools::error::{ BasicError, err };

  mod private
  {
  mod_interface!
  {

   /// Compile and run a Rust program.
   layer program;

  }
 }
}
