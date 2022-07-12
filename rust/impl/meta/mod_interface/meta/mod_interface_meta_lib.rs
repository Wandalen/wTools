// #![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/mod_interface_meta/latest/mod_interface_meta/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Protocol of modularity unifying interface of a module and introducing layers.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

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

// /// Protected namespace of the module.
// pub mod protected
// {
//   pub use super::orphan::*;
//   pub use super::
//   {
//     impls::orphan::*,
//     record::orphan::*,
//     visibility::orphan::*,
//   };
// }
//
// pub use protected::*;
//
// /// Parented namespace of the module.
// pub mod orphan
// {
//   pub use super::exposed::*;
// }
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   pub use super::prelude::*;
//   pub use super::
//   {
//     impls::exposed::*,
//     record::exposed::*,
//     visibility::exposed::*,
//   };
// }
//
// /// Prelude to use essentials: `use my_module::prelude::*`.
// pub mod prelude
// {
//   pub use super::
//   {
//     impls::prelude::*,
//     record::prelude::*,
//     visibility::prelude::*,
//   };
// }

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

slim module - ?
complete module - ?

*/

