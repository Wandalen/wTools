// #![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/reflect_tools/latest/reflect_tools/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Reflection utilities" ) ]
#![ allow( clippy::used_underscore_items ) ]
#![ allow( clippy::len_without_is_empty ) ]
#![ allow( clippy::iter_skip_next ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::used_underscore_binding ) ]
#![ allow( clippy::needless_return ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::elidable_lifetime_names ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::semicolon_if_nothing_returned ) ]
#![ allow( clippy::implicit_hasher ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::useless_conversion ) ]
#![ allow( clippy::needless_range_loop ) ]

#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "reflect_types" ) ]
pub mod reflect;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  // #[ cfg( any_derive ) ]
  pub use ::reflect_tools_meta;
}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ cfg( feature = "reflect_types" ) ]
  #[ doc( inline ) ]
  pub use super::reflect::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;

  #[ cfg( feature = "reflect_types" ) ]
  #[ doc( inline ) ]
  pub use super::reflect::exposed::*;

  // #[ cfg( any_derive ) ]
  #[ doc( inline ) ]
  pub use ::reflect_tools_meta::*;

  #[ doc( inline ) ]
  pub use ::derive_tools::{ From, InnerFrom };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ cfg( feature = "reflect_types" ) ]
  #[ doc( inline ) ]
  pub use super::reflect::prelude::*;

  #[ doc( inline ) ]
  pub use ::derive_tools::{ From, InnerFrom };

}
