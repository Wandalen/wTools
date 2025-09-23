#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/component_model_types/latest/component_model_types/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Component model type definitions" ) ]

/// Component-based forming.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_component_assign" ) ]
mod component;

/// Popular type support for common Rust types.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_component_assign" ) ]
pub mod popular_types;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::collection_tools;
}

#[ doc( inline ) ]
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod own
{
  #[ allow( unused_imports ) ]
  use crate::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::orphan::*; // Changed to crate::orphan::*
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ allow( unused_imports ) ]
  use crate::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate ::exposed :: *; // Changed to crate ::exposed :: *
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed 
{
  #[ allow( unused_imports ) ]
  use crate::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate ::prelude :: *; // Changed to crate ::prelude :: *
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude 
{
  #[ allow( unused_imports ) ]
  use crate::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types_component_assign" ) ]
  pub use crate ::component :: *; // Changed to crate ::component :: *
  #[ doc( inline ) ]
  #[ cfg( feature = "types_component_assign" ) ]
  pub use crate ::popular_types :: *;
}
