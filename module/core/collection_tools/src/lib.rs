#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/collection_tools/latest/collection_tools/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
extern crate alloc;

/// Module containing all collection macros
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
pub mod collection;

// #[ cfg( feature = "enabled" ) ]
// #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
// pub use collection::*;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{

  #[ cfg( feature = "use_alloc" ) ]
  pub use ::hashbrown;

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use collection::own::*;

}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use collection::orphan::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use collection::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use collection::prelude::*;

}

// pub use hmap as xxx;
// pub use own::HashMap as xxx;
// pub fn x()
// {
//   let x : HashMap< usize, usize > = hmap!{};
// }
