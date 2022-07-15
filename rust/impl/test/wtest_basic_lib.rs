#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wtest_basic/latest/wtest_basic/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Dependencies.
pub mod dependency
{
  pub use ::paste;
  pub use ::trybuild;
  pub use ::anyhow;
  pub use ::rustversion;
  pub use ::meta_tools;
  pub use ::mem_tools;
  pub use ::typing_tools;
  pub use ::num_traits;
  pub use ::diagnostics_tools;
}

pub use dependency::*;
use ::meta_tools::mod_interface;
// pub use ::meta_tools as meta;
// pub use ::mem_tools as mem;
// pub use ::typing_tools as typing;

mod_interface!
{
  /// Basics.
  layer basic;
  /// Helpers.
  layer helper;

  exposed use ::meta_tools as meta;
  exposed use ::mem_tools as mem;
  exposed use ::typing_tools as typing;

  exposed use ::diagnostics_tools::exposed::*;
  exposed use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };
  exposed use ::typing_tools::{ implements };
  exposed use ::inspect_type::exposed::*;

  prelude use ::diagnostics_tools::prelude::*;
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

// use ::diagnostics_tools::exposed::exposed;
// use ::diagnostics_tools::exposed::exposed;

// use exposed::exposed;
// qqq : for Dima : add negative test that wtest_basic::exposed::exposed does not exist
