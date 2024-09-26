#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/mod_interface_meta/latest/mod_interface_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#![ warn( dead_code ) ]

// /// Derives.
// layer derive;
// own use super::derive;
// // xxx : change to remove need to write explicitly that

// xxx : change to remove need to write explicitly that
// crate::mod_interface!
// {
//   /// Derives.
//   layer derive;
//   own use super::derive; // xxx : change to remove need to write explicitly that
// }

// xxx : clean up, ad solve problems
// - example based on simpified version of test::layer_have_layer with single sublayer
// - example with attribute `#![ debug ]`

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
//   own use
//   {
//     *
//   };
// }

// xxx : make use proper_path_tools::own::path working

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

// xxx : inherit all entities, somehow
//
// pub mod ca;
//
// crate::mod_interface!
// {
//   // #![ debug ]
//
//   // xxx : syntax for that, please
//   use super::ca;
//   own use super::ca::own::*;
//
//   // /// Commands aggregator library.
//   // layer ca;
// }

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
  own mod micro_own;
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

