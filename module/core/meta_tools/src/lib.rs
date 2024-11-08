#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/meta_tools/latest/meta_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  pub use ::mod_interface;
  #[ cfg( feature = "meta_for_each" ) ]
  pub use ::for_each;
  #[ cfg( feature = "meta_impls_index" ) ]
  pub use ::impls_index;
  #[ cfg( feature = "meta_idents_concat" ) ]
  pub use ::paste;

}

mod private {}

//

// // qqq : meta interface should be optional dependancy. please fix writing equivalent code manually
// #[ cfg( feature = "enabled" ) ]
// mod_interface::mod_interface!
// {
//   // #![ debug ]
//
//   layer meta;
//
// }

pub mod meta;

#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use meta::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use prelude::*;
  pub use meta::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use meta::prelude::*;
}
