#![ cfg_attr( not( feature = "use_std"), no_std ) ]
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

// /// General. To be moved out.
// pub mod a_general;

/// Container kind.
#[ cfg( feature = "use_std" ) ]
pub mod container_kind;
/// Helpers.
#[ cfg( feature = "use_std" ) ]
pub mod helper;
/// Trait name.
#[ cfg( feature = "use_std" ) ]
pub mod name;
/// Split with name.
#[ cfg( feature = "use_std" ) ]
pub mod split_with_name;
/// Syntax.
#[ cfg( feature = "use_std" ) ]
pub mod syntax;

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
  pub use super::prelude::*;

  // pub use super::a_general::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::container_kind::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::helper::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::name::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::split_with_name::exposed::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::syntax::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  // pub use super::a_general::prelude::wrap;
  // pub use super::a_general::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::container_kind::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::helper::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::name::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::split_with_name::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::syntax::prelude::*;

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
