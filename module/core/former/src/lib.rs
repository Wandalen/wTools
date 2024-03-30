#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/former/latest/former/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// zzz : remove
#![ allow( missing_docs ) ]

// zzz : describe "Context-aware forming process"
// zzz : explain role of container in former

/// Axiomatic things.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "derive_former" ) ]
mod axiomatic;

/// Interface for containers.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "derive_former" ) ]
mod container;
/// Former of a vector.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "derive_former" ) ]
mod vector;
/// Former of a hash map.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "derive_former" ) ]
mod hash_map;
/// Former of a hash set.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ cfg( feature = "derive_former" ) ]
mod hash_set;

/// Component-based forming.
#[ cfg( feature = "enabled" ) ]
#[ cfg( any( feature = "derive_component_from", feature = "derive_component_assign" ) ) ]
mod component;

// mod axiomatic2;
// mod axiomatic3;
// mod vector2;
// mod vector3;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use former_meta;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta as derive;
}

/// Parented namespace of the module.
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

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use former_meta::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( feature = "derive_former" ) ]
  pub use super::axiomatic::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "derive_former" ) ]
  pub use super::container::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "derive_former" ) ]
  pub use super::vector::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "derive_former" ) ]
  pub use super::hash_map::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
  #[ cfg( feature = "derive_former" ) ]
  pub use super::hash_set::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "enabled" ) ]
  #[ cfg( any( feature = "derive_component_from", feature = "derive_component_assign" ) ) ]
  pub use super::component::*;
}

