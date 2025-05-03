#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/component_model_types/latest/component_model_types/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// /// Axiomatic things. // Removed
// #[ cfg( feature = "enabled" ) ] // Removed
// #[ cfg( feature = "types_component_model" ) ] // Removed
// mod axiomatic; // Removed
// /// Definition of component_model. // Removed
// #[ cfg( feature = "enabled" ) ] // Removed
// #[ cfg( feature = "types_component_model" ) ] // Removed
// mod definition; // Removed
// /// Forming process. // Removed
// #[ cfg( feature = "enabled" ) ] // Removed
// #[ cfg( feature = "types_component_model" ) ] // Removed
// mod forming; // Removed
/// Storage.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_component_model" ) ]
mod storage;

// /// Interface for collections. // Removed
// #[ cfg( feature = "enabled" ) ] // Removed
// #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ] // Removed
// #[ cfg( feature = "types_component_model" ) ] // Removed
// mod collection; // Removed

/// Component-based forming.
#[ cfg( feature = "enabled" ) ]
#[ cfg( feature = "types_component_assign" ) ]
mod component;

/// Namespace with dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
  pub use ::collection_tools;
}

// #[ doc( inline ) ] // Removed this block
// #[ cfg( feature = "enabled" ) ]
// pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod own
{
  #[ doc( inline ) ]
  pub use crate::orphan::*; // Changed to crate::orphan::*
}

/// Parented namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use crate::exposed::*; // Changed to crate::exposed::*

  // #[ doc( inline ) ] // Removed this block
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ] // Removed
  // #[ cfg( feature = "types_component_model" ) ] // Removed
  // pub use crate::collection::orphan::*; // Removed

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  pub use crate::prelude::*; // Changed to crate::prelude::*

  #[ doc( inline ) ]
  #[ cfg( feature = "types_component_model" ) ]
  pub use super::
  {
    // axiomatic::*, // Removed
    // definition::*, // Removed
    // forming::*, // Removed
    storage::*,
  };

  // #[ doc( inline ) ] // Removed this block
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ] // Removed
  // #[ cfg( feature = "types_component_model" ) ] // Removed
  // pub use crate::collection::exposed::*; // Removed

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ cfg( feature = "types_component_assign" ) ]
  pub use crate::component::*; // Changed to crate::component::*

  // #[ doc( inline ) ] // Removed this block
  // #[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ] // Removed
  // #[ cfg( feature = "types_component_model" ) ] // Removed
  // pub use crate::collection::prelude::*; // Removed

}
