#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/mod_interface_meta/latest/mod_interface_meta/" ) ]
#![ deny( dead_code ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// xxx : write good description and the main use-case

// xxx : does not work. make it working
// use super::test::{ compiletime, helper, smoke_test };

// // xxx : eliminate need to do such things, putting itself to proper category
// exposed use super::test::compiletime;
// exposed use super::test::helper;
// exposed use super::test::smoke_test;

// crate::mod_interface!
// {
//   // xxx : make it working
//   // exposed use super;
//   exposed use super::super::compiletime;
//   protected use
//   {
//     *
//   };
// }

// xxx : make use proper_path_tools::protected::path working

// xxx : put modular files into a namespace `file` maybe
// #[ cfg( feature = "enabled" ) ]
// #[ path = "." ]
// mod file
// {
//   use super::*;
//   pub mod tokens;
//   pub mod typ;
//   pub mod item_struct;
// }

// xxx : check
//
// - does not work
// exposed use
// {
//   ::former::Former,
//   ::former::Assign,
// };
//
// - work
//
// exposed use ::former::
// {
//   Former,
//   Assign,
// };

mod impls;
#[ allow( unused_imports ) ]
use impls::exposed::*;
mod record;
use record::exposed::*;
mod visibility;
use visibility::exposed::*;
mod use_tree;
use use_tree::exposed::*;

///
/// Protocol of modularity unifying interface of a module and introducing layers.
///

#[ cfg( feature = "enabled" ) ]
#[ proc_macro ]
pub fn mod_interface( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = impls::mod_interface( input );
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

/*

mod_interface!
{

  pub mod file1;
  pub mod file2;

  private mod micro_private;
  protected mod micro_protected;
  orphan mod micro_orphan;
  exposed mod micro_exposed;
  prelude mod micro_prelude;

  use prelude_file::*;

}

      private      <      protected      <      orphan      <      exposed      <      prelude
      itself               itself             its parent       its inter-module    its inter-module
      private              public               public             public              public

micro-module < meso-module < macro-module < inter-module

*/

