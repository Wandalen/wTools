//! Typed text-with-optional-ANSI-color abstraction.
// `all(doc, feature = "enabled")` — not just `doc`: cfg(doc) is active during `cargo test --doc`,
// so without the feature guard the readme doc-tests run with no-default-features and fail (API absent).
#![ cfg_attr( all( doc, feature = "enabled" ), doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/color_tools/latest/color_tools/" ) ]
#![ allow( unused_imports ) ]

#[ cfg( feature = "enabled" ) ]
mod private
{
  pub use super::decorated_text::DecoratedText;
  pub use super::color::Color;
}

#[ cfg( feature = "enabled" ) ]
mod decorated_text;

#[ cfg( feature = "enabled" ) ]
mod color;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  pub use super::private::DecoratedText;
  pub use super::private::Color;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
}
