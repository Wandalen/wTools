#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/proc_macro_tools/latest/proc_macro_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

// #![ feature( type_name_of_val ) ]

//!
//! Tools for writing procedural macroses.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

pub mod container_kind;
pub mod helper;
pub mod name;
pub mod quantifier;
pub mod syntax;
pub mod generic_analyze;

///
/// Dependencies of the module.
///

pub mod dependency
{
  pub use ::syn;
  pub use ::quote;
  pub use ::proc_macro2;
  pub use ::winterval;
  pub use ::type_constructor;
}

#[ doc( inline ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Parented namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::
  {
    prelude::*,
    container_kind::exposed::*,
    generic_analyze::exposed::*,
    helper::exposed::*,
    name::exposed::*,
    // split_with_name::exposed::*,
    quantifier::exposed::*,
    syntax::exposed::*,
  };
  #[ doc( inline ) ]
  pub use super::quantifier::
  {
    Pair,
    Many,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  #[ doc( inline ) ]
  pub use ::winterval::prelude::*;
  #[ doc( inline ) ]
  pub use ::type_constructor::prelude::*;

  #[ doc( inline ) ]
  pub use ::syn;
  #[ doc( inline ) ]
  pub use ::proc_macro2;
  #[ doc( inline ) ]
  pub use ::quote;
  #[ doc( inline ) ]
  pub use ::quote::quote as qt;
  #[ doc( inline ) ]
  pub use ::syn::parse_quote as parse_qt;
  #[ doc( inline ) ]
  pub use ::proc_macro_tools::syn::spanned::Spanned;

  #[ doc( inline ) ]
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

  #[ doc( inline ) ]
  pub use super::
  {
    container_kind::prelude::*,
    generic_analyze::prelude::*,
    helper::prelude::*,
    name::prelude::*,
    // split_with_name::prelude::*,
    quantifier::prelude::*,
    syntax::prelude::*,
  };

}

// qqq : introduce features. make it smart. discuss list of features before implementing
