#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/pth/latest/pth/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]
#![ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]

#[ cfg( feature = "enabled" ) ]
use mod_interface::mod_interface;

#[ cfg( feature="no_std" ) ]
#[ macro_use ]
extern crate alloc;

mod private {}

#[ cfg( feature = "enabled" ) ]
mod_interface!
{

  /// Basic functionality.
  layer path;

  /// AsPath trait.
  layer as_path;
  /// TryIntoPath trait.
  layer try_into_path;
  /// TryIntoPath trait.
  layer try_into_cow_path;

  /// Transitive TryFrom and TryInto.
  layer transitive;

  #[ cfg( feature = "path_utf8" ) ]
  own use ::camino::{ Utf8Path, Utf8PathBuf };

  // #[ cfg( not( feature = "no_std" ) ) ]
  // own use ::std::path::{ PathBuf, Path, Component };

  #[ cfg( not( feature = "no_std" ) ) ]
  own use ::std::path::*;

  #[ cfg( not( feature = "no_std" ) ) ]
  own use ::std::borrow::Cow;

}
