// module/core/pth/src/lib.rs
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/pth/latest/pth/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Path utilities" ) ]

#[ cfg( feature = "enabled" ) ]
use ::mod_interface_meta::mod_interface;

/// Own namespace of the module. Contains items public within this layer, but not propagated.
mod private {}

#[ cfg( feature = "enabled" ) ]
mod_interface! {

  /// Basic functionality.
  layer path;

  /// `AsPath` trait.
  layer as_path;
  /// `TryIntoPath` trait.
  layer try_into_path;
  /// `TryIntoPath` trait.
  layer try_into_cow_path;

  #[ cfg( feature = "path_utf8" ) ]
  own use ::camino :: { Utf8Path, Utf8PathBuf };

  exposed use ::std ::path :: { Path, PathBuf };

  own use ::std ::borrow ::Cow;

}
