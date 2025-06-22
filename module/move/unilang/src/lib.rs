#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ allow( clippy::mod_module_files ) ]

///
/// A framework for creating multi-modal applications.
///

/// Internal namespace.
mod private
{
}

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{
  exposed mod data;
  exposed mod registry;
  exposed mod parsing;
  exposed mod semantic;
  exposed mod interpreter;
  exposed mod error;
  exposed mod help;
}
