
#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico")]
#![ doc( html_root_url = "https://docs.rs/type_constructor/latest/type_constructor/")]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Type constructors of fundamental data types.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// #![ without_std ]

// #[ cfg( feature = "no_std" ) ]
// extern crate core as std;
// #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
// extern crate alloc;

// #[ path = "./inc.rs" ]
// mod inc;
// pub mod type_constuctor;
// #[ doc( inline ) ]
// pub use inc::*;


#[ cfg( feature = "enabled" ) ]
pub mod type_constuctor;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
}

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  pub use super::type_constuctor::protected::*;
}

#[ doc( inline ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  pub use super::type_constuctor::orphan::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::type_constuctor::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use super::type_constuctor::prelude::*;
}
