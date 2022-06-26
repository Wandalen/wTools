// #![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/proc_macro_tools/latest/proc_macro_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( type_name_of_val ) ]

//!
//! Tools for writing procedural macroses.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Container kind.
pub mod container_kind;
/// Helpers.
pub mod helper;
/// Trait name.
pub mod name;
/// Split with name.
pub mod split_with_name;
/// Syntax.
pub mod syntax;
/// Analyze generic to provide more information.
pub mod generic_analyze;


///
/// Dependencies of the module.
///

pub mod dependencies
{
  pub use ::syn;
  pub use ::quote;
  pub use ::proc_macro2;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    container_kind::exposed::*,
    generic_analyze::exposed::*,
    helper::exposed::*,
    name::exposed::*,
    split_with_name::exposed::*,
    syntax::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  pub use super::
  {
    container_kind::prelude::*,
    generic_analyze::prelude::*,
    helper::prelude::*,
    name::prelude::*,
    split_with_name::prelude::*,
    syntax::prelude::*,
  };

  pub use ::syn;
  pub use ::proc_macro2;
  pub use quote;
  pub use quote::quote as qt;
  pub use syn::
  {
    parse::ParseStream,
    Token,
    braced,
    bracketed,
    custom_keyword,
    custom_punctuation,
    parenthesized,
    parse_macro_input,
    parse_quote,
    parse_quote_spanned,
  };

}
