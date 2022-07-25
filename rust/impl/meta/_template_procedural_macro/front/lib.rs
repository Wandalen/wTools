#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/_template_procedural_macro/latest/_template_procedural_macro/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

//!
//! Template.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Namespace with dependencies.
pub mod dependency
{
  pub use procedural_macro_runtime;
  pub use procedural_macro_meta;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::exposed::*;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use procedural_macro_runtime as runtime;
  pub use procedural_macro_meta as meta;
  pub use meta::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
