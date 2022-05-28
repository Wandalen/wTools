#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( type_name_of_val ) ]

//!
//! Collection of derives which extend STD.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Internal namespace.
mod internal
{
}

/// Own namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
  // use super::internal as i;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use ::derive_more::*;
  // #[ cfg( any( feature = "derive_display", feature = "derive_from_str" ) ) ]
  // pub use ::parse_display::
  // {
  //   *,
  //   #[ cfg( feature = "display" ) ]
  //   Display,
  //   #[ cfg( feature = "derive_from_str" ) ]
  //   FromStr,
  // };

  #[ cfg( feature = "derive_display" ) ]
  pub use ::parse_display::Display;

  #[ cfg( feature = "derive_from_str" ) ]
  pub use ::parse_display::FromStr;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
