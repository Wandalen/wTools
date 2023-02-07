#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/_blank/latest/_blank/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! ___.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

//

wtools::mod_interface!
{
  /// Features of Application Programming Interface that 100% should be implemented
  #[ cfg( feature = "use_std" ) ]
  layer core;

  /// Library of utility to work with commands.
  #[ cfg( feature = "use_std" ) ]
  layer commands;

  /// Operate over files.
  #[ cfg( feature = "use_std" ) ]
  layer files;

  #[ cfg( feature = "use_std" ) ]
  prelude use ::std::env;
  prelude use ::wca::instruction;
  protected( crate ) use ::wtools::prelude::*;
}
