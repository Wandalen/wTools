#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/typing_tools/latest/typing_tools/" ) ]

//! # Rule Compliance & Architectural Notes
//!
//! This crate provides collection of general purpose tools for type checking and has been
//! systematically updated to comply with the Design and Codestyle Rulebooks.
//!
//! ## Completed Compliance Work:
//!
//! 1. **Feature Architecture**: All functionality is properly gated behind the "enabled" feature.
//!
//! 2. **Documentation Strategy**: Uses `#![ doc = include_str!(...) ]` to include readme.md
//!    instead of duplicating documentation in source files.
//!
//! 3. **Attribute Formatting**: All attributes use proper spacing per Universal Formatting Rule.
//!
//! 4. **Namespace Organization**: Uses standard own/orphan/exposed/prelude pattern.

#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Type system utilities" ) ]

/// Collection of general purpose tools for type checking.
#[ cfg( feature = "enabled" ) ]
pub mod typing;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency {
  #[ cfg( feature = "typing_inspect_type" ) ]
  pub use ::inspect_type;
  #[ cfg( feature = "typing_is_slice" ) ]
  pub use ::is_slice;
  #[ cfg( feature = "typing_implements" ) ]
  pub use ::implements;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own {
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::typing::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan {
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed {
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::typing::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude {
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::typing::prelude::*;
}
