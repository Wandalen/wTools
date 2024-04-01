
#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico")]
#![ doc( html_root_url = "https://docs.rs/type_constructor/latest/type_constructor/")]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Type constructors of fundamental data types.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

pub use derive_tools::{ From_0, From_1, From_2, From_3, from };

/// Temporary workaround.
#[ macro_export ]
macro_rules! _if_from
{
  ( $( $code:tt )* )
  =>
  {
    $( $code )*
  };
}

// #![ without_std ]

// #[ cfg( feature = "no_std" ) ]
// extern crate core as std;
// #[ cfg( all( feature = "no_std", feature = "use_alloc" ) ) ]
// extern crate alloc;

// #[ path = "./inc.rs" ]
// mod inc;
// pub mod type_constuctor;
// #[ doc( inline ) ]
// #[ allow( unused_imports ) ]
// pub use inc::*;


#[ cfg( feature = "enabled" ) ]
pub mod type_constuctor;

/// Namespace with dependencies.

#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
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
  pub use super::type_constuctor::protected::*;
}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  pub use super::type_constuctor::orphan::*;
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
  pub use super::type_constuctor::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  pub use super::type_constuctor::prelude::*;
}
