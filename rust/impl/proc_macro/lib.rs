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

///
/// Dependencies of the module.
///

pub mod dependencies
{
  pub use ::syn;
  pub use ::quote;
  pub use ::proc_macro2;
}

/// Own namespace of the module.
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

  pub use super::container_kind::exposed::*;
  pub use super::helper::exposed::*;
  pub use super::name::exposed::*;
  pub use super::split_with_name::exposed::*;
  pub use super::syntax::exposed::*;

}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::container_kind::prelude::*;
  pub use super::helper::prelude::*;
  pub use super::name::prelude::*;
  pub use super::split_with_name::prelude::*;
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
    // parse_quote_spanned,
  };

}
