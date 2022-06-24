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

/// Basics.
pub mod basic;
/// Helpers.
pub mod helper;

/// Dependencies.
pub mod dependencies
{
  pub use ::paste;
  pub use ::trybuild;
  pub use ::anyhow;
  pub use ::rustversion;
  pub use ::meta_tools;
  pub use ::typing_tools;
  pub use ::num_traits;
  pub use ::diagnostics_tools;
}

pub use dependencies::*;
pub use ::meta_tools as meta;
pub use ::typing_tools as typing;

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::basic::exposed::*;
  #[ doc( inline ) ]
  pub use super::helper::exposed::*;

  #[ doc( inline ) ]
  pub use ::diagnostics_tools::exposed::*;
  #[ doc( inline ) ]
  pub use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };
  #[ doc( inline ) ]
  pub use ::typing_tools::{ implements };

  #[ doc( inline ) ]
  pub use ::inspect_type::*;
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::basic::prelude::*;
  #[ doc( inline ) ]
  pub use super::helper::prelude::*;

  #[ doc( inline ) ]
  pub use ::diagnostics_tools::prelude::*;
  #[ doc( inline ) ]
  pub use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };
  #[ doc( inline ) ]
  pub use ::typing_tools::{ implements };

}
