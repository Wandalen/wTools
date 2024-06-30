#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former_types/latest/former_types/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Axiomatic things.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
mod axiomatic;
/// Definition of former.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
mod definition;
/// Forming process.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
mod forming;
/// Storage.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_former" ) ]
mod storage;

/// Interface for collections.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "types_former" ) ]
mod collection;

/// Component-based forming.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( feature = "types_component_assign" ) ) ]
mod component;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::collection_tools;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use super::
  {
    axiomatic::*,
    definition::*,
    forming::*,
    storage::*,
  };

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "types_former" ) ]
  pub use super::collection::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( any( feature = "types_component_assign" ) ) ]
  pub use super::component::*;
}
