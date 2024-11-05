// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/test_tools/latest/test_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  // // zzz : exclude later
  // #[ doc( inline ) ]
  // pub use ::paste;
  #[ doc( inline ) ]
  pub use ::trybuild;
  #[ doc( inline ) ]
  pub use ::rustversion;

  // #[ doc( inline ) ]
  // pub use ::error_tools;
//   #[ doc( inline ) ]
//   pub use ::meta_tools;
//   #[ doc( inline ) ]
//   pub use ::mem_tools;
//   #[ doc( inline ) ]
//   pub use ::typing_tools;
//   #[ doc( inline ) ]
//   pub use ::num_traits;
//   #[ doc( inline ) ]
//   pub use ::diagnostics_tools;
//
//   #[ doc( inline ) ]
//   pub use ::process_tools;

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::process_tools as process_tools;

}

mod private {}

//

// #[ cfg( feature = "enabled" ) ]
// // #[ cfg( not( feature = "no_std" ) ) ]
// ::meta_tools::mod_interface!
// {
//   // #![ debug ]
//
//   own use super::dependency::*;
//
//   layer test;
//
//   // xxx : comment out
//   use super::exposed::meta;
//   use super::exposed::mem;
//   use super::exposed::typing;
//   use super::exposed::dt;
//   use super::exposed::diagnostics;
//   use super::exposed::collection;
//   // use super::exposed::process;
//
//   // prelude use ::rustversion::{ nightly, stable };
//
//   // // xxx : eliminate need to do such things, putting itself to proper category
//   // exposed use super::test::compiletime;
//   // exposed use super::test::helper;
//   // exposed use super::test::smoke_test;
//
//   prelude use ::meta_tools as meta;
//   prelude use ::mem_tools as mem;
//   prelude use ::typing_tools as typing;
//   prelude use ::data_type as dt;
//   prelude use ::diagnostics_tools as diagnostics;
//   prelude use ::collection_tools as collection;
//   // prelude use ::process_tools as process;
//
//   use ::collection_tools; // xxx : do that for all dependencies
//
//   prelude use ::meta_tools::
//   {
//     impls,
//     index,
//     tests_impls,
//     tests_impls_optional,
//     tests_index,
//   };
//
//   prelude use ::typing_tools::{ implements };
//
// }

// xxx : use module namespaces
// #[ cfg( feature = "enabled" ) ]
// #[ cfg( not( feature = "no_std" ) ) ]
// pub use test::{ compiletime, helper, smoke_test };

#[ cfg( feature = "enabled" ) ]
pub mod test;

/// Error tools.
#[ cfg( feature = "standalone" ) ]
#[ path = "../../../core/error_tools/src/error/mod.rs" ]
pub mod error;
#[ cfg( feature = "standalone" ) ]
pub use error as error_tools;

/// Collection tools.
#[ cfg( feature = "standalone" ) ]
#[ path = "../../../core/collection_tools/src/collection/mod.rs" ]
pub mod collection;
#[ cfg( feature = "standalone" ) ]
pub use collection as collection_tools;

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
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use test::own::*;

  #[ doc( inline ) ]
  pub use
  {
    error_tools::orphan::*,
    collection_tools::orphan::*,
    // meta_tools::orphan::*,
    mem_tools::orphan::*,
    typing_tools::orphan::*,
    diagnostics_tools::orphan::*,
  };

}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use test::orphan::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use test::exposed::*;

  #[ doc( inline ) ]
  pub use
  {
    error_tools::exposed::*,
    collection_tools::exposed::*,
    // meta_tools::exposed::*,
    mem_tools::exposed::*,
    typing_tools::exposed::*,
    diagnostics_tools::exposed::*,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use test::prelude::*;

  #[ doc( inline ) ]
  pub use
  {
    error_tools::prelude::*,
    collection_tools::prelude::*,
    // meta_tools::prelude::*,
    mem_tools::prelude::*,
    typing_tools::prelude::*,
    diagnostics_tools::prelude::*,
  };

}
