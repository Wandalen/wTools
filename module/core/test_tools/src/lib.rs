// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/test_tools/latest/test_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  // zzz : exclude later
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::paste;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::trybuild;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::rustversion;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::error_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::meta_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::mem_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::typing_tools;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::num_traits;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::diagnostics_tools;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::process_tools_published;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::process_tools_published as process_tools;

}

mod private {}

//

#[ cfg( feature = "enabled" ) ]
// #[ cfg( not( feature = "no_std" ) ) ]
::meta_tools::mod_interface!
{
  // #![ debug ]

  own use super::dependency::*;

  layer test;

  // xxx : comment out
  use super::exposed::meta;
  use super::exposed::mem;
  use super::exposed::typing;
  use super::exposed::dt;
  use super::exposed::diagnostics;
  use super::exposed::collection;
  // use super::exposed::process;

  // prelude use ::rustversion::{ nightly, stable };

  // // xxx : eliminate need to do such things, putting itself to proper category
  // exposed use super::test::compiletime;
  // exposed use super::test::helper;
  // exposed use super::test::smoke_test;

  prelude use ::meta_tools as meta;
  prelude use ::mem_tools as mem;
  prelude use ::typing_tools as typing;
  prelude use ::data_type as dt;
  prelude use ::diagnostics_tools as diagnostics;
  prelude use ::collection_tools as collection;
  // prelude use ::process_tools as process;

  use ::collection_tools; // xxx : do that for all dependencies

  prelude use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };

  prelude use ::typing_tools::{ implements };

}

// xxx : use module namespaces
// #[ cfg( feature = "enabled" ) ]
// #[ cfg( not( feature = "no_std" ) ) ]
// pub use test::{ compiletime, helper, smoke_test };
